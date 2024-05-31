use crate::define_bitfield_type;

define_bitfield_type!(
    MdbNode,2 => {
        prev, set_prev,0,0..64,0,false,
        next, set_next,0,64..126,0,true,
        revocable,set_revocable,0,126..127,0,false,
        first_badged,set_first_badged,0,127..128,0,false,
    }
);

#[cfg(ktest)]
mod test {
    use ktest::ktest;

    use crate::cspace::raw::mdb::MdbNode;

    #[ktest]
    fn mdb_test() {
        let prev = 0x0usize;
        let next = 0x8000_0000_0000usize;
        let revocable = 0x1usize;
        let first_badged = 0x1usize;

        let mdb = MdbNode::new(prev, next, revocable, first_badged);

        assert_eq!(mdb.prev(), prev);
        assert_eq!(mdb.next(), (0xffff << 48) | next);
        assert_eq!(mdb.revocable(), revocable);
        assert_eq!(mdb.first_badged(), first_badged);
    }
}
