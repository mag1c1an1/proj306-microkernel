pub mod structures;
use anti_frame::cpu::GeneralRegs;
pub use structures::*;

pub mod constants {
    use crate::{BIT, MASK};

    // BootInfo related
    pub const seL4_BootInfoFrameBits: usize = seL4_PageBits;

    pub const wordRadix: usize = 6;
    pub const wordBits: usize = BIT!(wordRadix);
    pub const seL4_EndpointBits: usize = 4;
    pub const seL4_NotificationBits: usize = 6;
    pub const seL4_SlotBits: usize = 5;
    pub const seL4_ReplyBits: usize = 4;
    pub const seL4_MinUntypedBits: usize = 4;
    pub const seL4_MaxUntypedBits: usize = 38;
    pub const seL4_CapRightsBits: usize = 4;

    // page table relevant
    pub const PT_SIZE_BITS: usize = 12;
    pub const PAGE_BITS: usize = seL4_PageBits;
    pub const RISCV_4K_Page: usize = 0;
    pub const RISCV_Mega_Page: usize = 1;
    pub const RISCV_Giga_Page: usize = 2;
    pub const RISCV_Tera_Page: usize = 3;

    pub const RISCVPageBits: usize = 12;
    pub const RISCVMegaPageBits: usize = 21;
    pub const RISCVGigaPageBits: usize = 30;

    pub const PML4E_SIZE_BITS: usize = seL4_PML4EntryBits;
    pub const PML4_INDEX_BITS: usize = seL4_PML4IndexBits;
    pub const PDPTE_SIZE_BITS: usize = seL4_PDPTEntryBits;
    pub const PDPT_INDEX_BITS: usize = seL4_PDPTIndexBits;
    pub const PDE_SIZE_BITS: usize = seL4_PageDirEntryBits;
    pub const PD_INDEX_BITS: usize = seL4_PageDirIndexBits;
    pub const PTE_SIZE_BITS: usize = seL4_PageTableEntryBits;
    pub const PT_INDEX_BITS: usize = seL4_PageTableIndexBits;

    pub const seL4_PageTableBits: usize = 12;
    pub const seL4_PageTableEntryBits: usize = 3;
    pub const seL4_PageTableIndexBits: usize = 9;

    pub const seL4_PageDirBits: usize = 12;
    pub const seL4_PageDirEntryBits: usize = 3;
    pub const seL4_PageDirIndexBits: usize = 9;

    pub const seL4_PDPTBits: usize = 12;
    pub const seL4_PDPTEntryBits: usize = 3;
    pub const seL4_PDPTIndexBits: usize = 9;

    pub const seL4_PML4Bits: usize = 12;
    pub const seL4_PML4EntryBits: usize = 3;
    pub const seL4_PML4IndexBits: usize = 9;
    pub const seL4_VSpaceBits: usize = seL4_PML4Bits;

    // pub const PT_INDEX_BITS: usize = 9;
    pub const CONFIG_PT_LEVELS: usize = 3;
    pub const seL4_PageBits: usize = 12;
    pub const seL4_HugePageBits: usize = 30;
    pub const seL4_LargePageBits: usize = 21;

    // ASID relevant
    pub const asidLowBits: usize = 9;
    pub const asidHighBits: usize = 7;
    pub const asidInvalid: usize = 0;
    pub const nASIDPools: usize = BIT!(asidHighBits);
    pub const ASID_BITS: usize = asidHighBits + asidLowBits;
    pub const IT_ASID: usize = 1;

    // boot 相关的常数
    pub const PPTR_TOP: usize = 0xffffffff80000000;
    pub const physBase: usize = 0x80000000;
    pub const KERNEL_ELF_PADDR_BASE: usize = physBase + 0x4000000;
    pub const KERNEL_ELF_BASE: usize = PPTR_TOP + (KERNEL_ELF_PADDR_BASE & MASK!(30));
    pub const KERNEL_ELF_BASE_OFFSET: usize = KERNEL_ELF_BASE - KERNEL_ELF_PADDR_BASE;
    pub const PPTR_BASE: usize = 0xffffff8000000000;
    pub const PADDR_BASE: usize = 0x0;
    pub const PPTR_BASE_OFFSET: usize = PPTR_BASE - PADDR_BASE;
    pub const PADDR_TOP: usize = PPTR_TOP - PPTR_BASE_OFFSET;

    // scheduler relevant
    pub const CONFIG_NUM_DOMAINS: usize = 1;
    pub const CONFIG_NUM_PRIORITIES: usize = 256;
    pub const L2_BITMAP_SIZE: usize = (CONFIG_NUM_PRIORITIES + wordBits - 1) / wordBits;
    pub const NUM_READY_QUEUES: usize = CONFIG_NUM_DOMAINS * CONFIG_NUM_PRIORITIES;
    pub const CONFIG_TIME_SLICE: usize = 5;

    // TCB relevant
    pub const seL4_TCBBits: usize = 10;
    pub const TCB_SIZE_BITS: usize = seL4_TCBBits - 1;
    pub const TCB_OFFSET: usize = BIT!(TCB_SIZE_BITS);
    pub const tcbCTable: usize = 0;
    pub const tcbVTable: usize = 1;
    pub const tcbReply: usize = 2;
    pub const tcbCaller: usize = 3;
    pub const tcbBuffer: usize = 4;
    pub const tcbCNodeEntries: usize = 5;

    // 多核相关
    #[cfg(not(feature = "ENABLE_SMP"))]
    pub const CONFIG_MAX_NUM_NODES: usize = 1;

    #[cfg(feature = "ENABLE_SMP")]
    pub const CONFIG_MAX_NUM_NODES: usize = 4;

    // 错误码
    pub const seL4_NoError: usize = 0;
    pub const seL4_InvalidArgument: usize = 1;
    pub const seL4_InvalidCapability: usize = 2;
    pub const seL4_IllegalOperation: usize = 3;
    pub const seL4_RangeError: usize = 4;
    pub const seL4_AlignmentError: usize = 5;
    pub const seL4_FailedLookup: usize = 6;
    pub const seL4_TruncatedMessage: usize = 7;
    pub const seL4_DeleteFirst: usize = 8;
    pub const seL4_RevokeFirst: usize = 9;
    pub const seL4_NotEnoughMemory: usize = 10;
    pub const seL4_NumErrors: usize = 11;

    // msg info
    pub const seL4_MsgMaxLength: usize = 118;
    pub const seL4_MsgExtraCapBits: usize = 2;
    pub const seL4_MsgMaxExtraCaps: usize = BIT!(seL4_MsgExtraCapBits) - 1;
    pub const MessageID_Syscall: usize = 0;
    pub const MessageID_Exception: usize = 1;

    pub const seL4_IPCBufferSizeBits: usize = 10;

    pub const CONFIG_RESET_CHUNK_BITS: usize = 8;

    pub const USER_STACK_SIZE: usize = 4096 * 2;
    pub const KERNEL_STACK_SIZE: usize = 4096 * 10;
    pub const KERNEL_HEAP_SIZE: usize = 0x800000;
    pub const MEMORY_END: usize = 0x88000000;
    pub const PAGE_SIZE: usize = 0x1000;
    pub const PAGE_SIZE_BITS: usize = 0xc;
    pub const MAX_SYSCALL_NUM: usize = 500;
    pub const MAX_APP_NUM: usize = 16;

    pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
    pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
    pub const CLOCK_FREQ: usize = 12500000;
    pub const BIG_STRIDE: isize = 1024;
    pub const APP_BASE_ADDRESS: usize = 0x84000000;
    pub const APP_SIZE_LIMIT: usize = 0x20000;
    pub const PT_OFFSET_BITS: usize = 12;
    pub const KDEV_BASE: usize = 0xFFFFFFFFC0000000;
    pub const KS_LOG_PPTR: usize = 0xFFFFFFFFFFE00000;
    pub const KERNEL_STACK_ALIGNMENT: usize = 4096;

    //FIXME:this constant is generated , maybe need to transfer from C code
    pub const CONFIG_PADDR_USER_DEVICE_TOP: usize = 0x8000000000;

    pub const MAX_NUM_FREEMEM_REG: usize = 16;
    pub const NUM_RESERVED_REGIONS: usize = 3;
    pub const MAX_NUM_RESV_REG: usize = MAX_NUM_FREEMEM_REG + NUM_RESERVED_REGIONS;

    pub const CONFIG_ROOT_CNODE_SIZE_BITS: usize = 16;
    pub const BI_FRAME_SIZE_BITS: usize = 12;
    pub const seL4_ASIDPoolBits: usize = 12;

    pub const seL4_CapNull: usize = 0;
    pub const seL4_CapInitThreadTCB: usize = 1;
    pub const seL4_CapInitThreadCNode: usize = 2;
    pub const seL4_CapInitThreadVspace: usize = 3;
    pub const seL4_CapIRQControl: usize = 4;
    pub const seL4_CapASIDControl: usize = 5;
    pub const seL4_CapInitThreadASIDPool: usize = 6;
    pub const seL4_CapIOPortControl: usize = 7;
    pub const seL4_CapIOSpace: usize = 8;
    pub const seL4_CapBootInfoFrame: usize = 9;
    pub const seL4_CapInitThreadIPCBuffer: usize = 10;
    pub const seL4_CapDomain: usize = 11;
    pub const seL4_CapSMMUSIDControl: usize = 12;
    pub const seL4_CapSMMUCBControl: usize = 13;
    pub const seL4_NumInitialCaps: usize = 14;

    pub const SIP_SSIP: usize = 1;
    pub const SIP_MSIP: usize = 3;
    pub const SIP_STIP: usize = 5;
    pub const SIP_MTIP: usize = 7;
    pub const SIP_SEIP: usize = 9;
    pub const SIP_MEIP: usize = 11;

    pub const SIE_SSIE: usize = 1;
    pub const SIE_MSIE: usize = 3;
    pub const SIE_STIE: usize = 5;
    pub const SIE_MTIE: usize = 7;
    pub const SIE_SEIE: usize = 9;
    pub const SIE_MEIE: usize = 11;

    pub const badgeRegister: usize = 9;
    pub const seL4_MsgLengthBits: usize = 7;

    pub const RISCVInstructionMisaligned: usize = 0;
    pub const RISCVInstructionAccessFault: usize = 1;
    pub const RISCVInstructionIllegal: usize = 2;
    pub const RISCVBreakPoint: usize = 3;
    pub const RISCVLoadAccessFault: usize = 5;
    pub const RISCVAddressMisaligned: usize = 6;
    pub const RISCVStoreAccessFault: usize = 7;
    pub const RISCVEnvCall: usize = 8;
    pub const RISCVInstructionPageFault: usize = 12;
    pub const RISCVLoadPageFault: usize = 13;
    pub const RISCVStorePageFault: usize = 15;
    pub const RISCVSupervisorTimer: usize = 9223372036854775813;

    pub const n_frameRegisters: usize = 16;
    pub const n_gpRegisters: usize = 16;

    pub const frameRegisters: [usize; n_frameRegisters] =
        [33, 0, 1, 2, 7, 8, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26];
    pub const gpRegisters: [usize; n_gpRegisters] =
        [9, 10, 11, 12, 13, 14, 15, 16, 4, 5, 6, 27, 28, 29, 30, 3];
    pub const thread_control_update_priority: usize = 0x1;
    pub const thread_control_update_ipc_buffer: usize = 0x2;
    pub const thread_control_update_space: usize = 0x4;
    pub const thread_control_update_mcp: usize = 0x8;

    pub const seL4_WordBits: usize = 64;

    pub const seL4_UserTop: usize = 0x00007fffffffffff;
    pub const USER_TOP: usize = seL4_UserTop;

    //IRQConstants
    #[cfg(feature = "ENABLE_SMP")]
    pub const PLIC_IRQ_OFFSET: usize = 0;
    pub const PLIC_MAX_IRQ: usize = 0;

    pub enum IRQConst {
        PLIC_IRQ_OFFSET = 0,
        PLIC_NET,
        PLIC_RESERVE,
        #[cfg(feature = "ENABLE_SMP")]
        INTERRUPT_IPI_0,
        #[cfg(feature = "ENABLE_SMP")]
        INTERRUPT_IPI_1,
        KERNEL_TIMER_IRQ,
    }

    pub const KERNEL_TIMER_IRQ: usize = IRQConst::KERNEL_TIMER_IRQ as usize;

    pub const maxIRQ: usize = KERNEL_TIMER_IRQ;

    pub const irqInvalid: usize = 0;

    pub const SEL4_BOOTINFO_HEADER_FDT: usize = 6;
    pub const SEL4_BOOTINFO_HEADER_PADDING: usize = 0;
    pub const CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS: usize = 230;

    pub const seL4_MaxPrio: usize = 255;

    pub const TIMER_CLOCK_HZ: usize = 10000000;
    pub const MS_IN_S: usize = 1000;
    pub const RESET_CYCLES: usize = (TIMER_CLOCK_HZ / MS_IN_S) * 2;

    pub const seL4_MinPrio: usize = 0;

    pub const CONFIG_MAX_NUM_WORK_UNITS_PER_PREEMPTION: usize = 100;
    pub const CONFIG_RETYPE_FAN_OUT_LIMIT: usize = 256;
}

pub use constants::*;

pub mod utils {
    use anti_frame::vm::kspace::paddr_to_vaddr;

    use crate::{sel4::CONFIG_ROOT_CNODE_SIZE_BITS, BIT, ROUND_DOWN, ROUND_UP};

    use super::{
        cspace::{cap_t, cte::cte_t, mdb::mdb_node_t},
        p_region_t, region_t, seL4_PageBits, v_region_t,
        vspace::{pptr_t, pte_t, vptr_t},
        CONFIG_PT_LEVELS, PADDR_TOP, PPTR_BASE, PPTR_TOP,
    };
    mod macros {
        #[macro_export]
        macro_rules! plus_define_bitfield {
    ($name:ident, $total_words:expr, $type_index:expr, $type_offset:expr, $type_bits:expr =>
        { $($variant:ident, $type_value:expr => { $($field:ident, $get_field:ident, $set_field:ident, $index:expr, $offset:expr, $bits:expr, $shift:expr, $sign_ext: expr),* }),* }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
        #[repr(C)]
        pub struct $name {
            pub words: [usize; $total_words],
        }

        impl $name {
            $(
                #[inline]
                pub fn $variant($($field: usize),*) -> Self {
                    let mut value = $name::default();
                    $(
                        let mask = (((1u128 << $bits) - 1)) as usize;
                        value.words[$index] |= ((($field >> $shift) & mask) << $offset);
                    )*
                    value.words[$type_index] |= (($type_value & ((1usize << $type_bits) - 1)) << $type_offset);

                    value
                }

                $(
                    #[inline]
                    pub fn $get_field(&self) -> usize {
                        let mask = ((1u128 << $bits) - 1) as usize;
                        let mut ret = ((self.words[$index] >> $offset) & mask) << $shift;
                        if $sign_ext && (ret & (1usize << 47)) != 0 {
                            ret |= 0xffff000000000000;
                        }
                        ret
                    }

                    #[inline]
                    pub fn $set_field(&mut self, new_field: usize) {
                        let mask = ((1u128 << $bits) - 1) as usize;
                        self.words[$index] &= !(mask << $offset);
                        self.words[$index] |= (((new_field >> $shift) & mask) << $offset);
                    }
                )*
            )*

            #[inline]
            pub fn get_type(&self) -> usize {
                (self.words[$type_index] >> $type_offset) & ((1usize << $type_bits) - 1)
            }
        }
    };
}

        #[macro_export]
        macro_rules! MASK {
    ($e:expr) => {
        {
             (1usize << $e) - 1usize
        }
    }
}

        #[macro_export]
        macro_rules! ROUND_DOWN {
            ($n:expr,$b:expr) => {{
                ((($n) >> ($b)) << ($b))
            }};
        }

        #[macro_export]
        macro_rules! ROUND_UP {
            ($n:expr,$b:expr) => {{
                ((((($n) - 1usize) >> ($b)) + 1usize) << ($b))
            }};
        }

        #[macro_export]
        macro_rules! IS_ALIGNED {
            ($n:expr,$b:expr) => {{
                $n & MASK!($b) == 0
            }};
        }

        pub fn ARRAY_SIZE<T>(arr: &[T]) -> usize {
            arr.len()
        }

        #[macro_export]
        macro_rules! BIT {
    ($e:expr) => {
        {
            1usize<<$e
        }
    }
}
    }

    pub fn MAX_FREE_INDEX(bits: usize) -> usize {
        // BIT!(bits - seL4_MinUntypedBits)
        todo!()
    }

    #[inline]
    pub fn convert_to_type_ref<T>(addr: usize) -> &'static T {
        assert_ne!(addr, 0);
        unsafe { &*(addr as *mut T) }
    }

    #[inline]
    pub fn convert_to_mut_type_ref<T>(addr: usize) -> &'static mut T {
        assert_ne!(addr, 0);
        unsafe { &mut *(addr as *mut T) }
    }

    #[inline]
    pub fn convert_to_mut_type_ref_unsafe<T>(addr: usize) -> &'static mut T {
        unsafe { &mut *(addr as *mut T) }
    }

    #[inline]
    pub fn convert_to_option_type_ref<T>(addr: usize) -> Option<&'static T> {
        if addr == 0 {
            return None;
        }
        Some(convert_to_type_ref::<T>(addr))
    }

    #[inline]
    pub fn convert_to_option_mut_type_ref<T>(addr: usize) -> Option<&'static mut T> {
        if addr == 0 {
            return None;
        }
        Some(convert_to_mut_type_ref::<T>(addr))
    }

    #[no_mangle]
    #[inline]
    pub fn pageBitsForSize(page_size: usize) -> usize {
        match page_size {
            _ => panic!("Invalid page size!"),
        }
    }

    #[inline]
    pub fn cpu_id() -> usize {
        #[cfg(feature = "ENABLE_SMP")]
        {
            use crate::smp::get_currenct_cpu_index;
            // unsafe { getCurrentCPUIndex() }
            get_currenct_cpu_index()
        }
        #[cfg(not(feature = "ENABLE_SMP"))]
        {
            0
        }
    }
    #[inline]
    pub fn is_reg_empty(reg: &region_t) -> bool {
        reg.start == reg.end
    }

    pub fn paddr_to_pptr(p: usize) -> usize {
        todo!()
    }
    pub fn pptr_to_paddr(p: usize) -> usize {
        todo!()
    }

    #[inline]
    pub fn paddr_to_pptr_reg(reg: &p_region_t) -> region_t {
        region_t {
            start: paddr_to_pptr(reg.start),
            end: paddr_to_pptr(reg.end),
        }
    }

    pub fn ceiling_kernel_window(mut p: usize) -> usize {
        if pptr_to_paddr(p) > PADDR_TOP {
            p = PPTR_TOP;
        }
        p
    }

    #[inline]
    pub fn pptr_to_paddr_reg(reg: region_t) -> p_region_t {
        p_region_t {
            start: pptr_to_paddr(reg.start),
            end: pptr_to_paddr(reg.end),
        }
    }

    pub fn pptr_in_kernel_window(pptr: usize) -> bool {
        pptr >= PPTR_BASE && pptr < PPTR_TOP
    }

    #[inline]
    pub fn get_n_paging(v_reg: v_region_t, bits: usize) -> usize {
        let start = ROUND_DOWN!(v_reg.start, bits);
        let end = ROUND_UP!(v_reg.end, bits);
        (end - start) / BIT!(bits)
    }

    pub fn arch_get_n_paging(it_v_reg: v_region_t) -> usize {
        todo!()
        // let mut n: usize = 0;
        // for i in 0..CONFIG_PT_LEVELS - 1 {
        //     n += get_n_paging(it_v_reg, RISCV_GET_LVL_PGSIZE_BITS(i));
        // }
        // return n;
    }

    /// ptr is paddr
    pub fn write_slot(ptr: usize, cap: cap_t) {
        unsafe {
            // contert it to kernel vaddr
            let ptr = paddr_to_vaddr(ptr) as *mut cte_t;
            (*ptr).cap = cap;
            (*ptr).cteMDBNode = mdb_node_t::default();
            let mdb = &mut (*ptr).cteMDBNode;
            mdb.set_revocable(1);
            mdb.set_first_badged(1);
        }
    }

    pub fn provide_cap(root_cnode_cap: &cap_t, cap: cap_t) -> bool {
        todo!()
        // unsafe {
        //     if ndks_boot.slot_pos_cur >= BIT!(CONFIG_ROOT_CNODE_SIZE_BITS) {
        //         debug!(
        //         "ERROR: can't add another cap, all {} (=2^CONFIG_ROOT_CNODE_SIZE_BITS) slots used",
        //         BIT!(CONFIG_ROOT_CNODE_SIZE_BITS)
        //     );
        //         return false;
        //     }
        //     let ptr = root_cnode_cap.get_cap_ptr() as *mut cte_t;
        //     write_slot(ptr.add(ndks_boot.slot_pos_cur), cap);
        //     ndks_boot.slot_pos_cur += 1;
        //     return true;
        // }
    }

    #[no_mangle]
    pub fn map_it_pt_cap(_vspace_cap: &cap_t, _pt_cap: &cap_t) {
        todo!()
        // let vptr = _pt_cap.get_pt_mapped_address();
        // let lvl1pt = convert_to_mut_type_ref::<pte_t>(_vspace_cap.get_cap_ptr());
        // let pt = _pt_cap.get_cap_ptr();
        // let pt_ret = lvl1pt.lookup_pt_slot(vptr);
        // let targetSlot = convert_to_mut_type_ref::<cte_t>(pt_ret.ptSlot as usize);
        // *targetSlot = pte_t::new(
        //     pptr_to_paddr(pt) >> seL4_PageBits,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     0,
        //     1,
        // );
    }

    pub fn create_it_pt_cap(vspace_cap: &cap_t, pptr: pptr_t, vptr: vptr_t, asid: usize) -> cap_t {
        todo!()
        // let cap = cap_t::new_page_table_cap(asid, pptr, 1, vptr);
        // map_it_pt_cap(vspace_cap, &cap);
        // return cap;
    }

    // #[no_mangle]
    pub fn map_it_frame_cap(_vspace_cap: &cap_t, _frame_cap: &cap_t) {
        // let vptr = _frame_cap.get_frame_mapped_address();
        // let lvl1pt = convert_to_mut_type_ref::<pte_t>(_vspace_cap.get_cap_ptr());
        // let frame_pptr: usize = _frame_cap.get_cap_ptr();
        // let pt_ret = lvl1pt.lookup_pt_slot(vptr);

        // let targetSlot = convert_to_mut_type_ref::<pte_t>(pt_ret.ptSlot as usize);
        // *targetSlot = pte_t::new(
        //     pptr_to_paddr(frame_pptr) >> seL4_PageBits,
        //     0,
        //     1,
        //     1,
        //     0,
        //     1,
        //     1,
        //     1,
        //     1,
        //     1,
        // );
    }

    pub fn rust_create_unmapped_it_frame_cap(pptr: pptr_t, _use_large: bool) -> cap_t {
        // cap_t::new_frame_cap(0, pptr, 0, 0, 0, 0)
        todo!()
    }

    pub fn write_it_asid_pool(it_ap_cap: &cap_t, it_lvl1pt_cap: &cap_t) {
        todo!()
        // let ap = it_ap_cap.get_cap_ptr();
        // unsafe {
        //     let ptr = (ap + 8 * IT_ASID) as *mut usize;
        //     *ptr = it_lvl1pt_cap.get_cap_ptr();
        //     riscvKSASIDTable[IT_ASID >> asidLowBits] = ap as *mut asid_pool_t;
        // }
    }
}

pub trait SeL4Regs {
    fn get_cap_reg(&self) -> usize;
    fn get_badge_reg(&self) -> usize;
    fn get_msg_info_reg(&self) -> usize;
    fn set_tls(&mut self, tls: usize);
}

impl SeL4Regs for GeneralRegs {
    fn get_cap_reg(&self) -> usize {
        self.rdi
    }

    fn get_badge_reg(&self) -> usize {
        self.rdi
    }

    fn get_msg_info_reg(&self) -> usize {
        self.rsi
    }

    fn set_tls(&mut self, tls: usize) {
        self.fsbase = tls;
    }
}
