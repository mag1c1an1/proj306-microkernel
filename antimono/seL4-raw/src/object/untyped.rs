// /* It is assumed that every untyped is within seL4_MinUntypedBits and seL4_MaxUntypedBits
//  * (inclusive). This means that every untyped stored as seL4_MinUntypedBits
//  * subtracted from its size before it is stored in capBlockSize, and
//  * capFreeIndex counts in chunks of size 2^seL4_MinUntypedBits. The seL4_MaxUntypedBits
//  * is the minimal untyped that can be stored when considering both how
//  * many bits of capBlockSize there are, and the largest offset that can
//  * be stored in capFreeIndex */
// #define MAX_FREE_INDEX(sizeBits) (BIT((sizeBits) - seL4_MinUntypedBits))
// #define FREE_INDEX_TO_OFFSET(freeIndex) ((freeIndex)<<seL4_MinUntypedBits)
// #define GET_FREE_REF(base,freeIndex) ((word_t)(((word_t)(base)) + FREE_INDEX_TO_OFFSET(freeIndex)))
// #define GET_FREE_INDEX(base,free) (((word_t)(free) - (word_t)(base))>>seL4_MinUntypedBits)
// #define GET_OFFSET_FREE_PTR(base, offset) ((void *)(((word_t)(base)) + (offset)))
// #define OFFSET_TO_FREE_INDEX(offset) ((offset)>>seL4_MinUntypedBits)

pub fn max_free_index(size_bits: u64) -> u64 {
    // seL4_MinUntypedBits
    1 << (size_bits - 4)
}
