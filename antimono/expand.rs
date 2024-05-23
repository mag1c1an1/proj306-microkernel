pub mod mdb {
    use crate::plus_define_bitfield;
    #[repr(C)]
    pub struct mdb_node_t {
        pub words: [usize; 2],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for mdb_node_t {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "mdb_node_t",
                "words",
                &&self.words,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for mdb_node_t {}
    #[automatically_derived]
    impl ::core::clone::Clone for mdb_node_t {
        #[inline]
        fn clone(&self) -> mdb_node_t {
            let _: ::core::clone::AssertParamIsClone<[usize; 2]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for mdb_node_t {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for mdb_node_t {
        #[inline]
        fn eq(&self, other: &mdb_node_t) -> bool {
            self.words == other.words
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for mdb_node_t {}
    #[automatically_derived]
    impl ::core::cmp::Eq for mdb_node_t {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<[usize; 2]>;
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for mdb_node_t {
        #[inline]
        fn default() -> mdb_node_t {
            mdb_node_t {
                words: ::core::default::Default::default(),
            }
        }
    }
    impl mdb_node_t {
        #[inline]
        pub fn new(
            mdbPrev: usize,
            mdbNext: usize,
            mdbRevocable: usize,
            mdbFirstBadged: usize,
        ) -> Self {
            let mut value = mdb_node_t::default();
            let mask = (((1u128 << 64) - 1)) as usize;
            value.words[0] |= (((mdbPrev >> 0) & mask) << 0);
            let mask = (((1u128 << 46) - 1)) as usize;
            value.words[1] |= (((mdbNext >> 2) & mask) << 0);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[1] |= (((mdbRevocable >> 0) & mask) << 1);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[1] |= (((mdbFirstBadged >> 0) & mask) << 0);
            value.words[0] |= ((0 & ((1usize << 0) - 1)) << 0);
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
            let mask = ((1u128 << 46) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 2;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_next(&mut self, new_field: usize) {
            let mask = ((1u128 << 46) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 2) & mask) << 0);
        }
        #[inline]
        pub fn get_revocable(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[1] >> 1) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_revocable(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[1] &= !(mask << 1);
            self.words[1] |= (((new_field >> 0) & mask) << 1);
        }
        #[inline]
        pub fn get_first_badged(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_first_badged(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_type(&self) -> usize {
            (self.words[0] >> 0) & ((1usize << 0) - 1)
        }
    }
}
