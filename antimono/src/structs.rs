enum CapTag {
    CapNullCap = 0,
    CapUntypedCap = 2,
    CapEndpointCap = 4,
    CapNotificationCap = 6,
    CapReplyCap = 8,
    CapCnodeCap = 10,
    CapThreadCap = 12,
    CapIrqControlCap = 14,
    CapIrqHandlerCap = 16,
    CapZombieCap = 18,
    CapDomainCap = 20,
    CapFrameCap = 1,
    CapPageTableCap = 3,
    CapPageDirectoryCap = 5,
    CapPdptCap = 7,
    CapPml4Cap = 9,
    CapAsidControlCap = 11,
    CapAsidPoolCap = 13,
    CapIoPortCap = 19,
    CapIoPortControlCap = 31 
}

struct Cap
{
  words: [u64;2],
}