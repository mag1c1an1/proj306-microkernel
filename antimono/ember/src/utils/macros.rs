#[macro_export]
macro_rules! plus_define_bitfield {
    ($name:ident, $total_words:expr, $type_index:expr, $type_offset:expr, $type_bits:expr =>
        { $($variant:ident, $type_value:expr => { $($field:ident, $get_field:ident, $set_field:ident, $index:expr, $offset:expr, $bits:expr, $shift:expr, $sign_ext: expr),* }),* }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
        #[repr(C)]
        pub struct $name {
            pub words: [usize; $total_words],
        }

        impl $name {
            $(
                #[inline]
                pub fn $variant($($field: usize),*) -> Self {
                    let mut value = $name::default();
                    $(
                        let mask = (((1u128 << $bits) - 1)) as usize;
                        value.words[$index] |= ((($field >> $shift) & mask) << $offset);
                    )*
                    value.words[$type_index] |= (($type_value & ((1usize << $type_bits) - 1)) << $type_offset);

                    value
                }

                $(
                    #[inline]
                    pub fn $get_field(&self) -> usize {
                        let mask = ((1u128 << $bits) - 1) as usize;
                        let mut ret = ((self.words[$index] >> $offset) & mask) << $shift;
                        if $sign_ext && (ret & (1usize << 47)) != 0 {
                            ret |= 0xffff000000000000;
                        }
                        ret
                    }

                    #[inline]
                    pub fn $set_field(&mut self, new_field: usize) {
                        let mask = ((1u128 << $bits) - 1) as usize;
                        self.words[$index] &= !(mask << $offset);
                        self.words[$index] |= (((new_field >> $shift) & mask) << $offset);
                    }
                )*
            )*

            #[inline]
            pub fn get_type(&self) -> usize {
                (self.words[$type_index] >> $type_offset) & ((1usize << $type_bits) - 1)
            }
        }
    };
}

#[macro_export]
macro_rules! mask {
    ($e:expr) => {
        {
             (1usize << $e) - 1usize
        }
    }
}

#[macro_export]
macro_rules! round_down {
    ($n:expr,$b:expr) => {{
        ((($n) >> ($b)) << ($b))
    }};
}

#[macro_export]
macro_rules! round_up {
    ($n:expr,$b:expr) => {{
        ((((($n) - 1usize) >> ($b)) + 1usize) << ($b))
    }};
}

#[macro_export]
macro_rules! is_aligned {
    ($n:expr,$b:expr) => {{
        $n & mask!($b) == 0
    }};
}

#[macro_export]
macro_rules! bit {
    ($e:expr) => {
        {
            1usize<<$e
        }
    }

        }

#[macro_export]
macro_rules! max_free_index {
    ($e:expr) => {{
        bit!($e - $crate::sel4::seL4_MinUntypedBits)
    }};
}

#[macro_export]
macro_rules! should_sign_extend {
    (true, $e:expr) => {
       <usize as $crate::common::PtrSignedExt>::sign_extend($e)
    };
    (false, $e:expr) => {
        $e
    }
}

#[macro_export]
macro_rules! should_offset {
    (0, $val:ident) => {};
    ($offset:tt, $val:ident) => {
        let $val = $val >> $offset;
    };
}

#[macro_export]
macro_rules! should_shift {
    (0, $val:ident) => {};
    ($shift:tt, $val:ident) => {
        let $val = $val << $shift;
    };
}


#[macro_export]
macro_rules! define_bitfield_type {
        ($name:ident, $total_words:expr => { $( $field:ident, $set_field:ident, $shift:tt, $field_range:expr, $offset:tt, $sign_ext:tt),* $(,)?}) => {

        #[repr(C)]
        #[derive(Debug,Clone, PartialEq, Eq)]
        pub struct $name(pub $crate::common::SeL4Bitfield<usize, $total_words>);

        impl Default for $name {
           fn default() -> Self {
               let bf = $crate::common::SeL4Bitfield::new([0usize; $total_words]);
               Self(bf)
            }
        }

        impl $name {
                #[inline]
                #[allow(unused_mut)]
                pub fn new($($field: usize),*) -> Self {
                    let mut ret =  $name::default();
                    $(
                        ret.$set_field($field);
                    )*
                    ret
                }

                $(
                    #[inline]
                    pub fn $field(&self) -> usize {
                        let ret = self.0.get_bits::<usize>($field_range);
                        $crate::should_shift!($shift, ret);
                        $crate::should_sign_extend!($sign_ext,self.0.get_bits::<usize>($field_range))
                    }

                    #[inline]
                    pub fn $set_field(&mut self, val: usize) {
                        $crate::should_offset!($offset, val);
                        self.0.set_bits::<usize>($field_range,val);
                    }
                )*
        }
    };
        ($name:ident, $total_words:expr,$type_range:expr =>
            { $($variant:ident, $type_value:expr => { $($field:ident, $set_field:ident, $offset:expr,$field_range:expr,$shift:expr, $sign_ext:tt),* $(,)?}), * $(,)?}) => {
            #[repr(C)]
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name(pub $crate::common::SeL4Bitfield<usize, $total_words>);

            impl Default for $name {
               fn default() -> Self {
                   let bf = $crate::common::SeL4Bitfield::new([0usize; $total_words]);
                   Self(bf)
                }
            }

            impl $name {
                $(
                    #[inline]
                    #[allow(unused_mut)]
                    pub fn $variant($($field: usize),*) -> Self {
                        let mut ret =  $name::default();
                        $(
                            ret.$set_field($field);
                        )*
                        ret.0.set_bits::<usize>($type_range, $type_value);
                        ret
                    }

                    $(
                        #[inline]
                        pub fn $field(&self) -> usize {
                            let ret = self.0.get_bits::<usize>($field_range);
                            $crate::should_shift!($shift, ret);
                            $crate::should_sign_extend!($sign_ext,ret)
                        }

                        #[inline]
                        pub fn $set_field(&mut self, val: usize) {
                            $crate::should_offset!($offset, val);
                            self.0.set_bits::<usize>($field_range,val);
                        }
                    )*
                )*

                #[inline]
                pub fn typ(&self) -> usize {
                    self.0.get_bits::<usize>($type_range)
                }
            }
        }
}

/// return current thread
#[macro_export]
macro_rules! current_thread {
    () => {
        $crate::thread::Thread::current()
    };
}



mod test {
    use ktest::ktest;

    use crate::common::{PtrSignedExt, SeL4Bitfield};

    #[ktest]
    fn bf_test() {
        let mut bf = SeL4Bitfield::new([0usize; 2]);
        bf.set_bits::<usize>(0..64, 0xffff_ffff_ffff_ffff);
        bf.set_bits::<usize>(64..126, 0x8000_0000_0000);
        bf.set_bits::<usize>(126..127, 0x1);
        bf.set_bits::<usize>(127..128, 0x1);
        let prev = bf.get_bits::<usize>(0..64);
        let next = bf.get_bits::<usize>(64..126).sign_extend();
        let revocable = bf.get_bits::<usize>(126..127);
        let first_badged = bf.get_bits::<usize>(127..128);
        assert_eq!(prev, 0xffff_ffff_ffff_ffff);
        assert_eq!(next, 0xffff_8000_0000_0000);
        assert_eq!(revocable, 0x1);
        assert_eq!(first_badged, 0x1);
    }

    #[ktest]
    fn should_sign_extend_test() {
        let mut bf = SeL4Bitfield::new([0usize; 2]);
        bf.set_bits::<usize>(0..64, 0x8000_0000_0000);
        let ret = should_sign_extend!(true, bf.get_bits::<usize>(0..64));
        assert_eq!(ret, 0xffff_8000_0000_0000);
    }
}
