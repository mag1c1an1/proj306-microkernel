struct RootServer {
    cnode: usize,
    vspace: usize,
    asid_pool: usize,
    ipc_buf: usize,
    boot_info: usize,
    extra_bi: usize,
    tcb: usize,
}

// 分配内存?
pub fn create_rootserver_objects() {}
/**
 * A region [start..end) of kernel-virtual memory.
 *
 * Empty when start == end. If end < start, the region wraps around, that is,
 * it represents the addresses in the set [start..-1] union [0..end). This is
 * possible after address translation and fine for e.g. device memory regions.
 */
struct Region;
// typedef struct region {
//     pptr_t start; /* inclusive */
//     pptr_t end; /* exclusive */
// } region_t;

// /** A region [start..end) of physical memory addresses. */
// typedef struct p_region {
//     paddr_t start; /* inclusive */
//     paddr_t end; /* exclusive */
// } p_region_t;

// /** A region [start..end) of user-virtual addresses. */
// typedef struct v_region {
//     vptr_t start; /* inclusive */
//     vptr_t end; /* exclusive */
// } v_region_t;
