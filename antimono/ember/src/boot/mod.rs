// mod heap;
// mod interface;
// mod mm;
// mod root_server;
// mod untyped;
// mod utils;

// use core::mem::{self, size_of};

// use crate::common::sel4_config::{
//     seL4_PageBits, CONFIG_MAX_NUM_NODES, KERNEL_ELF_BASE, PADDR_TOP, PAGE_BITS,
// };
// use crate::common::utils::convert_to_type_ref;
// use crate::debug::tcb_debug_append;
// use crate::{BIT, ROUND_UP};
// use log::debug;
// use spin::Mutex;

// use crate::boot::root_server::root_server_init;
// use crate::boot::untyped::create_untypeds;
// use crate::boot::utils::paddr_to_pptr_reg;
// use crate::config::*;
// use crate::structures::{
//     ndks_boot_t, p_region_t, region_t, seL4_BootInfo, seL4_BootInfoHeader, seL4_SlotRegion,
//     seL4_X86_BootInfo_mmap_t, v_region_t,
// };

// use crate::task_manager::*;
// use crate::vspace::*;
// pub use root_server::rootserver;

// #[cfg(feature = "ENABLE_SMP")]
// use crate::{
//     common::utils::cpu_id,
//     deps::{clh_lock_acquire, clh_lock_init},
// };

// #[cfg(feature = "ENABLE_SMP")]
// use core::arch::asm;

// use self::mm::init_freemem;

// pub static ksNumCPUs: Mutex<usize> = Mutex::new(0);
// pub static node_boot_lock: Mutex<usize> = Mutex::new(0);

// pub static mut cpu_idle: [bool; CONFIG_MAX_NUM_NODES] = [false; CONFIG_MAX_NUM_NODES];


// fn init_cpu() {
//     #[cfg(feature = "ENABLE_UINTC")]
//     crate::uintc::init();
// }

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

#[no_mangle]
pub static mut rootserver_mem: region_t = region_t { start: 0, end: 0 };

#[no_mangle]
pub static mut rootserver: rootserver_mem_t = rootserver_mem_t {
    cnode: 0,
    vspace: 0,
    asid_pool: 0,
    ipc_buf: 0,
    boot_info: 0,
    extra_bi: 0,
    tcb: 0,
    paging: region_t {
        start: (0),
        end: (0),
    },
};

// fn init_dtb(
//     dtb_size: usize,
//     dtb_phys_addr: usize,
//     extra_bi_size: &mut usize,
// ) -> Option<p_region_t> {
//     let mut dtb_p_reg = p_region_t { start: 0, end: 0 };
//     if dtb_size > 0 {
//         let dtb_phys_end = dtb_phys_addr + dtb_size;
//         if dtb_phys_end < dtb_phys_addr {
//             debug!(
//                 "ERROR: DTB location at {}
//              len {} invalid",
//                 dtb_phys_addr, dtb_size
//             );
//             return None;
//         }
//         if dtb_phys_end >= PADDR_TOP {
//             debug!(
//                 "ERROR: DTB at [{}..{}] exceeds PADDR_TOP ({})\n",
//                 dtb_phys_addr, dtb_phys_end, PADDR_TOP
//             );
//             return None;
//         }

//         (*extra_bi_size) += size_of::<seL4_BootInfoHeader>() + dtb_size;
//         dtb_p_reg = p_region_t {
//             start: dtb_phys_addr,
//             end: dtb_phys_end,
//         };
//     }
//     Some(dtb_p_reg)
// }

// fn init_bootinfo(dtb_size: usize, dtb_phys_addr: usize, extra_bi_size: usize) {
//     let mut extra_bi_offset = 0;
//     let mut header: seL4_BootInfoHeader = seL4_BootInfoHeader { id: 0, len: 0 };
//     if dtb_size > 0 {
//         header.id = SEL4_BOOTINFO_HEADER_FDT;
//         header.len = size_of::<seL4_BootInfoHeader>() + dtb_size;
//         unsafe {
//             *((rootserver.extra_bi + extra_bi_offset) as *mut seL4_BootInfoHeader) = header.clone();
//         }
//         extra_bi_offset += size_of::<seL4_BootInfoHeader>();
//         let src = unsafe {
//             core::slice::from_raw_parts(paddr_to_pptr(dtb_phys_addr) as *const u8, dtb_size)
//         };
//         unsafe {
//             let dst = core::slice::from_raw_parts_mut(
//                 (rootserver.extra_bi + extra_bi_offset) as *mut u8,
//                 dtb_size,
//             );
//             dst.copy_from_slice(src);
//         }
//     }
//     if extra_bi_size > extra_bi_offset {
//         header.id = SEL4_BOOTINFO_HEADER_PADDING;
//         header.len = extra_bi_size - extra_bi_offset;
//         unsafe {
//             *((rootserver.extra_bi + extra_bi_offset) as *mut seL4_BootInfoHeader) = header.clone();
//         }
//     }
// }

// fn bi_finalise(dtb_size: usize, dtb_phys_addr: usize, extra_bi_size: usize) {
//     unsafe {
//         (*ndks_boot.bi_frame).empty = seL4_SlotRegion {
//             start: ndks_boot.slot_pos_cur,
//             end: BIT!(CONFIG_ROOT_CNODE_SIZE_BITS),
//         };
//     }
//     init_bootinfo(dtb_size, dtb_phys_addr, extra_bi_size);
// }

// fn init_core_state(scheduler_action: *mut tcb_t) {
//     unsafe {
//         #[cfg(feature = "ENABLE_SMP")]
//         if scheduler_action as usize != 0 && scheduler_action as usize != 1 {
//             // tcbDebugAppend(scheduler_action);
//             tcb_debug_append(convert_to_type_ref::<tcb_t>(scheduler_action as usize));
//         }
//         let idle_thread = {
//             #[cfg(not(feature = "ENABLE_SMP"))]
//             {
//                 ksIdleThread as *mut tcb_t
//             }
//             #[cfg(feature = "ENABLE_SMP")]
//             {
//                 ksSMP[cpu_id()].ksIdleThread as *mut tcb_t
//             }
//         };

//         // tcbDebugAppend(idle_thread);
//         tcb_debug_append(convert_to_type_ref::<tcb_t>(idle_thread as usize));
//         set_current_scheduler_action(scheduler_action as usize);
//         set_current_thread(get_idle_thread());
//     }
// }

// pub fn try_init_kernel(ui_v_reg: v_region_t, pv_offset: isize, v_entry: usize) {
//     // phy mem to kernel addr

//     let mut extra_bi_size = mem::size_of::<seL4_BootInfoHeader>();
//     let ipcbuf_vptr = ui_v_reg.end;
//     let bi_frame_vptr = ipcbuf_vptr + BIT!(PAGE_BITS);
//     let extra_bi_frame_vptr = bi_frame_vptr + BIT!(BI_FRAME_SIZE_BITS);

//     extra_bi_size += mem::size_of::<seL4_X86_BootInfo_mmap_t>();
//     extra_bi_size += mem::size_of::<seL4_BootInfoHeader>() + 4;
//     // rust_map_kernel_window();
//     // init_cpu();
//     // init_irq_controller();
//     // init_hart();

//     let extra_bi_size_bits = calculate_extra_bi_size_bits(extra_bi_size);

//     // the region of the initial thread in the user image  + ipcbuf and boot info
//     let it_v_reg = v_region_t {
//         start: ui_v_reg.start,
//         end: ROUND_UP!(extra_bi_frame_vptr + BIT!(extra_bi_size_bits), PAGE_BITS),
//     };

//     assert!(
//         it_v_reg.end < USER_TOP,
//         "Userland image virt [{}..{}] execeeds USER_TOP ({})",
//         it_v_reg.start,
//         it_v_reg.end,
//         USER_TOP
//     );

//     // if let Some((initial_thread, root_cnode_cap)) = root_server_init(
//     //     it_v_reg,
//     //     extra_bi_size_bits,
//     //     ipcbuf_vptr,
//     //     bi_frame_vptr,
//     //     extra_bi_size,
//     //     extra_bi_frame_vptr,
//     //     ui_reg,
//     //     pv_offset,
//     //     v_entry,
//     // ) {
//     //     create_idle_thread();
//     //     init_core_state(initial_thread);
//     //     if !create_untypeds(&root_cnode_cap) {
//     //         debug!("ERROR: could not create untypteds for kernel image boot memory");
//     //     }
//     //     unsafe {
//     //         (*ndks_boot.bi_frame).sharedFrames = seL4_SlotRegion { start: 0, end: 0 };

//     //         bi_finalise(0, 0, extra_bi_size);
//     //     }
//     //     // debug!("release_secondary_cores start");
//     //     *ksNumCPUs.lock() = 1;
//     //     #[cfg(feature = "ENABLE_SMP")]
//     //     {
//     //         unsafe {
//     //             clh_lock_init();
//     //             release_secondary_cores();
//     //             clh_lock_acquire(cpu_id(), false);
//     //         }
//     //     }

//     //     debug!("Booting all finished, dropped to user space");
//     //     debug!("\n");
//     // } else {
//     //     return false;
//     // }
// }

use core::intrinsics::size_of;

use anti_frame::boot::initramfs;

use crate::{
    process::program_loader::elf::elf_file::Elf,
    sel4::{
        region_t, rootserver_mem_t, seL4_BootInfoFrameBits, seL4_BootInfoHeader, seL4_PageBits,
        seL4_WordBits, seL4_X86_BootInfo_mmap_t, seL4_X86_mb_mmap_t, v_region_t, PAGE_BITS,
        PAGE_SIZE,
    },
    BIT, ROUND_UP,
};

// pub(crate) fn boot() -> Elf {
//     trace!("in ember");
//     // get user land image
//     let user_image = initramfs();
//     trace!("user_image len : {}", user_image.len());
//     // let x = MEMORY_REGIONS.get().unwrap();
//     // trace!("{:#?}", x);
//     let elf = Elf::parse_elf(user_image).unwrap();
//     let v_entry = elf.entry_point();
//     let ui_v_reg = elf.memory_bounds();
//     trace!("ELF_loading userland images from boot modules:");
//     trace!(
//         "size=0x{:x} v_entry=0x{:x} v_start=0x{:x}, v_end=0x{:x}",
//         ui_v_reg.end - ui_v_reg.start,
//         v_entry,
//         ui_v_reg.start,
//         ui_v_reg.end,
//     );
//     assert!(
//         ui_v_reg.start % PAGE_SIZE == 0,
//         "Userland image virtual start address must be page aligned"
//     );
//     // for ipc buffer frame and bootinfo frame, need 2 * 4K of addditional userland virtual memory
//     assert!(
//         ui_v_reg.end + 2 * (1 << PAGE_BITS) <= 0x7FFFFFFFFFFF,
//         "Userland image virtual end address is too high"
//     );
//     assert!(
//         v_entry >= ui_v_reg.start && v_entry < ui_v_reg.end,
//         "Userland imgae entry point does not lie within userland image"
//     );

//     let ipcbuf_vptr = ui_v_reg.end;
//     let bi_frame_vptr = ipcbuf_vptr + BIT!(PAGE_BITS);
//     let extra_bi_frame_vptr = bi_frame_vptr + BIT!(seL4_BootInfoFrameBits);

//     let mut extra_bi_size = size_of::<seL4_BootInfoHeader>();
//     extra_bi_size += size_of::<seL4_X86_BootInfo_mmap_t>();
//     extra_bi_size += size_of::<seL4_BootInfoHeader>() + 4;

//     let extra_bi_size_bits = calculate_extra_bi_size_bits(extra_bi_size);

//     /* The region of the initial thread is the user image + ipcbuf and boot info */
//     let _it_v_reg = v_region_t {
//         start: ui_v_reg.start,
//         end: ROUND_UP!(extra_bi_frame_vptr + BIT!(extra_bi_size_bits), PAGE_BITS),
//     };

//     // iommu

//     // ignore vbe

//     // trace!("drop to user space");
//     elf
// }

// fn rootserver_mem_init() {}
