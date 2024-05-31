use crate::sel4::vspace::pptr_t;
use pod::Pod;

use super::{
    common::{p_region_t, region_t},
    cspace::{cap_t, cte::cte_t},
    exception::exception_t,
    seL4_MsgMaxExtraCaps, seL4_MsgMaxLength, CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS,
    MAX_NUM_FREEMEM_REG, MAX_NUM_RESV_REG,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seL4_BootInfoHeader {
    pub id: usize,
    pub len: usize,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct acpi_rsdp_t {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct seL4_X86_BootInfo_fb_t {
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub type_: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct seL4_X86_BootInfo_mmap_t {
    pub header: seL4_BootInfoHeader,
    pub mmap_length: u32,
    pub mmap: [seL4_X86_mb_mmap_t; 50],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct seL4_X86_mb_mmap_t {
    pub size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub type_: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ui_info_t {}

pub type seL4_SlotPos = usize;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Pod)]
pub struct seL4_SlotRegion {
    pub start: seL4_SlotPos,
    pub end: seL4_SlotPos,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod)]
pub struct seL4_UntypedDesc {
    pub paddr: usize,
    pub sizeBits: u8,
    pub isDevice: u8,
    pub padding: [u8; 6],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod)]
pub struct seL4_BootInfo {
    pub extraLen: usize,
    pub nodeID: usize,
    pub numNodes: usize,
    pub numIOPTLevels: usize,
    pub ipcBuffer: usize,
    pub empty: seL4_SlotRegion,
    pub sharedFrames: seL4_SlotRegion,
    pub userImageFrames: seL4_SlotRegion,
    pub userImagePaging: seL4_SlotRegion,
    pub ioSpaceCaps: seL4_SlotRegion,
    pub extraBIPages: seL4_SlotRegion,
    pub initThreadCNodeSizeBits: usize,
    pub initThreadDomain: usize,
    pub untyped: seL4_SlotRegion,
    pub untypedList: [seL4_UntypedDesc; CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndks_boot_t {
    pub reserved: [p_region_t; MAX_NUM_RESV_REG],
    pub resv_count: usize,
    pub freemem: [region_t; MAX_NUM_FREEMEM_REG],
    pub bi_frame: *mut seL4_BootInfo,
    pub slot_pos_cur: seL4_SlotPos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rootserver_mem_t {
    pub cnode: pptr_t,
    pub vspace: pptr_t,
    pub asid_pool: pptr_t,
    pub ipc_buf: pptr_t,
    pub boot_info: pptr_t,
    pub extra_bi: pptr_t,
    pub tcb: pptr_t,
    pub paging: region_t,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct create_frames_of_region_ret_t {
    pub region: seL4_SlotRegion,
    pub success: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct lookupCap_ret_t {
    pub status: exception_t,
    pub cap: cap_t,
}

impl Default for lookupCap_ret_t {
    fn default() -> Self {
        lookupCap_ret_t {
            status: exception_t::EXCEPTION_NONE,
            cap: cap_t::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct lookupCapAndSlot_ret_t {
    pub status: exception_t,
    pub cap: cap_t,
    pub slot: *mut cte_t,
}

impl Default for lookupCapAndSlot_ret_t {
    fn default() -> Self {
        lookupCapAndSlot_ret_t {
            status: exception_t::EXCEPTION_NONE,
            cap: cap_t::default(),
            slot: 0 as *mut cte_t,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct syscall_error_t {
    pub invalidArgumentNumber: usize,
    pub invalidCapNumber: usize,
    pub rangeErrorMin: usize,
    pub rangeErrorMax: usize,
    pub memoryLeft: usize,
    pub failedLookupWasSource: usize,
    pub _type: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seL4_IPCBuffer {
    pub tag: usize,
    pub msg: [usize; seL4_MsgMaxLength],
    pub userData: usize,
    pub caps_or_badges: [usize; seL4_MsgMaxExtraCaps],
    pub receiveCNode: usize,
    pub receiveIndex: usize,
    pub receiveDepth: usize,
    pub uintrFlag: usize,
    pub async_cid: usize,
}

impl seL4_IPCBuffer {
    pub fn get_extra_cptr(&self, i: usize) -> usize {
        self.caps_or_badges[i]
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct tss_t {
    pub workds: [usize; 13],
}

pub mod bootstrap {
    use core::{
        intrinsics::{pref_align_of, unaligned_volatile_load},
        mem,
    };

    use alloc::{boxed::Box, sync::Arc};
    use anti_frame::{
        boot::initramfs,
        vm::{kspace::paddr_to_vaddr, page_table::PageTable, HasPaddr, VmSpace},
    };
    use anti_rights::Rights;
    use static_assertions::const_assert;

    use crate::{
        process::{process_vm::ProcessVm, program_loader::elf::elf_file::Elf},
        sel4::{
            boot::seL4_UntypedDesc,
            common::{p_region_t, paddr_to_pptr, pptr_to_paddr, region_t, v_region_t},
            cspace::{cap_t, cte::cte_t},
            gen_caps_from_vm, seL4_ASIDPoolBits, seL4_BootInfoFrameBits, seL4_CapBootInfoFrame,
            seL4_CapInitThreadVspace, seL4_MaxUntypedBits, seL4_MinUntypedBits,
            seL4_NumInitialCaps, seL4_PageBits, seL4_PageTableBits, seL4_SlotBits, seL4_TCBBits,
            seL4_VSpaceBits, seL4_WordBits,
            utils::{is_reg_empty, is_v_reg_empty, write_slot},
            vspace::{arch_get_n_paging, pptr_t, vptr_t},
            wordBits, BI_FRAME_SIZE_BITS, CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS,
            CONFIG_ROOT_CNODE_SIZE_BITS, IT_ASID, MAX_NUM_FREEMEM_REG, MAX_NUM_RESV_REG, PAGE_BITS,
            PAGE_SIZE,
        },
        vm::vmo::{Vmo, VmoFlags, VmoOptions},
        BIT, IS_ALIGNED, MAX_FREE_INDEX, ROUND_UP,
    };
    use crate::{
        sel4::{
            common::pptr_of_cap, seL4_CapDomain, seL4_CapIOPortControl, seL4_CapIRQControl,
            seL4_CapInitThreadCNode, vspace::paddr_t,
        },
        vm::perms::VmPerms,
    };

    use super::{
        acpi_rsdp_t, create_frames_of_region_ret_t, ndks_boot_t, rootserver_mem_t, seL4_BootInfo,
        seL4_BootInfoHeader, seL4_SlotPos, seL4_SlotRegion, seL4_X86_BootInfo_fb_t,
        seL4_X86_BootInfo_mmap_t,
    };

    const CONFIG_KERNEL_MCS: bool = false;

    static mut ndks_boot: ndks_boot_t = ndks_boot_t {
        reserved: [p_region_t { start: 0, end: 0 }; MAX_NUM_RESV_REG],
        resv_count: 0,
        freemem: [region_t { start: 0, end: 0 }; MAX_NUM_FREEMEM_REG],
        bi_frame: 0 as *mut seL4_BootInfo,
        slot_pos_cur: seL4_NumInitialCaps,
    };
    /// addr is kernel address
    pub static mut rootserver_mem: region_t = region_t { start: 0, end: 0 };
    /// offset in RootServer vmo
    pub static mut rootserver: rootserver_mem_t = rootserver_mem_t {
        cnode: 0,
        vspace: 0,
        asid_pool: 0,
        ipc_buf: 0,
        boot_info: 0,
        extra_bi: 0,
        tcb: 0,
        paging: region_t { start: 0, end: 0 },
    };

    pub static mut root_untyped: Option<Box<Vmo>> = None;

    fn calculate_extra_bi_size_bits(extra_size: usize) -> usize {
        if extra_size == 0 {
            return 0;
        }

        let clzl_ret = clzl(ROUND_UP!(extra_size, seL4_PageBits));
        let mut msb = seL4_WordBits - 1 - clzl_ret;
        if extra_size > BIT!(msb) {
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

    pub fn init_sys_state() {
        let x = *initramfs();
        trace!("elf addr is 0x{:x}", x.as_ptr() as usize);
    }

    use crate::Result;
    /// elf is loaded into userspace
    pub fn init_rootserver(vm: &ProcessVm) -> vptr_t {
        // let parsed_elf = Elf::parse_elf(elf_binary)?;

        // let mem_bounds = parsed_elf.memory_bounds();

        // let p_reg = p_region_t {
        //     start: mem_bounds.start as paddr_t,
        //     end: ROUND_UP!(mem_bounds.end as paddr_t, PAGE_BITS),
        // };

        // info!(
        //     "size=0x{:x} v_entry=0x{:x} p_start=0x{:x} p_end=0x{:x}",
        //     p_reg.end - p_reg.end,
        //     parsed_elf.entry_point(),
        //     p_reg.start,
        //     p_reg.end
        // );

        // boot info frame is one page
        let mut extra_bi_size = mem::size_of::<seL4_BootInfoHeader>();

        // ignore vbe
        // for rsdp
        extra_bi_size += mem::size_of::<seL4_BootInfoHeader>() + mem::size_of::<acpi_rsdp_t>();
        // for fb
        extra_bi_size +=
            mem::size_of::<seL4_BootInfoHeader>() + mem::size_of::<seL4_X86_BootInfo_fb_t>();
        // boot mem map info
        extra_bi_size += mem::size_of::<seL4_X86_BootInfo_mmap_t>();
        // room for tsc frequency
        extra_bi_size += mem::size_of::<seL4_BootInfoHeader>() + 4;

        /* the largest object the PD, the root cnode, or the extra boot info */
        let extra_bi_size_bits = calculate_extra_bi_size_bits(extra_bi_size);
        let size = calculate_rootserver_size(extra_bi_size_bits);
        let max = rootserver_max_size_bits(extra_bi_size_bits);

        unsafe {
            let cnode_size_bits = CONFIG_ROOT_CNODE_SIZE_BITS + seL4_SlotBits;
            // vmo offset
            // create rootserver vmo
            let rootserver_vmo = VmoOptions::<Rights>::new(size)
                .flags(VmoFlags::CONTIGUOUS)
                .alloc()
                .unwrap();
            // map
            let start_pptr = paddr_to_pptr(rootserver_vmo.paddr() as paddr_t);
            let start_vptr = vm
                .root_vmar()
                .new_map(rootserver_vmo, VmPerms::READ | VmPerms::WRITE)
                .unwrap()
                .build()
                .unwrap();
            trace!("rootserver vmo start paddr 0x{:x}", start_pptr);
            rootserver_mem.start = start_pptr;
            rootserver_mem.end = start_pptr + size;

            maybe_alloc_extra_bi(max, extra_bi_size_bits);

            if CONFIG_ROOT_CNODE_SIZE_BITS + seL4_SlotBits > seL4_VSpaceBits {
                rootserver.cnode = alloc_rootserver_obj(cnode_size_bits, 1);
                maybe_alloc_extra_bi(seL4_VSpaceBits, extra_bi_size_bits);
                // pagetable's root frame address
                rootserver.vspace = paddr_to_pptr(vm.root_vmar().vm_space().pt_root_paddr());
                // rootserver.vspace = alloc_rootserver_obj(seL4_VSpaceBits, 1);
            } else {
                rootserver.vspace = paddr_to_pptr(vm.root_vmar().vm_space().pt_root_paddr());
                // rootserver.vspace = alloc_rootserver_obj(seL4_VSpaceBits, 1);
                maybe_alloc_extra_bi(cnode_size_bits, extra_bi_size_bits);
                rootserver.cnode = alloc_rootserver_obj(cnode_size_bits, 1);
            }
            /* at this point we are up to creating 4k objects - which is the min size of
             * extra_bi so this is the last chance to allocate it */
            maybe_alloc_extra_bi(seL4_PageBits, extra_bi_size_bits);
            const_assert!(seL4_ASIDPoolBits == seL4_PageBits);
            rootserver.asid_pool = alloc_rootserver_obj(seL4_ASIDPoolBits, 1);
            rootserver.ipc_buf = alloc_rootserver_obj(seL4_PageBits, 1);

            /* The boot info size must be at least one page. Due to the hard-coded order
             * of allocations used in the current implementation here, it can't be any
             * bigger.
             */
            const_assert!(seL4_BootInfoFrameBits == seL4_PageBits);
            rootserver.boot_info = alloc_rootserver_obj(seL4_BootInfoFrameBits, 1);

            /* TCBs on aarch32 can be larger than page tables in certain configs */
            if seL4_TCBBits >= seL4_PageTableBits {
                rootserver.tcb = alloc_rootserver_obj(seL4_TCBBits, 1);
            }

            /* paging structures are 4k on every arch except aarch32 (1k) */
            // let n = arch_get_n_paging(it_v_reg);
            let n = 0;
            // just place a page table in this
            // page is not continuous
            rootserver.paging.start = alloc_rootserver_obj(seL4_PageTableBits, n);
            rootserver.paging.end = rootserver.paging.start + n * BIT!(seL4_PageTableBits);

            /* for most archs, TCBs are smaller than page tables */
            if seL4_TCBBits < seL4_PageTableBits {
                rootserver.tcb = alloc_rootserver_obj(seL4_TCBBits, 1);
            }

            // if CONFIG_KERNEL_MCS {
            //     // rootserver.sc = alloc_rootserver_obj(seL4_MinSchedContextBits, 1);
            // }
            /* we should have allocated all our memory */
            assert!(rootserver_mem.start == rootserver_mem.end);

            let ipcbuf_vptr = start_vptr + (rootserver.ipc_buf - start_pptr);
            let bi_frame_vptr = start_vptr + (rootserver.boot_info - start_pptr);

            let root_cnode_cap = create_root_cnode();
            // create io control port
            let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
            write_slot(
                ptr.add(seL4_CapIOPortControl),
                cap_t::new_io_port_control_cap(),
            );

            /* create the capability for managing thread domains IGNORED*/
            create_domain_cap(&root_cnode_cap);
            /* initialise the IRQ states and provide the IRQ control capability */
            init_irqs(&root_cnode_cap);

            // tsc

            populate_bi_frame(0, 1, ipcbuf_vptr, extra_bi_size);

            /* Construct an initial address space with enough virtual addresses
             * to cover the user image + ipc buffer and bootinfo frames */
            let it_vspace_cap = create_it_address_space(&root_cnode_cap, vm.root_vmar().vm_space());
            // if (cap_get_capType(it_vspace_cap) == cap_null_cap) {
            //     return false;
            // }

            /* Create and map bootinfo frame capability */
            create_bi_frame_cap(&root_cnode_cap, &it_vspace_cap, bi_frame_vptr);

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
            let ipcbuf_cap = create_ipcbuf_frame_cap(&root_cnode_cap, &it_vspace_cap, ipcbuf_vptr);
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

            unsafe {
                (*ndks_boot.bi_frame).numIOPTLevels = usize::MAX;
            }

            // create all of the untypeds. Both devices and kernel window memory
            create_untypeds(&root_cnode_cap);

            // finalise the bootinfo frame
            bi_finalise();
            bi_frame_vptr
        }
    }

    unsafe fn maybe_alloc_extra_bi(cmp_size_bits: usize, extra_bi_size_bits: usize) {
        if extra_bi_size_bits >= cmp_size_bits && rootserver.extra_bi == 0 {
            rootserver.extra_bi = alloc_rootserver_obj(extra_bi_size_bits, 1);
        }
    }

    // ignore code itself
    fn calculate_rootserver_size(extra_bi_size_bits: usize) -> usize {
        let mut size = BIT!(CONFIG_ROOT_CNODE_SIZE_BITS + seL4_SlotBits); // cnodes ??
        size += BIT!(seL4_TCBBits); // root thread tcb
        size += BIT!(seL4_PageBits); // ipc buf
        size += BIT!(BI_FRAME_SIZE_BITS); // boot info
        size += BIT!(seL4_ASIDPoolBits);
        size += if extra_bi_size_bits > 0 {
            BIT!(extra_bi_size_bits)
        } else {
            0
        };
        // no need to allocate root vspace
        // size += BIT!(seL4_VSpaceBits); // root vspace
        size
    }

    fn rootserver_max_size_bits(extra_bi_size_bits: usize) -> usize {
        let cnode_size_bits = CONFIG_ROOT_CNODE_SIZE_BITS + seL4_SlotBits;
        cnode_size_bits.max(seL4_VSpaceBits).max(extra_bi_size_bits)
    }

    fn alloc_rootserver_obj(size_bits: usize, n: usize) -> usize {
        unsafe {
            let allocated = rootserver_mem.start;
            assert!(allocated % BIT!(size_bits) == 0);
            rootserver_mem.start += n * BIT!(size_bits);
            assert!(rootserver_mem.start <= rootserver_mem.end);
            allocated
        }
    }
    fn create_domain_cap(root_cnode_cap: &cap_t) {
        // assert!(ksDomScheduleLength > 0);
        // for i in 0..ksDomScheduleLength {
        // unsafe {
        //     assert!(ksDomSchedule[i].domain < CONFIG_NUM_DOMAINS);
        //     assert!(ksDomSchedule[i].length > 0);
        // }
        // }
        // todo add things here
        let cap = cap_t::new_domain_cap();
        unsafe {
            let pos = root_cnode_cap.get_cap_ptr() as *mut cte_t;
            write_slot(pos.add(seL4_CapDomain), cap);
        }
    }

    // TODO
    fn init_irqs(root_cnode_cap: &cap_t) {
        // for i in 0..maxIRQ + 1 {
        //     if i != irqInvalid {
        //         setIRQState(IRQState::IRQInactive, i);
        //     }
        // }
        // setIRQState(IRQState::IRQTimer, KERNEL_TIMER_IRQ);
        // #[cfg(feature = "ENABLE_SMP")]
        // {
        //     setIRQState(IRQState::IRQIPI, IRQConst::INTERRUPT_IPI_0 as usize);
        //     setIRQState(IRQState::IRQIPI, IRQConst::INTERRUPT_IPI_1 as usize);
        // }
        unsafe {
            let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
            write_slot(ptr.add(seL4_CapIRQControl), cap_t::new_irq_control_cap());
        }
    }

    unsafe fn create_it_address_space(root_cnode_cap: &cap_t, vm_space: &Arc<VmSpace>) -> cap_t {
        let mut slot_pos_before = ndks_boot.slot_pos_cur;
        let vspace_cap = cap_t::new_pml4_cap(IT_ASID, 1, rootserver.vspace);
        let ptr = vspace_cap.get_cap_ptr() as *mut cte_t;
        write_slot(ptr.add(seL4_CapInitThreadVspace), vspace_cap);
        gen_caps_from_vm(vm_space.pt(), root_cnode_cap);
        let slot_pos_after = ndks_boot.slot_pos_cur;
        (*ndks_boot.bi_frame).userImagePaging = seL4_SlotRegion {
            start: slot_pos_before,
            end: slot_pos_after,
        };
        vspace_cap
    }

    pub unsafe fn create_bi_frame_cap(root_cnode_cap: &cap_t, pd_cap: &cap_t, vptr: usize) {
        let cap =
            create_unmapped_it_frame_cap(pd_cap, rootserver.boot_info, vptr, IT_ASID, false, false);
        let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
        write_slot(ptr.add(seL4_CapBootInfoFrame), cap);
    }

    // no need to map already mapped
    pub fn create_unmapped_it_frame_cap(
        pd_cap: &cap_t,
        pptr: usize,
        vptr: usize,
        asid: usize,
        use_large: bool,
        _exec: bool,
    ) -> cap_t {
        let frame_size: usize;
        if use_large {
            // c enum X86_LargePage
            frame_size = 1;
        } else {
            // c enum X86_SmallPage
            frame_size = 0;
        }
        cap_t::new_frame_cap(
            frame_size, 1, // X86_MappingVSpace
            vptr, 3, // VMReadWrite
            0, asid, pptr,
        )
    }

    unsafe fn populate_bi_frame(
        node_id: usize,
        num_nodes: usize,
        ipcbuf_vptr: usize,
        extra_bi_size: usize,
    ) {
        let bi = &mut *(rootserver.boot_info as *mut seL4_BootInfo);
        bi.nodeID = node_id;
        bi.numNodes = num_nodes;
        bi.numIOPTLevels = 0;
        bi.ipcBuffer = ipcbuf_vptr;
        bi.initThreadCNodeSizeBits = CONFIG_ROOT_CNODE_SIZE_BITS;
        // bi.initThreadDomain = ksDomSchedule[ksDomScheduleIdx].domain;
        bi.initThreadDomain = 0;
        bi.extraLen = extra_bi_size;

        ndks_boot.bi_frame = bi as *const seL4_BootInfo as *mut seL4_BootInfo;
        ndks_boot.slot_pos_cur = seL4_NumInitialCaps;
    }

    unsafe fn create_ipcbuf_frame_cap(
        root_cnode_cap: &cap_t,
        pd_cap: &cap_t,
        vptr: usize,
    ) -> cap_t {
        // clear_memory(rootserver.ipc_buf as *mut u8, PAGE_BITS);
        let cap =
            create_unmapped_it_frame_cap(pd_cap, rootserver.ipc_buf, vptr, IT_ASID, false, false);
        let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
        write_slot(ptr, cap);
        return cap;
    }
    // unsafe fn create_initial_thread(
    //     root_cnode_cap: &cap_t,
    //     it_pd_cap: &cap_t,
    //     ui_v_entry: usize,
    //     bi_frame_vptr: usize,
    //     ipcbuf_vptr: usize,
    //     ipcbuf_cap: cap_t,
    // ) -> *mut tcb_t {
    //     let tcb = convert_to_mut_type_ref::<tcb_t>(rootserver.tcb + TCB_OFFSET);
    //     tcb.tcbTimeSlice = CONFIG_TIME_SLICE;
    //     tcb.tcbArch = arch_tcb_t::default();

    //     let cnode = convert_to_mut_type_ref::<cte_t>(root_cnode_cap.get_cap_ptr());
    //     let ipc_buf_slot = cnode.get_offset_slot(seL4_CapInitThreadIPCBuffer);
    //     let dc_ret = ipc_buf_slot.derive_cap(&ipcbuf_cap.clone());
    //     if dc_ret.status != exception_t::EXCEPTION_NONE {
    //         debug!("Failed to derive copy of IPC Buffer\n");
    //         return 0 as *mut tcb_t;
    //     }

    //     cte_insert(
    //         root_cnode_cap,
    //         cnode.get_offset_slot(seL4_CapInitThreadCNode),
    //         tcb.get_cspace_mut_ref(tcbCTable),
    //     );

    //     cte_insert(
    //         it_pd_cap,
    //         cnode.get_offset_slot(seL4_CapInitThreadVspace),
    //         tcb.get_cspace_mut_ref(tcbVTable),
    //     );

    //     cte_insert(
    //         &dc_ret.capability,
    //         cnode.get_offset_slot(seL4_CapInitThreadIPCBuffer),
    //         tcb.get_cspace_mut_ref(tcbBuffer),
    //     );

    //     tcb.tcbIPCBuffer = ipcbuf_vptr;
    //     tcb.set_register(capRegister, bi_frame_vptr);
    //     tcb.set_register(NextIP, ui_v_entry);
    //     tcb.tcbMCP = seL4_MaxPrio;
    //     tcb.tcbPriority = seL4_MaxPrio;
    //     set_thread_state(tcb, ThreadState::ThreadStateRunning);
    //     tcb.setup_reply_master();
    //     unsafe {
    //         ksCurDomain = ksDomSchedule[ksDomScheduleIdx].domain;
    //         ksDomainTime = ksDomSchedule[ksDomScheduleIdx].length;
    //     }
    //     #[cfg(feature = "ENABLE_SMP")]
    //     {
    //         tcb.tcbAffinity = 0;
    //     }

    //     let capability = cap_t::new_thread_cap(tcb.get_ptr());
    //     write_slot(
    //         cnode.get_offset_slot(seL4_CapInitThreadTCB) as *mut cte_t,
    //         capability,
    //     );
    //     // forget(*tcb);
    //     tcb as *mut tcb_t
    // }

    // fn asid_init(root_cnode_cap: cap_t, it_pd_cap: cap_t) -> bool {
    //     let it_ap_cap = create_it_asid_pool(&root_cnode_cap);
    //     if it_ap_cap.get_cap_type() == CapTag::CapNullCap {
    //         debug!("ERROR: could not create ASID pool for initial thread");
    //         return false;
    //     }

    //     unsafe {
    //         let ap = it_ap_cap.get_cap_ptr();
    //         let ptr = (ap + 8 * IT_ASID) as *mut usize;
    //         *ptr = it_pd_cap.get_cap_ptr();
    //         riscvKSASIDTable[IT_ASID >> asidLowBits] = ap as *mut asid_pool_t;
    //     }
    //     true
    // }

    // fn create_it_asid_pool(root_cnode_cap: &cap_t) -> cap_t {
    //     let ap_cap = unsafe { cap_t::new_asid_pool_cap(IT_ASID >> asidLowBits, rootserver.asid_pool) };
    //     unsafe {
    //         let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
    //         write_slot(ptr.add(seL4_CapInitThreadASIDPool), ap_cap.clone());
    //         write_slot(ptr.add(seL4_CapASIDControl), cap_t::new_asid_control_cap());
    //     }
    //     ap_cap
    // }

    // fn create_frame_ui_frames(
    //     root_cnode_cap: cap_t,
    //     it_pd_cap: cap_t,
    //     ui_reg: region_t,
    //     pv_offset: isize,
    // ) -> bool {
    //     let create_frames_ret = rust_create_frames_of_region(
    //         &root_cnode_cap,
    //         &it_pd_cap,
    //         ui_reg,
    //         true,
    //         pv_offset as isize,
    //     );
    //     if !create_frames_ret.success {
    //         debug!("ERROR: could not create all userland image frames");
    //         return false;
    //     }
    //     unsafe {
    //         (*ndks_boot.bi_frame).userImageFrames = create_frames_ret.region;
    //     }
    //     true
    // }

    /// physical memory is mapped for kernel page table
    unsafe fn create_root_cnode() -> cap_t {
        let cap = cap_t::new_cnode_cap(
            CONFIG_ROOT_CNODE_SIZE_BITS,
            wordBits - CONFIG_ROOT_CNODE_SIZE_BITS,
            rootserver.cnode,
            0,
        );
        let ptr = rootserver.cnode as *mut cte_t;
        write_slot(ptr.add(seL4_CapInitThreadCNode), cap);
        cap
    }

    pub unsafe fn provide_cap(root_cnode_cap: &cap_t, cap: cap_t) {
        if ndks_boot.slot_pos_cur >= BIT!(CONFIG_ROOT_CNODE_SIZE_BITS) {
            error!(
                "can't add another capability, all {} (=2^CONFIG_ROOT_CNODE_SIZE_BITS) slots used",
                BIT!(CONFIG_ROOT_CNODE_SIZE_BITS)
            );
            panic!("boom");
        }
        let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
        write_slot(ptr.add(ndks_boot.slot_pos_cur), cap);
        ndks_boot.slot_pos_cur += 1;
    }

    fn create_untypeds(root_cnode_cap: &cap_t) {
        // allocate 40 * PAGES as untyped memory
        let mem = VmoOptions::<Rights>::new(40 * PAGE_SIZE)
            .flags(VmoFlags::CONTIGUOUS)
            .alloc()
            .unwrap();
        let pptr = paddr_to_pptr(mem.paddr());
        let first_untyped_slot = unsafe { ndks_boot.slot_pos_cur };
        if !create_untypeds_for_region(
            root_cnode_cap,
            region_t {
                start: pptr,
                end: pptr + mem.size(),
            },
            first_untyped_slot,
            false,
        ) {
            error!("creation of untypeds failed");
            panic!("gg");
        }
        unsafe {
            let bi = &mut (*ndks_boot.bi_frame);
            bi.untyped = seL4_SlotRegion {
                start: first_untyped_slot,
                end: ndks_boot.slot_pos_cur,
            };
            root_untyped = Some(Box::new(mem));
        }
    }

    fn create_untypeds_for_region(
        root_cnode_cap: &cap_t,
        mut reg: region_t,
        first_untyped_slot: seL4_SlotPos,
        is_device_memory: bool,
    ) -> bool {
        /*
           This code works with regions that wrap (where end < start), because the
           loop cuts up the region into size-aligned chunks, one for each capability. Memory
           chunks that are size-aligned cannot themselves overflow, so they satisfy
           alignment, size, and overflow conditions. The region [0..end) is not
           necessarily part of the kernel window (depending on the value of
           PPTR_BASE). This is fine for device untypeds. For normal untypeds, the
           region is assumed to be fully in the kernel window. This is not checked
           here.
        */
        while !is_reg_empty(&reg) {
            /*
               Calculate the bit size of the region. This is also correct for end < start:
               it will return the correct size of the set [start..-1] union [0..end).
               This will then be too large for alignment, so the code further
               down will reduce the size.
            */
            let mut size_bits =
                seL4_WordBits - 1 - clzl(unsafe { reg.end.unchecked_sub(reg.start) });
            size_bits = size_bits.min(seL4_MaxUntypedBits);
            /* The start address 0 satisfies any alignment needs, otherwise ensure
             * the region's bit size does not exceed the alignment of the region.
             */
            if 0 != reg.start {
                let align_bits = ctzl(reg.start);
                if (size_bits > align_bits) {
                    size_bits = align_bits;
                }
            }
            /* Provide an untyped capability for the region only if it is large
             * enough to be retyped into objects later. Otherwise the region can't
             * be used anyway.
             */
            if (size_bits >= seL4_MinUntypedBits) {
                if (!provide_untyped_cap(
                    root_cnode_cap,
                    is_device_memory,
                    reg.start,
                    size_bits,
                    first_untyped_slot,
                )) {
                    return false;
                }
            }
            reg.start = unsafe { reg.start.unchecked_add(BIT!(size_bits)) };
        }
        true
    }

    fn provide_untyped_cap(
        root_cnode_cap: &cap_t,
        is_device_memory: bool,
        pptr: pptr_t,
        size_bits: usize,
        first_untyped_slot: seL4_SlotPos,
    ) -> bool {
        if size_bits > seL4_MaxUntypedBits || size_bits < seL4_MinUntypedBits {
            error!("invaild size_bits");
            return false;
        }
        // All capability ptrs must be aligned to object size
        if !IS_ALIGNED!(pptr, size_bits) {
            error!("unaligned untyped ponter");
            return false;
        }
        // ignore device mem check
        let i = unsafe { ndks_boot.slot_pos_cur } - first_untyped_slot;
        if i < CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS {
            unsafe {
                let bi = &mut (*ndks_boot.bi_frame);
                bi.untypedList[i] = seL4_UntypedDesc {
                    paddr: pptr_to_paddr(pptr),
                    sizeBits: size_bits as u8,
                    isDevice: is_device_memory as u8,
                    padding: [0; 6],
                }
            }
            let ut_cap = cap_t::new_untyped_cap(
                pptr,
                MAX_FREE_INDEX!(size_bits),
                is_device_memory as usize,
                size_bits,
            );
            unsafe {
                provide_cap(root_cnode_cap, ut_cap);
            }
            true
        } else {
            error!("too many untyped caps");
            true
        }
    }

    fn bi_finalise() {
        unsafe {
            let x = &mut *ndks_boot.bi_frame;
            x.empty = seL4_SlotRegion {
                start: ndks_boot.slot_pos_cur,
                end: BIT!(CONFIG_ROOT_CNODE_SIZE_BITS),
            }
        }
    }
}
