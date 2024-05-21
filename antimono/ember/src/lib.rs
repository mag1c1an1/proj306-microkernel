#![no_std]
#![allow(dead_code)]
#![feature(core_intrinsics)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(while_true)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(linkage)]
#![feature(let_chains)]
#![feature(btree_cursors)]
// FIXME: This feature is used to support vm capbility now as a work around.
// Since this is an incomplete feature, use this feature is unsafe.
// We should find a proper method to replace this feature with min_specialization, which is a sound feature.
#![feature(specialization)]
#![feature(const_option)]
#![feature(generic_const_exprs)]
#![allow(warnings)]

use align_ext::AlignExt;
use alloc::{
    sync::Arc,
    vec::{self, Vec},
};
use anti_frame::{
    arch::qemu::{exit_qemu, QemuExitCode},
    boot::initramfs,
    cpu::UserContext,
    sync::Mutex,
    task::{Task, TaskOptions},
    user::{UserEvent, UserMode, UserSpace},
    vm::{PageFlags, Vaddr, VmAllocOptions, VmIo, VmMapOptions, VmSpace, PAGE_SIZE},
};
use error::Error;
use process::Process;
use sel4::create_frames_of_region_ret_t;
use thread::{task::create_new_user_task, Thread};

use crate::{
    thread::kernel_thread::{KernelThreadExt, ThreadOptions},
    vm::vmo::Vmo,
};

#[macro_use]
extern crate log;

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate lazy_static;

// mod boot;
pub(crate) mod sel4;
// mod common;
// mod config;
// mod cspace;
// // mod debug;
// // mod deps;
pub mod error;

// mod exception;
// mod interrupt;
// mod kernel;
// pub(crate) mod loader;
// mod object;
// #[cfg(feature = "ENABLE_SMP")]
// mod smp;
// // mod structures;
mod syscall;
// // mod task_manager;
// #[cfg(feature = "ENABLE_UINTC")]
// mod uintc;
// #[cfg(feature = "ENABLE_UINTC")]
// mod uintr;
// mod utils;
// mod vspace;
pub mod process;
pub mod sched;
pub mod thread;
pub mod utils;
pub mod vm;

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;
/// handle can be used map to a vspace
pub struct Handle {}

pub struct RootServer {}

pub fn init() {
    sched::init();
}

pub fn root_server() {
    let elf_binary = initramfs();
    let argv = Vec::new();
    let envp = Vec::new();
    Process::spawn_user_process(elf_binary, argv, envp, true);
    exit_qemu(QemuExitCode::Success);
}

pub fn run_root_server() -> ! {
    Thread::spawn_kernel_thread(ThreadOptions::new(root_server));
    unreachable!()
}

pub mod tmp {
    use core::mem;

    use anti_frame::vm::{kspace::paddr_to_vaddr, HasPaddr};
    use anti_rights::Rights;
    use static_assertions::const_assert;

    use crate::{
        root_server,
        sel4::{
            acpi_rsdp_t,
            cspace::{cap_t, cte::cte_t, CapTag},
            region_t, rootserver_mem_t, seL4_ASIDPoolBits, seL4_BootInfo, seL4_BootInfoFrameBits,
            seL4_BootInfoHeader, seL4_CapBootInfoFrame, seL4_CapInitThreadCNode,
            seL4_CapInitThreadIPCBuffer, seL4_PageBits, seL4_PageTableBits, seL4_SlotBits,
            seL4_TCBBits, seL4_VSpaceBits, seL4_WordBits, seL4_X86_BootInfo_fb_t,
            seL4_X86_BootInfo_mmap_t,
            utils::write_slot,
            v_region_t, wordBits, BI_FRAME_SIZE_BITS, CONFIG_ROOT_CNODE_SIZE_BITS, IT_ASID,
            PAGE_BITS,
        },
        vm::vmo::{Vmo, VmoFlags, VmoOptions},
        BIT, ROUND_UP,
    };

    const CONFIG_KERNEL_MCS: bool = false;
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

    fn calculate_extra_bi_size_bits(size: usize) -> usize {
        if size == 0 {
            return 0;
        }

        let clzl_ret = ROUND_UP!(size, seL4_PageBits).leading_zeros() as usize;
        let mut msb = seL4_WordBits - 1 - clzl_ret;
        if size > BIT!(msb) {
            msb += 1;
        }
        return msb;
    }
    pub fn create_rootserver_vmo() -> Vmo {
        // start addr in a vmo
        let ipcbuf_vptr = 0;
        // ipcbuf is one page
        let bi_frame_vptr = ipcbuf_vptr + BIT!(PAGE_BITS);
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
            let start = rootserver_vmo.paddr();
            trace!("rootserver vmo start paddr 0x{:x}", start);
            rootserver_mem.start = start;
            rootserver_mem.end = start + size;

            maybe_alloc_extra_bi(max, extra_bi_size_bits);

            if CONFIG_ROOT_CNODE_SIZE_BITS + seL4_SlotBits > seL4_VSpaceBits {
                rootserver.cnode = alloc_rootserver_obj(cnode_size_bits, 1);
                maybe_alloc_extra_bi(seL4_VSpaceBits, extra_bi_size_bits);
                rootserver.vspace = alloc_rootserver_obj(seL4_VSpaceBits, 1);
            } else {
                rootserver.vspace = alloc_rootserver_obj(seL4_VSpaceBits, 1);
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
            //   let n = arch_get_n_paging(it_v_reg);
            let n = 0;
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
            let root_cnode_cap = create_root_cnode();

            // create the io port cap ignore for now
            /* create the cap for managing thread domains IGNORED*/
            /* initialise the IRQ states and provide the IRQ control cap */
            // init_irqs(root_cnode_cap);

            // tsc
            /* Construct an initial address space with enough virtual addresses

            * to cover the user image + ipc buffer and bootinfo frames */
            // let it_vspace_cap = create_it_address_space(&root_cnode_cap, it_v_reg);
            let it_vspace_cap = cap_t::new_null_cap();
            // if (cap_get_capType(it_vspace_cap) == cap_null_cap) {
            //     return false;
            // }

            /* Create and map bootinfo frame cap */
            // create_bi_frame_cap(
            //     root_cnode_cap,
            //     it_vspace_cap,
            //     bi_frame_vptr
            // );

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

            // #ifdef CONFIG_IOMMU
            //     /* initialise VTD-related data structures and the IOMMUs */
            //     if (!vtd_init(cpu_id, rmrr_list)) {
            //         return false;
            //     }

            //     /* write number of IOMMU PT levels into bootinfo */
            //     ndks_boot.bi_frame->numIOPTLevels = x86KSnumIOPTLevels;

            //     /* write IOSpace master cap */
            //     write_slot(SLOT_PTR(pptr_of_cap(root_cnode_cap), seL4_CapIOSpace), master_iospace_cap());
            // #else
            // ndks_boot.bi_frame->numIOPTLevels = -1;
            // #endif

            /* create all of the untypeds. Both devices and kernel window memory */
            // if (!create_untypeds(root_cnode_cap)) {
            //     return false;
            // }

            /* finalise the bootinfo frame */
            // bi_finalise();
            rootserver_vmo
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
        size += BIT!(seL4_VSpaceBits); // root vspace
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
    // fn create_domain_cap(root_cnode_cap: &cap_t) {
    //     assert!(ksDomScheduleLength > 0);
    //     for i in 0..ksDomScheduleLength {
    //         unsafe {
    //             assert!(ksDomSchedule[i].domain < CONFIG_NUM_DOMAINS);
    //             assert!(ksDomSchedule[i].length > 0);
    //         }
    //     }
    //     let cap = cap_t::new_domain_cap();
    //     unsafe {
    //         let pos = root_cnode_cap.get_cap_ptr() as *mut cte_t;
    //         write_slot(pos.add(seL4_CapDomain), cap);
    //     }
    // }

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
        // unsafe {
        //     let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
        //     write_slot(ptr.add(seL4_CapIRQControl), cap_t::new_irq_control_cap());
        // }
    }

    // /// TODO refactor
    unsafe fn create_it_address_space(root_cnode_cap: &cap_t, it_v_reg: v_region_t) -> cap_t {
        todo!()
        // copyGlobalMappings(rootserver.vspace);
        // let lvl1pt_cap = cap_t::new_page_table_cap(IT_ASID, rootserver.vspace, 1, rootserver.vspace);
        // let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
        // let slot_pos_before = ndks_boot.slot_pos_cur;
        // write_slot(ptr.add(seL4_CapInitThreadVspace), lvl1pt_cap.clone());
        // let mut i = 0;
        // while i < CONFIG_PT_LEVELS - 1 {
        //     let mut pt_vptr = ROUND_DOWN!(it_v_reg.start, RISCV_GET_LVL_PGSIZE_BITS(i));
        //     while pt_vptr < it_v_reg.end {
        //         if !provide_cap(
        //             root_cnode_cap,
        //             create_it_pt_cap(&lvl1pt_cap, it_alloc_paging(), pt_vptr, IT_ASID),
        //         ) {
        //             return cap_t::new_null_cap();
        //         }
        //         pt_vptr += RISCV_GET_LVL_PGSIZE(i);
        //     }
        //     i += 1;
        // }
        // let slot_pos_after = ndks_boot.slot_pos_cur;
        // (*ndks_boot.bi_frame).userImagePaging = seL4_SlotRegion {
        //     start: slot_pos_before,
        //     end: slot_pos_after,
        // };
        // lvl1pt_cap
    }

    // fn init_bi_frame_cap(
    //     root_cnode_cap: cap_t,
    //     it_pd_cap: cap_t,
    //     bi_frame_vptr: usize,
    //     extra_bi_size: usize,
    //     extra_bi_frame_vptr: usize,
    // ) -> bool {
    //     unsafe {
    //         create_bi_frame_cap(&root_cnode_cap, &it_pd_cap, bi_frame_vptr);
    //     }
    //     if extra_bi_size > 0 {
    //         let extra_bi_region = unsafe {
    //             region_t {
    //                 start: rootserver.extra_bi,
    //                 end: rootserver.extra_bi + extra_bi_size,
    //             }
    //         };
    //         let extra_bi_ret = rust_create_frames_of_region(
    //             &root_cnode_cap,
    //             &it_pd_cap,
    //             extra_bi_region,
    //             true,
    //             pptr_to_paddr(extra_bi_region.start) as isize - extra_bi_frame_vptr as isize,
    //         );

    //         if !extra_bi_ret.success {
    //             debug!("ERROR: mapping extra boot info to initial thread failed");
    //             return false;
    //         }
    //         unsafe {
    //             (*ndks_boot.bi_frame).extraBIPages = extra_bi_ret.region;
    //         }
    //     }
    //     true
    // }

    // fn rust_create_frames_of_region(
    //     root_cnode_cap: &cap_t,
    //     pd_cap: &cap_t,
    //     reg: region_t,
    //     do_map: bool,
    //     pv_offset: isize,
    // ) -> create_frames_of_region_ret_t {
    //     let slot_pos_before = unsafe { ndks_boot.slot_pos_cur };
    //     let mut f = reg.start;
    //     let mut frame_cap: cap_t;
    //     while f < reg.end {
    //         if do_map {
    //             frame_cap = rust_create_mapped_it_frame_cap(
    //                 pd_cap,
    //                 f,
    //                 pptr_to_paddr((f as isize - pv_offset) as usize),
    //                 IT_ASID,
    //                 false,
    //                 true,
    //             );
    //         } else {
    //             frame_cap = rust_create_unmapped_it_frame_cap(f, false);
    //         }

    //         if !provide_cap(root_cnode_cap, frame_cap) {
    //             return create_frames_of_region_ret_t {
    //                 region: seL4_SlotRegion { start: 0, end: 0 },
    //                 success: false,
    //             };
    //         }
    //         f += BIT!(PAGE_BITS);
    //     }
    //     unsafe {
    //         let slot_pos_after = ndks_boot.slot_pos_cur;
    //         return create_frames_of_region_ret_t {
    //             region: seL4_SlotRegion {
    //                 start: slot_pos_before,
    //                 end: slot_pos_after,
    //             },
    //             success: true,
    //         };
    //     }
    // }

    unsafe fn create_bi_frame_cap(root_cnode_cap: &cap_t, pd_cap: &cap_t, vptr: usize) {
        let cap =
            create_mapped_it_frame_cap(pd_cap, rootserver.boot_info, vptr, IT_ASID, false, false);
        let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
        // write_slot(ptr.add(seL4_CapBootInfoFrame), cap);
    }

    pub fn create_mapped_it_frame_cap(
        pd_cap: &cap_t,
        pptr: usize,
        vptr: usize,
        asid: usize,
        use_large: bool,
        _exec: bool,
    ) -> cap_t {
        todo!()
        // let frame_size: usize;
        // if use_large {
        //     frame_size = RISCVMegaPageBits;
        // } else {
        //     frame_size = RISCVPageBits;
        // }
        // let cap = cap_t::new_frame_cap(asid, pptr, frame_size, VMReadWrite, 0, vptr);
        // map_it_frame_cap(pd_cap, &cap);
        // cap
    }

    // fn rust_create_unmapped_it_frame_cap(pptr: pptr_t, _use_large: bool) -> cap_t {
    //     cap_t::new_frame_cap(0, pptr, 0, 0, 0, 0)
    // }

    // todo
    unsafe fn populate_bi_frame(
        node_id: usize,
        num_nodes: usize,
        ipcbuf_vptr: usize,
        extra_bi_size: usize,
    ) {
        // clear_memory(rootserver.boot_info as *mut u8, BI_FRAME_SIZE_BITS);
        // if extra_bi_size != 0 {
        //     clear_memory(
        //         rootserver.extra_bi as *mut u8,
        //         calculate_extra_bi_size_bits(extra_bi_size),
        //     );
        // }
        let bi = &mut *(rootserver.boot_info as *mut seL4_BootInfo);
        bi.nodeID = node_id;
        bi.numNodes = num_nodes;
        bi.numIOPTLevels = 0;
        bi.ipcBuffer = ipcbuf_vptr;
        bi.initThreadCNodeSizeBits = CONFIG_ROOT_CNODE_SIZE_BITS;
        // bi.initThreadDomain = ksDomSchedule[ksDomScheduleIdx].domain;
        bi.extraLen = extra_bi_size;

        // ndks_boot.bi_frame = bi as *const seL4_BootInfo as *mut seL4_BootInfo;
        // ndks_boot.slot_pos_cur = seL4_NumInitialCaps;
    }

    unsafe fn create_ipcbuf_frame_cap(
        root_cnode_cap: &cap_t,
        pd_cap: &cap_t,
        vptr: usize,
    ) -> cap_t {
        // clear_memory(rootserver.ipc_buf as *mut u8, PAGE_BITS);
        let cap =
            create_mapped_it_frame_cap(pd_cap, rootserver.ipc_buf, vptr, IT_ASID, false, false);
        write_slot(root_cnode_cap.get_cap_ptr(), cap.clone());
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
    //         &dc_ret.cap,
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

    //     let cap = cap_t::new_thread_cap(tcb.get_ptr());
    //     write_slot(
    //         cnode.get_offset_slot(seL4_CapInitThreadTCB) as *mut cte_t,
    //         cap,
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

    // unsafe fn root_server_mem_init(it_v_reg: v_region_t, extra_bi_size_bits: usize) {
    //     let size = calculate_rootserver_size(it_v_reg, extra_bi_size_bits);
    //     let max = rootserver_max_size_bits(extra_bi_size_bits);
    //     let mut i = ndks_boot.freemem.len() - 1;
    //     /* skip any empty regions */
    //     while i != usize::MAX && is_reg_empty(&ndks_boot.freemem[i]) {
    //         i -= 1;
    //     }
    //     while i != usize::MAX && i < ndks_boot.freemem.len() {
    //         /* Invariant: both i and (i + 1) are valid indices in ndks_boot.freemem. */
    //         assert!(i < (ndks_boot.freemem.len() - 1));
    //         /* Invariant; the region at index i is the current candidate.
    //          * Invariant: regions 0 up to (i - 1), if any, are additional candidates.
    //          * Invariant: region (i + 1) is empty. */
    //         assert!(is_reg_empty(&ndks_boot.freemem[i + 1]));

    //         /* Invariant: regions above (i + 1), if any, are empty or too small to use.
    //          * Invariant: all non-empty regions are ordered, disjoint and unallocated. */
    //         /* We make a fresh variable to index the known-empty region, because the
    //          * SimplExportAndRefine verification test has poor support for array
    //          * indices that are sums of variables and small constants. */
    //         let empty_index = i + 1;
    //         /* Try to take the top-most suitably sized and aligned chunk. */
    //         let unaligned_start = ndks_boot.freemem[i].end - size;
    //         let start = ROUND_DOWN!(unaligned_start, max);

    //         /* if unaligned_start didn't underflow, and start fits in the region,
    //          * then we've found a region that fits the root server objects. */
    //         if unaligned_start <= ndks_boot.freemem[i].end && start >= ndks_boot.freemem[i].start {
    //             create_rootserver_objects(start, it_v_reg, extra_bi_size_bits);
    //             ndks_boot.freemem[empty_index] = region_t {
    //                 start: start + size,
    //                 end: ndks_boot.freemem[i].end,
    //             };
    //             ndks_boot.freemem[i].end = start;
    //             return;
    //         }
    //         /* Region i isn't big enough, so shuffle it up to slot (i + 1),
    //          * which we know is unused. */
    //         ndks_boot.freemem[empty_index] = ndks_boot.freemem[i];
    //         ndks_boot.freemem[i] = region_t { start: 0, end: 0 };
    //         i -= 1;
    //     }
    // }

    /// physical memory is mapped for kernel page table
    unsafe fn create_root_cnode() -> cap_t {
        let cap = cap_t::new_cnode_cap(
            CONFIG_ROOT_CNODE_SIZE_BITS,
            wordBits - CONFIG_ROOT_CNODE_SIZE_BITS,
            0,
            rootserver.cnode,
        );
        write_slot(rootserver.cnode, cap.clone());
        cap
    }

    // #[inline]
    // unsafe fn it_alloc_paging() -> usize {
    //     let allocated = rootserver.paging.start;
    //     rootserver.paging.start += BIT!(seL4_PageTableBits);
    //     assert!(rootserver.paging.start <= rootserver.paging.end);
    //     allocated
    // }
}
