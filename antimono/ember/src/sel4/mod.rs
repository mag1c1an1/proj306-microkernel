use anti_frame::{
    cpu::GeneralRegs,
    vm::page_table::{PageTable, UserMode},
};

pub mod boot;
pub mod common;
pub mod cspace;
pub mod exception;
pub mod utils;
pub mod vspace;

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
    pub const seL4_MaxUntypedBits: usize = 47;
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

    pub const PML4_INDEX_OFFSET: usize = 12 + 9 + 9 + 9;
    pub const PDPT_INDEX_OFFSET: usize = 12 + 9 + 9;
    pub const PD_INDEX_OFFSET: usize = 12 + 9;
    pub const PT_INDEX_OFFSET: usize = 12;

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
    pub const sel4_CapInitThreadSC: usize = 14;
    pub const seL4_CapSMC: usize = 15;
    pub const seL4_NumInitialCaps: usize = 16;

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
use pod::Pod;

use crate::sel4::common::paddr_to_pptr;

use self::{
    boot::bootstrap::provide_cap,
    cspace::{cap_t, cte::cte_t},
};

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

pub trait SeL4PageTable {
    /// generates page table related caps from a page table
    fn provide_caps(&self, cap: &cap_t);
}

use anti_frame::vm::page_table::frame::Child;
impl SeL4PageTable for PageTable<UserMode> {
    // all the pt caps without pml4
    fn provide_caps(&self, cap: &cap_t) {
        unsafe {
            let root_frame = self.frame();
            for (idx, child) in root_frame.lock().childs().iter().enumerate() {
                match child {
                    Child::PageTable(node) => {
                        // let vptr =  idx <<
                        // create_it_pdpt_cap()
                    }
                    _ => panic!("wrong"),
                }
            }
        }
    }
}

/// generate page table related caps from a page table
pub fn gen_caps_from_vm(pt: &PageTable<UserMode>, root_cnode_cap: &cap_t) {
    unsafe {
        // pml4
        let root_frame = pt.frame();
        for (idx, child) in root_frame.lock().childs().iter().enumerate() {
            // child is pdpt
            match child {
                Child::PageTable(node) => {
                    let pdpt_vptr = idx << PML4_INDEX_OFFSET;
                    let base_pptr = paddr_to_pptr(node.lock().start_paddr());
                    let pdpt_cap = cap_t::new_pdpt_cap(1, pdpt_vptr, IT_ASID, base_pptr);
                    provide_cap(root_cnode_cap, pdpt_cap);
                    for (idx, child) in node.lock().childs().iter().enumerate() {
                        // child is pd
                        match child {
                            Child::PageTable(node) => {
                                let pd_vptr = pdpt_vptr | (idx << PDPT_INDEX_OFFSET);
                                let base_pptr = paddr_to_pptr(node.lock().start_paddr());
                                let pd_cap =
                                    cap_t::new_page_directory_cap(1, pd_vptr, IT_ASID, base_pptr);
                                provide_cap(root_cnode_cap, pd_cap);
                                for (idx, child) in node.lock().childs().iter().enumerate() {
                                    // child is pt
                                    match child {
                                        Child::PageTable(node) => {
                                            let pt_vptr = pd_vptr | (idx << PD_INDEX_OFFSET);
                                            let base_pptr =
                                                paddr_to_pptr(node.lock().start_paddr());
                                            let pt_cap = cap_t::new_page_table_cap(
                                                1, pt_vptr, IT_ASID, base_pptr,
                                            );
                                            provide_cap(root_cnode_cap, pt_cap);
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                _ => continue,
            }
        }
    }
}
