//! Root Server is the first user thread in sel4.
//! It has the ownership of all kernel resources.

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::mem::size_of;
use core::ops::Range;
use core::ptr::NonNull;

use aster_frame::boot::initramfs;
use aster_frame::cpu::UserContext;
use aster_frame::sync::{Mutex, SpinLock};
use aster_frame::task::{Task, TaskOptions};
use aster_frame::user::UserSpace;
use aster_frame::vm::{HasPaddr, PAGE_SIZE, PageFlags, Vaddr, VmAllocOptions, VmFrame, VmFrameVec, VmIo, VmMapOptions, VmSpace};
use spin::Once;
use static_assertions::const_assert;

use sel4::sys::seL4_PageTableBits;

use crate::{bit, is_aligned, round_down, round_up};
use crate::boot_info::{BOOT_INFO, BootInfo, BootInfoBuilder, BootState};
use crate::common::region::{Kaddr, paddr_to_kaddr, Region};
use crate::cspace::CNode;
use crate::cspace::raw::RawCap;
use crate::root_server::elf::{create_user_image, ElfLoadInfo};
use crate::root_server::elf::elf_file::Elf;
use crate::sel4::cnode::ROOT_CNODE_SIZE_BITS;
use crate::sel4::config::consts::USER_TOP;
use crate::sel4::sys::{seL4_ASIDPoolBits, seL4_BootInfoFrameBits, seL4_BootInfoHeader, seL4_PageBits, seL4_SlotBits, seL4_TCBBits, seL4_VSpaceBits, seL4_WordBits};
use crate::sel4::sys::seL4_RootCNodeCapSlots::{seL4_CapInitThreadCNode, seL4_CapIOPortControl};
use crate::thread::{task, TcbObject, Thread};
use crate::vspace::MemType;

mod elf;

static ROOT_SERVER_MEM: Once<Mutex<RootServerControlBlock>> = Once::new();

#[repr(C)]
/// root_server_mem_t
pub struct RootServerControlBlock {
    pub cnode: Arc<Mutex<CNode>>,
    pub vspace: Arc<VmSpace>,
    pub asid_pool: Kaddr,
    pub ipc_buf: Kaddr,
    pub boot_info: Kaddr,
    pub extra_bi: Kaddr,
    pub tcb: Kaddr,
    pub paging: Region,
}

#[derive(Debug)]
pub struct UserImage {
    // elf segments and its virtual address
    pub descs: Vec<SegmentDesc>,
    pub ui_bounds: Range<Vaddr>,
    pub elf_load_info: ElfLoadInfo,
}

impl Display for UserImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("\nentry: 0x{:x}\n", self.elf_load_info.entry_point()))?;
        f.write_fmt(format_args!("bounds: [0x{:x}, 0x{:x})\n", self.ui_bounds.start, self.ui_bounds.end))?;
        for desc in self.descs.iter() {
            f.write_fmt(format_args!("{}\n", desc))?
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct SegmentDesc {
    pub segment: VmFrameVec,
    pub start: Vaddr,
    pub pt_flags: PageFlags,
}

impl Display for SegmentDesc {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("[0x{:x}, 0x{:x}) {:?}", self.start, self.start + self.segment.nbytes(), self.pt_flags))
    }
}


/// launch the root server thread
pub struct SysLauncher {
    boot_state: BootState,
    elf_binary: &'static [u8],
    elf: Elf,
    user_image: Option<UserImage>,
    start_kaddr: Kaddr,
    end_kaddr: Kaddr,
    rscb: Option<RootServerControlBlock>,
}

impl SysLauncher {
    pub fn new() -> Self {
        let elf_binary = initramfs();
        let elf = Elf::parse_elf(elf_binary).unwrap();
        Self {
            boot_state: Default::default(),
            elf_binary,
            elf,
            user_image: None,
            start_kaddr: 0,
            end_kaddr: 0,
            rscb: None,
        }
    }
    pub fn launch(mut self) {
        let bi_vptr = self.create_root_thread();
        let Self {
            boot_state,
            user_image,
            rscb,
            ..
        }
            = self;
        let ui = user_image.unwrap();
        let rscb = rscb.unwrap();
        let vm_space = rscb.vspace.clone();
        for desc in ui.descs {
            let mut map = VmMapOptions::new();
            map.addr(Some(desc.start))
                .flags(desc.pt_flags);
            vm_space.map(desc.segment, &map).unwrap();
        }
        // use addr explicitly
        let frame = VmAllocOptions::new(1).alloc_single().expect("no memory");
        let bi = BootInfo(boot_state.bi_builder.build());
        frame.write_val::<BootInfo>(0, &bi).expect("write failed");
        let mut map = VmMapOptions::new();
        // todo change this
        map.addr(Some(bi_vptr)).flags(PageFlags::R);
        let vm_vec = VmFrameVec::from_one_frame(frame);
        let addr = vm_space.map(vm_vec.clone(), &map).unwrap();
        BOOT_INFO.call_once(|| MemType::new(vm_vec));
        let mut cpu_ctx = UserContext::default();
        cpu_ctx.set_rip(ui.elf_load_info.entry_point() as _);
        cpu_ctx.set_rdi(addr as _);
        // cpu_ctx.set_rdi(vptr);
        let user_space = Arc::new(UserSpace::new(vm_space.clone(), cpu_ctx));
        // let thread_name = Some(ThreadName::new_from_executable_path(executable_path)?);
        let thread = Arc::new_cyclic(|thread_ref| unsafe {
            let task = task::create_new_user_task(user_space, thread_ref.clone());
            let mut m = Box::new([0u8; bit!(seL4_TCBBits)]);
            let tcb_object = TcbObject::new(NonNull::new(m.as_mut_ptr()).unwrap());
            Thread::new(task, tcb_object, vm_space.clone())
        });
        thread.run();
    }
    fn maybe_alloc_extra_bi(&mut self, cmp_size_bits: usize, extra_bi_size_bits: usize) {
        // if extra_bi_size_bits >= cmp_size_bits && self.rscb.extra_bi == 0 {
        //     self.rscb.extra_bi = self.alloc_root_server_obj(extra_bi_size_bits, 1);
        // }
    }
    fn alloc_root_server_obj(&mut self, size_bits: usize, n: usize) -> usize {
        let allocated = self.start_kaddr;
        assert_eq!(allocated % bit!(size_bits), 0);
        self.start_kaddr += n * bit!(size_bits);
        assert!(self.start_kaddr <= self.end_kaddr);
        allocated
    }
    // alloc enough phy memory
    // basic user image +  ipc buffer + boot info
    fn create_root_thread(&mut self) -> Vaddr {
        info!("{}",self.elf_binary.len());
        let bounds = self.elf.memory_bounds();
        assert!(is_aligned!(bounds.start as usize));
        let ui_bounds = round_down!(bounds.start as usize)..round_up!(bounds.end as usize);
        assert!(ui_bounds.end + 2 * bit!(seL4_PageBits) <= USER_TOP);
        assert!(ui_bounds.contains(&self.elf.entry_point()));
        let extra_bi_size = size_of::<seL4_BootInfoHeader>();
        let extra_bi_offset: Kaddr = 0;

        let ipc_buf_vptr = ui_bounds.end;
        let bi_frame_vptr = ipc_buf_vptr + bit!(seL4_PageBits);
        let extra_bi_frame_vptr = bi_frame_vptr + bit!(seL4_BootInfoFrameBits as usize);
        // vbe
        // acpi_rsdp
        // fb_info
        // mb_mmap ignore
        // tsc

        // the largest object the PD, the root cnode, or the extra boot info
        let extra_bi_size_bits = calculate_extra_bi_size_bits(extra_bi_size);
        let size = calculate_root_server_size(extra_bi_size_bits);
        let max = root_server_max_size_bits(extra_bi_size_bits);

        let ui = create_user_image(self.elf_binary, ui_bounds, &self.elf).unwrap();
        info!("{}",ui);

        let cnode_size_bits = *ROOT_CNODE_SIZE_BITS + seL4_SlotBits as usize;

        assert!(cnode_size_bits > seL4_VSpaceBits as usize);
        // create root cnode

        let mut cnode = unsafe {
            let frames = Arc::new(VmAllocOptions::new(bit!(cnode_size_bits) / PAGE_SIZE).is_contiguous(true).alloc().unwrap());
            Arc::new(Mutex::new(CNode::new(frames)))
        };
        let ptr = Arc::as_ptr(&cnode) as usize;
        error!("0x{:x}",ptr);
        let root_cnode_cap = RawCap::new_cnode_cap(
            Arc::as_ptr(&cnode) as usize,
            *ROOT_CNODE_SIZE_BITS,
            seL4_WordBits as usize - *ROOT_CNODE_SIZE_BITS,
            0,
        );
        // cnode.lock().write_slot(seL4_CapInitThreadCNode as usize, root_cnode_cap);
        // self.maybe_alloc_extra_bi(seL4_VSpaceBits as usize, extra_bi_size_bits);
        // Caution: use vm space's addr not page table's root frame address
        let vspace = Arc::new(VmSpace::new());

        self.rscb = Some(RootServerControlBlock {
            cnode,
            vspace,
            asid_pool: 0,
            ipc_buf: 0,
            boot_info: 0,
            extra_bi: 0,
            tcb: 0,
            paging: Default::default(),
        });


        // at this point we are up to creating 4k objects
        // which is the min size of extra_bi so this is the last chance to allocate it

        // self.maybe_alloc_extra_bi(seL4_PageBits as usize, extra_bi_size_bits);
        const_assert!(seL4_ASIDPoolBits == seL4_PageBits);
        // self.rscb.asid_pool = self.alloc_root_server_obj(seL4_ASIDPoolBits as usize, 1);
        // self.rscb.ipc_buf = self.alloc_root_server_obj(seL4_PageBits as usize, 1);

        // The boot info size must be at least one page. Due to the hard-coded order
        // of allocations used in the current implementation here, it can't be any
        // bigger.

        const_assert!(seL4_BootInfoFrameBits == seL4_PageBits);
        // self.rscb.boot_info = self.alloc_root_server_obj(seL4_BootInfoFrameBits as usize, 1);

//         TCBs on aarch32 can be larger than page tables in certain configs

//         if seL4_TCBBits >= seL4_PageTableBits {
        // self.rscb.tcb = self.alloc_root_server_obj(seL4_TCBBits as usize, 1);
        // }

        // paging structures are 4k on every arch except aarch32 (1k)
        // let n = arch_get_n_paging(it_v_reg);
        // just place a page table in this
        // page is not continuous

        // self.rscb.paging.start = self.alloc_root_server_obj(seL4_PageTableBits as usize, 0);
        // self.rscb.paging.end = self.rscb.paging.start;

        /* for most archs, TCBs are smaller than page tables */
        // if seL4_TCBBits < seL4_PageTableBits {
        //     self.rscb.tcb = self.alloc_root_server_obj(seL4_TCBBits as usize, 1);
        // }

        // if CONFIG_KERNEL_MCS {
        //     // rootserver.sc = alloc_rootserver_obj(seL4_MinSchedContextBits, 1);
        // }
        /* we should have allocated all our memory */
        // assert_eq!(self.start_kaddr, self.end_kaddr);

        // self.rscb.cnode.write_slot(seL4_CapIOPortControl,)

        // write_slot(
        //     ptr.add(seL4_CapIOPortControl),
        //     cap_t::new_io_port_control_cap(),
        // );

        /* create the capability for managing thread domains IGNORED*/
        // create_domain_cap(&root_cnode_cap);
        /* initialise the IRQ states and provide the IRQ control capability */
        // init_irqs(&root_cnode_cap);

        // tsc

        // populate_bi_frame(0, 1, ipcbuf_vptr, extra_bi_size);

        /* Construct an initial address space with enough virtual addresses
         * to cover the user image + ipc buffer and bootinfo frames */
        // let it_vspace_cap = create_it_address_space(&root_cnode_cap, vm.root_vmar().vm_space());
        // if (cap_get_capType(it_vspace_cap) == cap_null_cap) {
        //     return false;
        // }

        /* Create and map bootinfo frame capability */
        // create_bi_frame_cap(&root_cnode_cap, &it_vspace_cap, bi_frame_vptr);

        /* create and map extra bootinfo region */
        //  let extra_bi_ret =
        //     create_frames_of_region(
        //         root_cnode_cap,
        //         it_vspace_cap,
        //         extra_bi_region,
        //         true,
        //         pptr_to_paddr((void *)(extra_bi_region.start - extra_bi_frame_vptr))
        //     );
        // if (!extra_bi_ret.success) {
        //     return false;
        // }
        // ndks_boot.bi_frame->extraBIPages = extra_bi_ret.region;

        /* create the initial thread's IPC buffer */
        // let ipcbuf_cap = create_ipcbuf_frame_cap(&root_cnode_cap, &it_vspace_cap, ipcbuf_vptr);
        // if ipcbuf_cap.get_cap_type() == CapTag::CapNullCap {
        //     panic!("ipcbuf_cap is null")
        // }

        /* create all userland image frames */
        // create_frames_ret =
        //     create_frames_of_region(
        //         root_cnode_cap,
        //         it_vspace_cap,
        //         ui_reg,
        //         true,
        //         ui_info.pv_offset
        //     );
        // if (!create_frames_ret.success) {
        //     return false;
        // }
        // ndks_boot.bi_frame->userImageFrames = create_frames_ret.region;

        /* create the initial thread's ASID pool */
        // it_ap_cap = create_it_asid_pool(root_cnode_cap);
        // if (cap_get_capType(it_ap_cap) == cap_null_cap) {
        //     return false;
        // }
        // write_it_asid_pool(it_ap_cap, it_vspace_cap);

        // #ifdef CONFIG_KERNEL_MCS
        //     NODE_STATE(ksCurTime) = getCurrentTime();
        // #endif

        /* create the idle thread */
        // create_idle_thread();

        /* create the initial thread */
        // tcb_t *initial = create_initial_thread(root_cnode_cap,
        //                                        it_vspace_cap,
        //                                        ui_info.v_entry,
        //                                        bi_frame_vptr,
        //                                        ipcbuf_vptr,
        //                                        ipcbuf_cap);
        // if (initial == NULL) {
        //     return false;
        // }
        // init_core_state(initial);

        self.boot_state.bi_builder.num_io_pt_levels(usize::MAX);

        // create all of the untypeds. Both devices and kernel window memory
        // create_untypeds(&root_cnode_cap);

        // finalise the bootinfo frame
        // bi_finalise();
        bi_frame_vptr
    }
}

fn calculate_extra_bi_size_bits(extra_size: usize) -> usize {
    if extra_size == 0 {
        return 0;
    }

    let clzl_ret = clzl(round_up!(extra_size));
    let mut msb = seL4_WordBits as usize - 1 - clzl_ret;
    if extra_size > bit!(msb) {
        msb += 1;
    }
    return msb;
}

fn clzl(size: usize) -> usize {
    size.leading_zeros() as usize
}

fn ctzl(size: usize) -> usize {
    size.trailing_zeros() as usize
}

fn calculate_root_server_size(extra_bi_size_bits: usize) -> usize {
    let mut size = bit!(*ROOT_CNODE_SIZE_BITS + seL4_SlotBits as usize); // cnodes ??
    size += bit!(seL4_TCBBits as usize); // root thread tcb
    size += bit!(seL4_PageBits as usize); // ipc buf
    size += bit!(seL4_BootInfoFrameBits as usize); // boot info
    size += bit!(seL4_ASIDPoolBits as usize);
    size += if extra_bi_size_bits > 0 {
        bit!(extra_bi_size_bits)
    } else {
        0
    };
    // no need to allocate root vspace and paging frames
    // size += bit!(seL4_VSpaceBits as usize); // root vspace
    size
}


fn root_server_max_size_bits(extra_bi_size_bits: usize) -> usize {
    let cnode_size_bits = *ROOT_CNODE_SIZE_BITS + seL4_SlotBits as usize;
    cnode_size_bits.max(seL4_VSpaceBits as usize).max(extra_bi_size_bits)
}

mod test {
    use ktest::ktest;

    use crate::root_server::RootServerControlBlock;

    #[ktest]
    fn size_test() {
        assert_eq!(core::mem::size_of::<RootServerControlBlock>(), 72);
    }
}