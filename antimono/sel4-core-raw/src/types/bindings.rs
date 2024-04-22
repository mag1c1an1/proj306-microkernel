#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::seL4_MessageInfo_t;


pub const seL4_True: u32 = 1;
pub const seL4_False: u32 = 0;
pub const TLS_GDT_ENTRY: u32 = 7;
pub const TLS_GDT_SELECTOR: u32 = 59;
pub const IPCBUF_GDT_ENTRY: u32 = 8;
pub const IPCBUF_GDT_SELECTOR: u32 = 67;
pub const seL4_DataFault: u32 = 0;
pub const seL4_InstructionFault: u32 = 1;
pub const seL4_WordBits: u32 = 64;
pub const seL4_WordSizeBits: u32 = 3;
pub const seL4_PageBits: u32 = 12;
pub const seL4_SlotBits: u32 = 5;
pub const seL4_TCBBits: u32 = 11;
pub const seL4_EndpointBits: u32 = 4;
pub const seL4_NotificationBits: u32 = 5;
pub const seL4_PageTableBits: u32 = 12;
pub const seL4_PageTableEntryBits: u32 = 3;
pub const seL4_PageTableIndexBits: u32 = 9;
pub const seL4_PageDirBits: u32 = 12;
pub const seL4_PageDirEntryBits: u32 = 3;
pub const seL4_PageDirIndexBits: u32 = 9;
pub const seL4_PDPTBits: u32 = 12;
pub const seL4_PDPTEntryBits: u32 = 3;
pub const seL4_PDPTIndexBits: u32 = 9;
pub const seL4_PML4Bits: u32 = 12;
pub const seL4_PML4EntryBits: u32 = 3;
pub const seL4_PML4IndexBits: u32 = 9;
pub const seL4_VSpaceBits: u32 = 12;
pub const seL4_IOPageTableBits: u32 = 12;
pub const seL4_LargePageBits: u32 = 21;
pub const seL4_HugePageBits: u32 = 30;
pub const seL4_NumASIDPoolsBits: u32 = 3;
pub const seL4_ASIDPoolBits: u32 = 12;
pub const seL4_ASIDPoolIndexBits: u32 = 9;
pub const seL4_MinUntypedBits: u32 = 4;
pub const seL4_MaxUntypedBits: u32 = 47;
pub const seL4_FastMessageRegisters: u32 = 4;
pub const seL4_IPCBufferSizeBits: u32 = 10;
pub const seL4_UserTop: u64 = 140737488351232;
pub const seL4_X86_IOPageTableObject: u32 = 16777215;
pub const seL4_X86_VCPUObject: u32 = 16777214;
pub const seL4_X86_EPTPML4Object: u32 = 16777213;
pub const seL4_X86_EPTPDPTObject: u32 = 16777212;
pub const seL4_X86_EPTPDObject: u32 = 16777211;
pub const seL4_X86_EPTPTObject: u32 = 16777210;
pub const seL4_CapRightsBits: u32 = 4;
pub const seL4_GuardSizeBits: u32 = 6;
pub const seL4_GuardBits: u32 = 58;
pub const seL4_BadgeBits: u32 = 64;
pub const seL4_UntypedRetypeMaxObjects: u32 = 256;
pub const seL4_NilData: u32 = 0;
pub const IRQ_OFFSET: u32 = 48;
pub const VECTOR_MIN: u32 = 0;
pub const VECTOR_MAX: u32 = 109;
pub const MSI_MIN: u32 = 0;
pub const MSI_MAX: u32 = 109;
pub const seL4_VCPUBits: u32 = 14;
pub const seL4_X86_VCPUBits: u32 = 14;
pub const seL4_X86_EPTPML4EntryBits: u32 = 3;
pub const seL4_X86_EPTPML4IndexBits: u32 = 9;
pub const seL4_X86_EPTPML4Bits: u32 = 12;
pub const seL4_X86_EPTPDPTEntryBits: u32 = 3;
pub const seL4_X86_EPTPDPTIndexBits: u32 = 9;
pub const seL4_X86_EPTPDPTBits: u32 = 12;
pub const seL4_X86_EPTPDEntryBits: u32 = 3;
pub const seL4_X86_EPTPDIndexBits: u32 = 9;
pub const seL4_X86_EPTPDBits: u32 = 12;
pub const seL4_X86_EPTPTEntryBits: u32 = 3;
pub const seL4_X86_EPTPTIndexBits: u32 = 9;
pub const seL4_X86_EPTPTBits: u32 = 12;
pub const seL4_BootInfoFrameBits: u32 = 12;
pub const SEL4_MAPPING_LOOKUP_LEVEL: u32 = 2;
pub const SEL4_MAPPING_LOOKUP_NO_PT: u32 = 21;
pub const SEL4_MAPPING_LOOKUP_NO_PD: u32 = 30;
pub const SEL4_MAPPING_LOOKUP_NO_PDPT: u32 = 39;
pub const SEL4_MAPPING_LOOKUP_NO_EPTPDPT: u32 = 39;
pub const SEL4_MAPPING_LOOKUP_NO_EPTPD: u32 = 30;
pub const SEL4_MAPPING_LOOKUP_NO_EPTPT: u32 = 21;
pub type seL4_Int8 = core::ffi::c_schar;
pub type seL4_Uint8 = core::ffi::c_uchar;
pub type seL4_Int16 = core::ffi::c_short;
pub type seL4_Uint16 = core::ffi::c_ushort;
pub type seL4_Int32 = core::ffi::c_int;
pub type seL4_Uint32 = core::ffi::c_uint;
pub type seL4_Int64 = core::ffi::c_long;
pub type seL4_Uint64 = core::ffi::c_ulong;
pub type seL4_Bool = seL4_Int8;
pub type seL4_Word = seL4_Uint64;
pub type seL4_CPtr = seL4_Word;
pub mod seL4_VMFault_Msg {
    pub type Type = core::ffi::c_ulong;
    pub const seL4_VMFault_IP: Type = 0;
    pub const seL4_VMFault_Addr: Type = 1;
    pub const seL4_VMFault_PrefetchFault: Type = 2;
    pub const seL4_VMFault_FSR: Type = 3;
    pub const seL4_VMFault_Length: Type = 4;
    pub const _enum_pad_seL4_VMFault_Msg: Type = 9223372036854775807;
}
pub mod seL4_UnknownSyscall_Msg {
    pub type Type = core::ffi::c_ulong;
    pub const seL4_UnknownSyscall_RAX: Type = 0;
    pub const seL4_UnknownSyscall_RBX: Type = 1;
    pub const seL4_UnknownSyscall_RCX: Type = 2;
    pub const seL4_UnknownSyscall_RDX: Type = 3;
    pub const seL4_UnknownSyscall_RSI: Type = 4;
    pub const seL4_UnknownSyscall_RDI: Type = 5;
    pub const seL4_UnknownSyscall_RBP: Type = 6;
    pub const seL4_UnknownSyscall_R8: Type = 7;
    pub const seL4_UnknownSyscall_R9: Type = 8;
    pub const seL4_UnknownSyscall_R10: Type = 9;
    pub const seL4_UnknownSyscall_R11: Type = 10;
    pub const seL4_UnknownSyscall_R12: Type = 11;
    pub const seL4_UnknownSyscall_R13: Type = 12;
    pub const seL4_UnknownSyscall_R14: Type = 13;
    pub const seL4_UnknownSyscall_R15: Type = 14;
    pub const seL4_UnknownSyscall_FaultIP: Type = 15;
    pub const seL4_UnknownSyscall_SP: Type = 16;
    pub const seL4_UnknownSyscall_FLAGS: Type = 17;
    pub const seL4_UnknownSyscall_Syscall: Type = 18;
    pub const seL4_UnknownSyscall_Length: Type = 19;
    pub const _enum_pad_seL4_UnknownSyscall_Msg: Type = 9223372036854775807;
}
pub mod seL4_UserException_Msg {
    pub type Type = core::ffi::c_ulong;
    pub const seL4_UserException_FaultIP: Type = 0;
    pub const seL4_UserException_SP: Type = 1;
    pub const seL4_UserException_FLAGS: Type = 2;
    pub const seL4_UserException_Number: Type = 3;
    pub const seL4_UserException_Code: Type = 4;
    pub const seL4_UserException_Length: Type = 5;
    pub const _enum_pad_seL4_UserException_Msg: Type = 9223372036854775807;
}
pub type seL4_X64_PML4 = seL4_CPtr;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_UserContext_ {
    pub rip: seL4_Word,
    pub rsp: seL4_Word,
    pub rflags: seL4_Word,
    pub rax: seL4_Word,
    pub rbx: seL4_Word,
    pub rcx: seL4_Word,
    pub rdx: seL4_Word,
    pub rsi: seL4_Word,
    pub rdi: seL4_Word,
    pub rbp: seL4_Word,
    pub r8: seL4_Word,
    pub r9: seL4_Word,
    pub r10: seL4_Word,
    pub r11: seL4_Word,
    pub r12: seL4_Word,
    pub r13: seL4_Word,
    pub r14: seL4_Word,
    pub r15: seL4_Word,
    pub fs_base: seL4_Word,
    pub gs_base: seL4_Word,
}
#[test]
fn bindgen_test_layout_seL4_UserContext_() {
    const UNINIT: core::mem::MaybeUninit<seL4_UserContext_> = core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_UserContext_>(),
        160usize,
        concat!("Size of: ", stringify!(seL4_UserContext_))
    );
    assert_eq!(
        core::mem::align_of::<seL4_UserContext_>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_UserContext_))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rip) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rip)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rsp) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rsp)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rflags) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rflags)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rax) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rax)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rbx) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rbx)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rcx) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rcx)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rdx) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rdx)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rsi) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rsi)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rdi) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rdi)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rbp) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(rbp)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).r8) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(r8)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).r9) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(r9)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).r10) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(r10)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).r11) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(r11)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).r12) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(r12)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).r13) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(r13)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).r14) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(r14)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).r15) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(r15)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).fs_base) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(fs_base)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).gs_base) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UserContext_),
            "::",
            stringify!(gs_base)
        )
    );
}
pub type seL4_UserContext = seL4_UserContext_;
pub type seL4_X86_ASIDControl = seL4_CPtr;
pub type seL4_X86_ASIDPool = seL4_CPtr;
pub type seL4_X86_IOSpace = seL4_CPtr;
pub type seL4_X86_IOPort = seL4_CPtr;
pub type seL4_X86_IOPortControl = seL4_CPtr;
pub type seL4_X86_Page = seL4_CPtr;
pub type seL4_X86_PDPT = seL4_CPtr;
pub type seL4_X86_PageDirectory = seL4_CPtr;
pub type seL4_X86_PageTable = seL4_CPtr;
pub type seL4_X86_IOPageTable = seL4_CPtr;
pub type seL4_X86_EPTPML4 = seL4_CPtr;
pub type seL4_X86_EPTPDPT = seL4_CPtr;
pub type seL4_X86_EPTPD = seL4_CPtr;
pub type seL4_X86_EPTPT = seL4_CPtr;
pub type seL4_X86_VCPU = seL4_CPtr;
pub mod seL4_X86_VMAttributes {
    pub type Type = core::ffi::c_ulong;
    pub const seL4_X86_Default_VMAttributes: Type = 0;
    pub const seL4_X86_WriteBack: Type = 0;
    pub const seL4_X86_WriteThrough: Type = 1;
    pub const seL4_X86_CacheDisabled: Type = 2;
    pub const seL4_X86_Uncacheable: Type = 3;
    pub const seL4_X86_WriteCombining: Type = 4;
    pub const _enum_pad_seL4_X86_VMAttributes: Type = 9223372036854775807;
}
pub mod seL4_X86_EPT_VMAttributes {
    pub type Type = core::ffi::c_ulong;
    pub const seL4_X86_EPT_Uncached_VMAttributes: Type = 6;
    pub const seL4_X86_EPT_Uncacheable: Type = 0;
    pub const seL4_X86_EPT_WriteCombining: Type = 1;
    pub const seL4_X86_EPT_WriteThrough: Type = 4;
    pub const seL4_X86_EPT_WriteProtected: Type = 5;
    pub const seL4_X86_EPT_WriteBack: Type = 6;
    pub const seL4_X86_EPT_Default_VMAttributes: Type = 6;
    pub const _enum_pad_seL4_X86_EPT_VMAttributes: Type = 9223372036854775807;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_VCPUContext_ {
    pub eax: seL4_Word,
    pub ebx: seL4_Word,
    pub ecx: seL4_Word,
    pub edx: seL4_Word,
    pub esi: seL4_Word,
    pub edi: seL4_Word,
    pub ebp: seL4_Word,
}
#[test]
fn bindgen_test_layout_seL4_VCPUContext_() {
    const UNINIT: core::mem::MaybeUninit<seL4_VCPUContext_> = core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_VCPUContext_>(),
        56usize,
        concat!("Size of: ", stringify!(seL4_VCPUContext_))
    );
    assert_eq!(
        core::mem::align_of::<seL4_VCPUContext_>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_VCPUContext_))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).eax) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_VCPUContext_),
            "::",
            stringify!(eax)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).ebx) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_VCPUContext_),
            "::",
            stringify!(ebx)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).ecx) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_VCPUContext_),
            "::",
            stringify!(ecx)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).edx) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_VCPUContext_),
            "::",
            stringify!(edx)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).esi) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_VCPUContext_),
            "::",
            stringify!(esi)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).edi) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_VCPUContext_),
            "::",
            stringify!(edi)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).ebp) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_VCPUContext_),
            "::",
            stringify!(ebp)
        )
    );
}
pub type seL4_VCPUContext = seL4_VCPUContext_;
pub mod api_object {
    pub type Type = core::ffi::c_uint;
    pub const seL4_UntypedObject: Type = 0;
    pub const seL4_TCBObject: Type = 1;
    pub const seL4_EndpointObject: Type = 2;
    pub const seL4_NotificationObject: Type = 3;
    pub const seL4_CapTableObject: Type = 4;
    pub const seL4_NonArchObjectTypeCount: Type = 5;
}
pub use self::api_object::Type as seL4_ObjectType;
pub mod _mode_object {
    pub type Type = core::ffi::c_uint;
    pub const seL4_X86_PDPTObject: Type = 5;
    pub const seL4_X64_PML4Object: Type = 6;
    pub const seL4_X64_HugePageObject: Type = 7;
    pub const seL4_ModeObjectTypeCount: Type = 8;
}
pub use self::_mode_object::Type as seL4_seL4ArchObjectType;
pub mod _object {
    pub type Type = core::ffi::c_uint;
    pub const seL4_X86_4K: Type = 8;
    pub const seL4_X86_LargePageObject: Type = 9;
    pub const seL4_X86_PageTableObject: Type = 10;
    pub const seL4_X86_PageDirectoryObject: Type = 11;
    pub const seL4_ObjectTypeCount: Type = 12;
}
pub use self::_object::Type as seL4_ArchObjectType;
pub mod seL4_Error {
    pub type Type = core::ffi::c_uint;
    pub const seL4_NoError: Type = 0;
    pub const seL4_InvalidArgument: Type = 1;
    pub const seL4_InvalidCapability: Type = 2;
    pub const seL4_IllegalOperation: Type = 3;
    pub const seL4_RangeError: Type = 4;
    pub const seL4_AlignmentError: Type = 5;
    pub const seL4_FailedLookup: Type = 6;
    pub const seL4_TruncatedMessage: Type = 7;
    pub const seL4_DeleteFirst: Type = 8;
    pub const seL4_RevokeFirst: Type = 9;
    pub const seL4_NotEnoughMemory: Type = 10;
    pub const seL4_NumErrors: Type = 11;
}
pub mod priorityConstants {
    pub type Type = core::ffi::c_int;
    pub const seL4_InvalidPrio: Type = -1;
    pub const seL4_MinPrio: Type = 0;
    pub const seL4_MaxPrio: Type = 255;
}
pub mod seL4_MsgLimits {
    pub type Type = core::ffi::c_uint;
    pub const seL4_MsgLengthBits: Type = 7;
    pub const seL4_MsgExtraCapBits: Type = 2;
    pub const seL4_MsgMaxLength: Type = 120;
}
pub mod seL4_LookupFailureType {
    pub type Type = core::ffi::c_ulong;
    pub const seL4_NoFailure: Type = 0;
    pub const seL4_InvalidRoot: Type = 1;
    pub const seL4_MissingCapability: Type = 2;
    pub const seL4_DepthMismatch: Type = 3;
    pub const seL4_GuardMismatch: Type = 4;
    pub const _enum_pad_seL4_LookupFailureType: Type = 9223372036854775807;
}
#[repr(C)]
pub struct seL4_IPCBuffer_ {
    pub tag: seL4_MessageInfo_t,
    pub msg: [seL4_Word; 120usize],
    pub userData: seL4_Word,
    pub caps_or_badges: [seL4_Word; 3usize],
    pub receiveCNode: seL4_CPtr,
    pub receiveIndex: seL4_CPtr,
    pub receiveDepth: seL4_Word,
}
#[test]
fn bindgen_test_layout_seL4_IPCBuffer_() {
    const UNINIT: core::mem::MaybeUninit<seL4_IPCBuffer_> = core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_IPCBuffer_>(),
        1024usize,
        concat!("Size of: ", stringify!(seL4_IPCBuffer_))
    );
    assert_eq!(
        core::mem::align_of::<seL4_IPCBuffer_>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_IPCBuffer_))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).tag) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_IPCBuffer_),
            "::",
            stringify!(tag)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).msg) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_IPCBuffer_),
            "::",
            stringify!(msg)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).userData) as usize - ptr as usize },
        968usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_IPCBuffer_),
            "::",
            stringify!(userData)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).caps_or_badges) as usize - ptr as usize },
        976usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_IPCBuffer_),
            "::",
            stringify!(caps_or_badges)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).receiveCNode) as usize - ptr as usize },
        1000usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_IPCBuffer_),
            "::",
            stringify!(receiveCNode)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).receiveIndex) as usize - ptr as usize },
        1008usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_IPCBuffer_),
            "::",
            stringify!(receiveIndex)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).receiveDepth) as usize - ptr as usize },
        1016usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_IPCBuffer_),
            "::",
            stringify!(receiveDepth)
        )
    );
}
impl Default for seL4_IPCBuffer_ {
    fn default() -> Self {
        let mut s = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type seL4_IPCBuffer = seL4_IPCBuffer_;
pub mod seL4_CapFault_Msg {
    pub type Type = core::ffi::c_ulong;
    pub const seL4_CapFault_IP: Type = 0;
    pub const seL4_CapFault_Addr: Type = 1;
    pub const seL4_CapFault_InRecvPhase: Type = 2;
    pub const seL4_CapFault_LookupFailureType: Type = 3;
    pub const seL4_CapFault_BitsLeft: Type = 4;
    pub const seL4_CapFault_DepthMismatch_BitsFound: Type = 5;
    pub const seL4_CapFault_GuardMismatch_GuardFound: Type = 5;
    pub const seL4_CapFault_GuardMismatch_BitsFound: Type = 6;
    pub const _enum_pad_seL4_CapFault_Msg: Type = 9223372036854775807;
}
pub type seL4_NodeId = seL4_Word;
pub type seL4_PAddr = seL4_Word;
pub type seL4_Domain = seL4_Word;
pub type seL4_CNode = seL4_CPtr;
pub type seL4_IRQHandler = seL4_CPtr;
pub type seL4_IRQControl = seL4_CPtr;
pub type seL4_TCB = seL4_CPtr;
pub type seL4_Untyped = seL4_CPtr;
pub type seL4_DomainSet = seL4_CPtr;
pub type seL4_SchedContext = seL4_CPtr;
pub type seL4_SchedControl = seL4_CPtr;
pub type seL4_Time = seL4_Uint64;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_X86_VCPU_ReadMSR {
    pub error: core::ffi::c_int,
    pub value: seL4_Word,
}
#[test]
fn bindgen_test_layout_seL4_X86_VCPU_ReadMSR() {
    const UNINIT: core::mem::MaybeUninit<seL4_X86_VCPU_ReadMSR> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_X86_VCPU_ReadMSR>(),
        16usize,
        concat!("Size of: ", stringify!(seL4_X86_VCPU_ReadMSR))
    );
    assert_eq!(
        core::mem::align_of::<seL4_X86_VCPU_ReadMSR>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_X86_VCPU_ReadMSR))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_VCPU_ReadMSR),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_VCPU_ReadMSR),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_X86_VCPU_WriteMSR {
    pub error: core::ffi::c_int,
    pub written: seL4_Word,
}
#[test]
fn bindgen_test_layout_seL4_X86_VCPU_WriteMSR() {
    const UNINIT: core::mem::MaybeUninit<seL4_X86_VCPU_WriteMSR> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_X86_VCPU_WriteMSR>(),
        16usize,
        concat!("Size of: ", stringify!(seL4_X86_VCPU_WriteMSR))
    );
    assert_eq!(
        core::mem::align_of::<seL4_X86_VCPU_WriteMSR>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_X86_VCPU_WriteMSR))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_VCPU_WriteMSR),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).written) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_VCPU_WriteMSR),
            "::",
            stringify!(written)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_X86_PageDirectory_GetStatusBits {
    pub error: core::ffi::c_int,
    pub accessed: seL4_Word,
    pub dirty: seL4_Word,
}
#[test]
fn bindgen_test_layout_seL4_X86_PageDirectory_GetStatusBits() {
    const UNINIT: core::mem::MaybeUninit<seL4_X86_PageDirectory_GetStatusBits> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_X86_PageDirectory_GetStatusBits>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(seL4_X86_PageDirectory_GetStatusBits)
        )
    );
    assert_eq!(
        core::mem::align_of::<seL4_X86_PageDirectory_GetStatusBits>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(seL4_X86_PageDirectory_GetStatusBits)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_PageDirectory_GetStatusBits),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).accessed) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_PageDirectory_GetStatusBits),
            "::",
            stringify!(accessed)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).dirty) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_PageDirectory_GetStatusBits),
            "::",
            stringify!(dirty)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_X86_VCPU_ReadVMCS {
    pub error: core::ffi::c_int,
    pub value: seL4_Word,
}
#[test]
fn bindgen_test_layout_seL4_X86_VCPU_ReadVMCS() {
    const UNINIT: core::mem::MaybeUninit<seL4_X86_VCPU_ReadVMCS> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_X86_VCPU_ReadVMCS>(),
        16usize,
        concat!("Size of: ", stringify!(seL4_X86_VCPU_ReadVMCS))
    );
    assert_eq!(
        core::mem::align_of::<seL4_X86_VCPU_ReadVMCS>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_X86_VCPU_ReadVMCS))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_VCPU_ReadVMCS),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_VCPU_ReadVMCS),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_X86_VCPU_WriteVMCS {
    pub error: core::ffi::c_int,
    pub written: seL4_Word,
}
#[test]
fn bindgen_test_layout_seL4_X86_VCPU_WriteVMCS() {
    const UNINIT: core::mem::MaybeUninit<seL4_X86_VCPU_WriteVMCS> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_X86_VCPU_WriteVMCS>(),
        16usize,
        concat!("Size of: ", stringify!(seL4_X86_VCPU_WriteVMCS))
    );
    assert_eq!(
        core::mem::align_of::<seL4_X86_VCPU_WriteVMCS>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_X86_VCPU_WriteVMCS))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_VCPU_WriteVMCS),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).written) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_X86_VCPU_WriteVMCS),
            "::",
            stringify!(written)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_TCB_GetBreakpoint {
    pub error: core::ffi::c_int,
    pub vaddr: seL4_Word,
    pub type_: seL4_Word,
    pub size: seL4_Word,
    pub rw: seL4_Word,
    pub is_enabled: seL4_Bool,
}
#[test]
fn bindgen_test_layout_seL4_TCB_GetBreakpoint() {
    const UNINIT: core::mem::MaybeUninit<seL4_TCB_GetBreakpoint> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_TCB_GetBreakpoint>(),
        48usize,
        concat!("Size of: ", stringify!(seL4_TCB_GetBreakpoint))
    );
    assert_eq!(
        core::mem::align_of::<seL4_TCB_GetBreakpoint>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_TCB_GetBreakpoint))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_TCB_GetBreakpoint),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).vaddr) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_TCB_GetBreakpoint),
            "::",
            stringify!(vaddr)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_TCB_GetBreakpoint),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).size) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_TCB_GetBreakpoint),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).rw) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_TCB_GetBreakpoint),
            "::",
            stringify!(rw)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).is_enabled) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_TCB_GetBreakpoint),
            "::",
            stringify!(is_enabled)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_TCB_ConfigureSingleStepping {
    pub error: core::ffi::c_int,
    pub bp_was_consumed: seL4_Bool,
}
#[test]
fn bindgen_test_layout_seL4_TCB_ConfigureSingleStepping() {
    const UNINIT: core::mem::MaybeUninit<seL4_TCB_ConfigureSingleStepping> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_TCB_ConfigureSingleStepping>(),
        8usize,
        concat!("Size of: ", stringify!(seL4_TCB_ConfigureSingleStepping))
    );
    assert_eq!(
        core::mem::align_of::<seL4_TCB_ConfigureSingleStepping>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(seL4_TCB_ConfigureSingleStepping)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_TCB_ConfigureSingleStepping),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).bp_was_consumed) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_TCB_ConfigureSingleStepping),
            "::",
            stringify!(bp_was_consumed)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_SchedContext_Consumed {
    pub error: core::ffi::c_int,
    pub consumed: seL4_Time,
}
#[test]
fn bindgen_test_layout_seL4_SchedContext_Consumed() {
    const UNINIT: core::mem::MaybeUninit<seL4_SchedContext_Consumed> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_SchedContext_Consumed>(),
        16usize,
        concat!("Size of: ", stringify!(seL4_SchedContext_Consumed))
    );
    assert_eq!(
        core::mem::align_of::<seL4_SchedContext_Consumed>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_SchedContext_Consumed))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_SchedContext_Consumed),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).consumed) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_SchedContext_Consumed),
            "::",
            stringify!(consumed)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_SchedContext_YieldTo {
    pub error: core::ffi::c_int,
    pub consumed: seL4_Time,
}
#[test]
fn bindgen_test_layout_seL4_SchedContext_YieldTo() {
    const UNINIT: core::mem::MaybeUninit<seL4_SchedContext_YieldTo> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_SchedContext_YieldTo>(),
        16usize,
        concat!("Size of: ", stringify!(seL4_SchedContext_YieldTo))
    );
    assert_eq!(
        core::mem::align_of::<seL4_SchedContext_YieldTo>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_SchedContext_YieldTo))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_SchedContext_YieldTo),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).consumed) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_SchedContext_YieldTo),
            "::",
            stringify!(consumed)
        )
    );
}
pub mod seL4_RootCNodeCapSlots {
    pub type Type = core::ffi::c_uint;
    pub const seL4_CapNull: Type = 0;
    pub const seL4_CapInitThreadTCB: Type = 1;
    pub const seL4_CapInitThreadCNode: Type = 2;
    pub const seL4_CapInitThreadVSpace: Type = 3;
    pub const seL4_CapIRQControl: Type = 4;
    pub const seL4_CapASIDControl: Type = 5;
    pub const seL4_CapInitThreadASIDPool: Type = 6;
    pub const seL4_CapIOPortControl: Type = 7;
    pub const seL4_CapIOSpace: Type = 8;
    pub const seL4_CapBootInfoFrame: Type = 9;
    pub const seL4_CapInitThreadIPCBuffer: Type = 10;
    pub const seL4_CapDomain: Type = 11;
    pub const seL4_CapSMMUSIDControl: Type = 12;
    pub const seL4_CapSMMUCBControl: Type = 13;
    pub const seL4_CapInitThreadSC: Type = 14;
    pub const seL4_CapSMC: Type = 15;
    pub const seL4_NumInitialCaps: Type = 16;
}
pub type seL4_SlotPos = seL4_Word;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_SlotRegion {
    pub start: seL4_SlotPos,
    pub end: seL4_SlotPos,
}
#[test]
fn bindgen_test_layout_seL4_SlotRegion() {
    const UNINIT: core::mem::MaybeUninit<seL4_SlotRegion> = core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_SlotRegion>(),
        16usize,
        concat!("Size of: ", stringify!(seL4_SlotRegion))
    );
    assert_eq!(
        core::mem::align_of::<seL4_SlotRegion>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_SlotRegion))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).start) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_SlotRegion),
            "::",
            stringify!(start)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).end) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_SlotRegion),
            "::",
            stringify!(end)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_UntypedDesc {
    pub paddr: seL4_Word,
    pub sizeBits: seL4_Uint8,
    pub isDevice: seL4_Uint8,
    pub padding: [seL4_Uint8; 6usize],
}
#[test]
fn bindgen_test_layout_seL4_UntypedDesc() {
    const UNINIT: core::mem::MaybeUninit<seL4_UntypedDesc> = core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_UntypedDesc>(),
        16usize,
        concat!("Size of: ", stringify!(seL4_UntypedDesc))
    );
    assert_eq!(
        core::mem::align_of::<seL4_UntypedDesc>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_UntypedDesc))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).paddr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UntypedDesc),
            "::",
            stringify!(paddr)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).sizeBits) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UntypedDesc),
            "::",
            stringify!(sizeBits)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).isDevice) as usize - ptr as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UntypedDesc),
            "::",
            stringify!(isDevice)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).padding) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_UntypedDesc),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct seL4_BootInfo {
    pub extraLen: seL4_Word,
    pub nodeID: seL4_NodeId,
    pub numNodes: seL4_Word,
    pub numIOPTLevels: seL4_Word,
    pub ipcBuffer: *mut seL4_IPCBuffer,
    pub empty: seL4_SlotRegion,
    pub sharedFrames: seL4_SlotRegion,
    pub userImageFrames: seL4_SlotRegion,
    pub userImagePaging: seL4_SlotRegion,
    pub ioSpaceCaps: seL4_SlotRegion,
    pub extraBIPages: seL4_SlotRegion,
    pub initThreadCNodeSizeBits: seL4_Word,
    pub initThreadDomain: seL4_Domain,
    pub untyped: seL4_SlotRegion,
    pub untypedList: [seL4_UntypedDesc; 230usize],
}
#[test]
fn bindgen_test_layout_seL4_BootInfo() {
    const UNINIT: core::mem::MaybeUninit<seL4_BootInfo> = core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_BootInfo>(),
        3848usize,
        concat!("Size of: ", stringify!(seL4_BootInfo))
    );
    assert_eq!(
        core::mem::align_of::<seL4_BootInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_BootInfo))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).extraLen) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(extraLen)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).nodeID) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(nodeID)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).numNodes) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(numNodes)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).numIOPTLevels) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(numIOPTLevels)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).ipcBuffer) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(ipcBuffer)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).empty) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(empty)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).sharedFrames) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(sharedFrames)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).userImageFrames) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(userImageFrames)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).userImagePaging) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(userImagePaging)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).ioSpaceCaps) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(ioSpaceCaps)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).extraBIPages) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(extraBIPages)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).initThreadCNodeSizeBits) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(initThreadCNodeSizeBits)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).initThreadDomain) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(initThreadDomain)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).untyped) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(untyped)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).untypedList) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfo),
            "::",
            stringify!(untypedList)
        )
    );
}
impl Default for seL4_BootInfo {
    fn default() -> Self {
        let mut s = core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub mod seL4_BootInfoID {
    pub type Type = core::ffi::c_ulong;
    pub const SEL4_BOOTINFO_HEADER_PADDING: Type = 0;
    pub const SEL4_BOOTINFO_HEADER_X86_VBE: Type = 1;
    pub const SEL4_BOOTINFO_HEADER_X86_MBMMAP: Type = 2;
    pub const SEL4_BOOTINFO_HEADER_X86_ACPI_RSDP: Type = 3;
    pub const SEL4_BOOTINFO_HEADER_X86_FRAMEBUFFER: Type = 4;
    pub const SEL4_BOOTINFO_HEADER_X86_TSC_FREQ: Type = 5;
    pub const SEL4_BOOTINFO_HEADER_FDT: Type = 6;
    pub const SEL4_BOOTINFO_HEADER_NUM: Type = 7;
    pub const _enum_pad_seL4_BootInfoID: Type = 9223372036854775807;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct seL4_BootInfoHeader {
    pub id: seL4_Word,
    pub len: seL4_Word,
}
#[test]
fn bindgen_test_layout_seL4_BootInfoHeader() {
    const UNINIT: core::mem::MaybeUninit<seL4_BootInfoHeader> =
        core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        core::mem::size_of::<seL4_BootInfoHeader>(),
        16usize,
        concat!("Size of: ", stringify!(seL4_BootInfoHeader))
    );
    assert_eq!(
        core::mem::align_of::<seL4_BootInfoHeader>(),
        8usize,
        concat!("Alignment of ", stringify!(seL4_BootInfoHeader))
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfoHeader),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { core::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(seL4_BootInfoHeader),
            "::",
            stringify!(len)
        )
    );
}
