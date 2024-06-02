pub const ROOT_CNODE_SIZE_BITS: usize = 20;

mod test {
    use crate::sel4::cnode::ROOT_CNODE_SIZE_BITS;
    use crate::sel4::config;

    fn root_cnode_size_test() {
        let out = config::consts::ROOT_CNODE_SIZE_BITS.parse().unwrap();
        assert_eq!(ROOT_CNODE_SIZE_BITS, out);
    }
}
