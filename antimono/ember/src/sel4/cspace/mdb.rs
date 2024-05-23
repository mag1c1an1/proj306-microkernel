// 两个机器字组成，维护一个双向链表，其中还有revocable和firstbadged两个标志位字段。
//
// revocable：可以在不通知对象持有者的情况下被删除或撤销。
//
// firstbadged：表示此能力是否是具有相同对象和相同类型的一组能力中的第一个被赋予badge的能力。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct mdb_node_t {
    pub words: [usize; 2],
}
impl mdb_node_t {
    #[inline]
    pub fn new(mdbPrev: usize, mdbNext: usize, mdbRevocable: usize, mdbFirstBadged: usize) -> Self {
        let mut value = mdb_node_t::default();
        value.words[0] = 0 | mdbPrev << 0;
        value.words[1] = 0
            | (mdbNext & 0xfffffffffffc) >> 0
            | (mdbRevocable & 0x1) << 1
            | (mdbFirstBadged & 0x1) << 0;
        value
    }
    #[inline]
    pub fn get_prev(&self) -> usize {
        let mask = ((1u128 << 64) - 1) as usize;
        let mut ret = ((self.words[0] >> 0) & mask) << 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_prev(&mut self, new_field: usize) {
        let mask = ((1u128 << 64) - 1) as usize;
        self.words[0] &= !(mask << 0);
        self.words[0] |= (((new_field >> 0) & mask) << 0);
    }
    #[inline]
    pub fn get_next(&self) -> usize {
        let mut ret = (self.words[1] & 0xfffffffffffc) << 0;
        if true && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_next(&mut self, new_field: usize) {
        let mask = ((1u128 << 46) - 1) as usize;
        self.words[1] &= !0xfffffffffffc;
        self.words[1] |= (new_field >> 0) & 0xfffffffffffc;
    }
    #[inline]
    pub fn get_revocable(&self) -> usize {
        let mut ret = (self.words[1] & 0x2) >> 1;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_revocable(&mut self, new_field: usize) {
        self.words[1] &= !0x2;
        self.words[1] |= (new_field << 1) & 0x2;
    }
    #[inline]
    pub fn get_first_badged(&self) -> usize {
        let mut ret = (self.words[1] & 0x1) >> 0;
        if false && (ret & (1usize << 47)) != 0 {
            ret |= 0xffff000000000000;
        }
        ret
    }
    #[inline]
    pub fn set_first_badged(&mut self, new_field: usize) {
        self.words[1] &= !0x1;
        self.words[1] |= (new_field << 0) & 0x1;
    }
}
