// #define TLS_GDT_ENTRY   7
// #define TLS_GDT_SELECTOR ((TLS_GDT_ENTRY << 3) | 3)

// #define IPCBUF_GDT_ENTRY    8
// #define IPCBUF_GDT_SELECTOR ((IPCBUF_GDT_ENTRY << 3) | 3)

// #define seL4_DataFault 0
// #define seL4_InstructionFault 1

// /* for x86-64, the large page size is 2 MiB and huge page size is 1 GiB */
// #define seL4_WordBits           64
// #define seL4_WordSizeBits       3
// #define seL4_PageBits           12
// #define seL4_SlotBits           5
// #if CONFIG_XSAVE_SIZE >= 832
// #define seL4_TCBBits            12
// #else
// #define seL4_TCBBits            11
// #endif
// #define seL4_EndpointBits       4
// #ifdef CONFIG_KERNEL_MCS
// #define seL4_NotificationBits   6
// #define seL4_ReplyBits          5
// #else
// #define seL4_NotificationBits   5
// #endif

// #define seL4_PageTableBits      12
// #define seL4_PageTableEntryBits 3
// #define seL4_PageTableIndexBits 9

// #define seL4_PageDirBits        12
// #define seL4_PageDirEntryBits   3
// #define seL4_PageDirIndexBits   9

// #define seL4_PDPTBits           12
// #define seL4_PDPTEntryBits      3
// #define seL4_PDPTIndexBits      9

// #define seL4_PML4Bits           12
// #define seL4_PML4EntryBits      3
// #define seL4_PML4IndexBits      9
// #define seL4_VSpaceBits seL4_PML4Bits

// #define seL4_IOPageTableBits    12
// #define seL4_LargePageBits      21
// #define seL4_HugePageBits       30
// #define seL4_NumASIDPoolsBits    3
// #define seL4_ASIDPoolBits       12
// #define seL4_ASIDPoolIndexBits 9


// #define seL4_FastMessageRegisters 4

// /* IPC buffer is 1024 bytes, giving size bits of 10 */
// #define seL4_IPCBufferSizeBits 10

// /* First address in the virtual address space that is not accessible to user level */
// #define seL4_UserTop 0x00007ffffffff000