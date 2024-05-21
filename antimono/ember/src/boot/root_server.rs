// use super::calculate_extra_bi_size_bits;
// use super::utils::{
//     arch_get_n_paging, create_it_pt_cap, map_it_frame_cap, provide_cap, write_slot,
// };
// use super::{ndks_boot, utils::is_reg_empty};
// use crate::common::sel4_config::{
//     asidLowBits, seL4_PageBits, seL4_PageTableBits, seL4_SlotBits, seL4_TCBBits, tcbBuffer,
//     tcbCTable, tcbVTable, wordBits, CONFIG_MAX_NUM_NODES, CONFIG_NUM_DOMAINS, CONFIG_PT_LEVELS,
//     CONFIG_TIME_SLICE, IT_ASID, PAGE_BITS, TCB_OFFSET,
// };
// use crate::common::structures::{exception_t, seL4_IPCBuffer};
// use crate::common::utils::convert_to_mut_type_ref;
// use crate::cspace::interface::*;
// use crate::interrupt::{setIRQState, IRQState};
// use crate::structures::{
//     create_frames_of_region_ret_t, region_t, rootserver_mem_t, seL4_BootInfo, seL4_SlotRegion,
//     v_region_t,
// };
// use crate::{BIT, ROUND_DOWN};
// use log::debug;

// use crate::config::*;
// use crate::utils::clear_memory;

// use crate::task_manager::*;
// use crate::vspace::*;
// #[no_mangle]
// #[link_section = ".boot.bss"]
// pub static mut rootserver_mem: region_t = region_t { start: 0, end: 0 };

// #[no_mangle]
// #[link_section = ".boot.bss"]
// pub static mut rootserver: rootserver_mem_t = rootserver_mem_t {
//     cnode: 0,
//     vspace: 0,
//     asid_pool: 0,
//     ipc_buf: 0,
//     boot_info: 0,
//     extra_bi: 0,
//     tcb: 0,
//     paging: region_t {
//         start: (0),
//         end: (0),
//     },
// };

// pub fn root_server_init(
//     it_v_reg: v_region_t,
//     extra_bi_size_bits: usize,
//     ipcbuf_vptr: usize,
//     bi_frame_vptr: usize,
//     extra_bi_size: usize,
//     extra_bi_frame_vptr: usize,
//     ui_reg: region_t,
//     pv_offset: isize,
//     v_entry: usize,
// ) -> Option<(*mut tcb_t, cap_t)> {
//     unsafe {
//         root_server_mem_init(it_v_reg, extra_bi_size_bits);
//     }

//     let root_cnode_cap = unsafe { create_root_cnode() };
//     if root_cnode_cap.get_cap_type() == CapTag::CapNullCap {
//         debug!("ERROR: root c-node creation failed\n");
//         return None;
//     }

//     create_domain_cap(&root_cnode_cap);
//     init_irqs(&root_cnode_cap);
//     unsafe {
//         rust_populate_bi_frame(0, CONFIG_MAX_NUM_NODES, ipcbuf_vptr, extra_bi_size);
//     }
//     let it_pd_cap = unsafe { rust_create_it_address_space(&root_cnode_cap, it_v_reg) };
//     if it_pd_cap.get_cap_type() == CapTag::CapNullCap {
//         debug!("ERROR: address space creation for initial thread failed");
//         return None;
//     }

//     if !init_bi_frame_cap(
//         root_cnode_cap,
//         it_pd_cap,
//         bi_frame_vptr,
//         extra_bi_size,
//         extra_bi_frame_vptr,
//     ) {
//         return None;
//     }
//     let ipcbuf_cap = unsafe { create_ipcbuf_frame_cap(&root_cnode_cap, &it_pd_cap, ipcbuf_vptr) };
//     if ipcbuf_cap.get_cap_type() == CapTag::CapNullCap {
//         debug!("ERROR: could not create IPC buffer for initial thread");
//         return None;
//     }

//     if ipcbuf_cap.get_cap_type() == CapTag::CapNullCap {
//         debug!("ERROR: could not create IPC buffer for initial thread");
//         return None;
//     }
//     if !create_frame_ui_frames(root_cnode_cap, it_pd_cap, ui_reg, pv_offset) {
//         return None;
//     }

//     if !asid_init(root_cnode_cap, it_pd_cap) {
//         return None;
//     }

//     let initial = unsafe {
//         create_initial_thread(
//             &root_cnode_cap,
//             &it_pd_cap,
//             v_entry,
//             bi_frame_vptr,
//             ipcbuf_vptr,
//             ipcbuf_cap,
//         )
//     };

//     if initial as usize == 0 {
//         debug!("ERROR: could not create initial thread");
//         return None;
//     }
//     Some((initial, root_cnode_cap))
// }



// unsafe fn create_rootserver_objects(start: usize, it_v_reg: v_region_t, extra_bi_size_bits: usize) {
//     let cnode_size_bits = CONFIG_ROOT_CNODE_SIZE_BITS + seL4_SlotBits;
//     let max = rootserver_max_size_bits(extra_bi_size_bits);

//     let size = calculate_rootserver_size(it_v_reg, extra_bi_size_bits);
//     rootserver_mem.start = start;
//     rootserver_mem.end = start + size;
//     maybe_alloc_extra_bi(max, extra_bi_size_bits);

//     rootserver.cnode = alloc_rootserver_obj(cnode_size_bits, 1);
//     maybe_alloc_extra_bi(seL4_VSpaceBits, extra_bi_size_bits);
//     rootserver.vspace = alloc_rootserver_obj(seL4_VSpaceBits, 1);

//     maybe_alloc_extra_bi(seL4_PageBits, extra_bi_size_bits);
//     rootserver.asid_pool = alloc_rootserver_obj(seL4_ASIDPoolBits, 1);
//     rootserver.ipc_buf = alloc_rootserver_obj(seL4_PageBits, 1);
//     rootserver.boot_info = alloc_rootserver_obj(BI_FRAME_SIZE_BITS, 1);

//     let n = arch_get_n_paging(it_v_reg);
//     rootserver.paging.start = alloc_rootserver_obj(seL4_PageTableBits, n);
//     rootserver.paging.end = rootserver.paging.start + n * BIT!(seL4_PageTableBits);
//     rootserver.tcb = alloc_rootserver_obj(seL4_TCBBits, 1);

//     assert_eq!(rootserver_mem.start, rootserver_mem.end);
// }
