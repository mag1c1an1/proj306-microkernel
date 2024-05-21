pub mod cspace {
    use crate::{
        plus_define_bitfield,
        sel4::{
            seL4_EndpointBits, seL4_MsgMaxExtraCaps, seL4_NotificationBits,
            seL4_ReplyBits, seL4_SlotBits,
            utils::{convert_to_mut_type_ref, pageBitsForSize},
            wordBits, PT_SIZE_BITS,
        },
        MASK,
    };
    use super::{exception_t, vspace::pptr_t};
    pub mod cte {
        use core::{
            intrinsics::{likely, unlikely},
            ptr,
        };
        use crate::{
            sel4::{
                cspace::is_cap_revocable, exception_t,
                utils::{
                    convert_to_mut_type_ref, convert_to_option_mut_type_ref,
                    convert_to_type_ref, MAX_FREE_INDEX,
                },
                wordRadix,
            },
            MASK,
        };
        use super::{
            cap_t, finaliseCap, finaliseSlot_ret, mdb::mdb_node_t, post_cap_deletion,
            preemptionPoint, same_object_as, same_region_as, zombie::capCyclicZombie,
            CapTag,
        };
        #[repr(C)]
        pub struct deriveCap_ret {
            pub status: exception_t,
            pub cap: cap_t,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for deriveCap_ret {
            #[inline]
            fn clone(&self) -> deriveCap_ret {
                let _: ::core::clone::AssertParamIsClone<exception_t>;
                let _: ::core::clone::AssertParamIsClone<cap_t>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for deriveCap_ret {}
        /// 由cap_t和 mdb_node 组成，是CSpace的基本组成单元
        #[repr(C)]
        pub struct cte_t {
            pub cap: cap_t,
            pub cteMDBNode: mdb_node_t,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for cte_t {
            #[inline]
            fn clone(&self) -> cte_t {
                let _: ::core::clone::AssertParamIsClone<cap_t>;
                let _: ::core::clone::AssertParamIsClone<mdb_node_t>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for cte_t {}
        #[automatically_derived]
        impl ::core::default::Default for cte_t {
            #[inline]
            fn default() -> cte_t {
                cte_t {
                    cap: ::core::default::Default::default(),
                    cteMDBNode: ::core::default::Default::default(),
                }
            }
        }
        impl cte_t {
            pub fn get_ptr(&self) -> usize {
                self as *const cte_t as usize
            }
            pub fn get_offset_slot(&mut self, index: usize) -> &'static mut Self {
                convert_to_mut_type_ref::<
                    Self,
                >(self.get_ptr() + core::mem::size_of::<cte_t>() * index)
            }
            pub fn derive_cap(&mut self, cap: &cap_t) -> deriveCap_ret {
                if cap.isArchCap() {
                    return self.arch_derive_cap(cap);
                }
                let mut ret = deriveCap_ret {
                    status: exception_t::EXCEPTION_NONE,
                    cap: cap_t::default(),
                };
                match cap.get_cap_type() {
                    CapTag::CapZombieCap => {
                        ret.cap = cap_t::new_null_cap();
                    }
                    CapTag::CapUntypedCap => {
                        ret.status = self.ensure_no_children();
                        if ret.status != exception_t::EXCEPTION_NONE {
                            ret.cap = cap_t::new_null_cap();
                        } else {
                            ret.cap = cap.clone();
                        }
                    }
                    CapTag::CapReplyCap => {
                        ret.cap = cap_t::new_null_cap();
                    }
                    CapTag::CapIrqControlCap => {
                        ret.cap = cap_t::new_null_cap();
                    }
                    _ => {
                        ret.cap = cap.clone();
                    }
                }
                ret
            }
            fn arch_derive_cap(&mut self, cap: &cap_t) -> deriveCap_ret {
                let mut ret = deriveCap_ret {
                    status: exception_t::EXCEPTION_NONE,
                    cap: cap_t::default(),
                };
                match cap.get_cap_type() {
                    CapTag::CapPageTableCap => {
                        if cap.get_pt_is_mapped() != 0 {
                            ret.cap = cap.clone();
                            ret.status = exception_t::EXCEPTION_NONE;
                        } else {
                            ret.cap = cap_t::new_null_cap();
                            ret.status = exception_t::EXCEPTION_SYSCALL_ERROR;
                        }
                    }
                    CapTag::CapFrameCap => {
                        let mut newCap = cap.clone();
                        newCap.set_frame_mapped_address(0);
                        newCap.set_frame_mapped_asid(0);
                        ret.cap = newCap;
                    }
                    CapTag::CapASIDControlCap | CapTag::CapASIDPoolCap => {
                        ret.cap = cap.clone();
                    }
                    _ => {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    " Invalid arch cap type : {0}",
                                    cap.get_cap_type() as usize,
                                ),
                            );
                        };
                    }
                }
                ret
            }
            pub fn ensure_no_children(&self) -> exception_t {
                if self.cteMDBNode.get_next() != 0 {
                    let next = convert_to_type_ref::<cte_t>(self.cteMDBNode.get_next());
                    if self.is_mdb_parent_of(next) {
                        return exception_t::EXCEPTION_SYSCALL_ERROR;
                    }
                }
                return exception_t::EXCEPTION_NONE;
            }
            fn is_mdb_parent_of(&self, next: &Self) -> bool {
                if !(self.cteMDBNode.get_revocable() != 0) {
                    return false;
                }
                if !same_region_as(&self.cap, &next.cap) {
                    return false;
                }
                match self.cap.get_cap_type() {
                    CapTag::CapEndpointCap => {
                        match (&next.cap.get_cap_type(), &CapTag::CapEndpointCap) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                        let badge = self.cap.get_ep_badge();
                        if badge == 0 {
                            return true;
                        }
                        return badge == next.cap.get_ep_badge()
                            && !(next.cteMDBNode.get_first_badged() != 0);
                    }
                    CapTag::CapNotificationCap => {
                        match (&next.cap.get_cap_type(), &CapTag::CapNotificationCap) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                        let badge = self.cap.get_nf_badge();
                        if badge == 0 {
                            return true;
                        }
                        return badge == next.cap.get_nf_badge()
                            && !(next.cteMDBNode.get_first_badged() != 0);
                    }
                    _ => true,
                }
            }
            pub fn is_final_cap(&self) -> bool {
                let mdb = &self.cteMDBNode;
                let prev_is_same_obj = if mdb.get_prev() == 0 {
                    false
                } else {
                    let prev = convert_to_type_ref::<cte_t>(mdb.get_prev());
                    same_object_as(&prev.cap, &self.cap)
                };
                if prev_is_same_obj {
                    false
                } else {
                    if mdb.get_next() == 0 {
                        true
                    } else {
                        let next = convert_to_type_ref::<cte_t>(mdb.get_next());
                        return !same_object_as(&self.cap, &next.cap);
                    }
                }
            }
            pub fn is_long_running_delete(&self) -> bool {
                if self.cap.get_cap_type() == CapTag::CapNullCap || !self.is_final_cap()
                {
                    return false;
                }
                match self.cap.get_cap_type() {
                    CapTag::CapThreadCap | CapTag::CapZombieCap | CapTag::CapCNodeCap => {
                        true
                    }
                    _ => false,
                }
            }
            fn finalise(&mut self, immediate: bool) -> finaliseSlot_ret {
                let mut ret = finaliseSlot_ret::default();
                while self.cap.get_cap_type() != CapTag::CapNullCap {
                    let fc_ret = finaliseCap(&self.cap, self.is_final_cap(), false);
                    if cap_removable(&fc_ret.remainder, self) {
                        ret.status = exception_t::EXCEPTION_NONE;
                        ret.success = true;
                        ret.cleanupInfo = fc_ret.cleanupInfo;
                        return ret;
                    }
                    self.cap = fc_ret.remainder;
                    if !immediate && capCyclicZombie(&fc_ret.remainder, self) {
                        ret.status = exception_t::EXCEPTION_NONE;
                        ret.success = false;
                        ret.cleanupInfo = fc_ret.cleanupInfo;
                        return ret;
                    }
                    let status = self.reduce_zombie(immediate);
                    if exception_t::EXCEPTION_NONE != status {
                        ret.status = status;
                        ret.success = false;
                        ret.cleanupInfo = cap_t::new_null_cap();
                        return ret;
                    }
                    let status = preemptionPoint();
                    if exception_t::EXCEPTION_NONE != status {
                        ret.status = status;
                        ret.success = false;
                        ret.cleanupInfo = cap_t::new_null_cap();
                        return ret;
                    }
                }
                ret
            }
            pub fn delete_all(&mut self, exposed: bool) -> exception_t {
                let fs_ret = self.finalise(exposed);
                if fs_ret.status != exception_t::EXCEPTION_NONE {
                    return fs_ret.status;
                }
                if exposed || fs_ret.success {
                    self.set_empty(&fs_ret.cleanupInfo);
                }
                return exception_t::EXCEPTION_NONE;
            }
            pub fn delete_one(&mut self) {
                if self.cap.get_cap_type() != CapTag::CapNullCap {
                    let fc_ret = finaliseCap(&self.cap, self.is_final_cap(), true);
                    if !(cap_removable(&fc_ret.remainder, self)
                        && fc_ret.cleanupInfo.get_cap_type() == CapTag::CapNullCap)
                    {
                        ::core::panicking::panic(
                            "assertion failed: cap_removable(&fc_ret.remainder, self) &&\n    fc_ret.cleanupInfo.get_cap_type() == CapTag::CapNullCap",
                        )
                    }
                    self.set_empty(&cap_t::new_null_cap());
                }
            }
            fn set_empty(&mut self, cleanup_info: &cap_t) {
                if self.cap.get_cap_type() != CapTag::CapNullCap {
                    let mdb_node = self.cteMDBNode;
                    let prev_addr = mdb_node.get_prev();
                    let next_addr = mdb_node.get_next();
                    if prev_addr != 0 {
                        let prev_node = convert_to_mut_type_ref::<cte_t>(prev_addr);
                        prev_node.cteMDBNode.set_next(next_addr);
                    }
                    if next_addr != 0 {
                        let next_node = convert_to_mut_type_ref::<cte_t>(next_addr);
                        next_node.cteMDBNode.set_prev(prev_addr);
                        let first_badged = ((next_node.cteMDBNode.get_first_badged()
                            != 0) || (mdb_node.get_first_badged() != 0)) as usize;
                        next_node.cteMDBNode.set_first_badged(first_badged);
                    }
                    self.cap = cap_t::new_null_cap();
                    self.cteMDBNode = mdb_node_t::default();
                    post_cap_deletion(cleanup_info);
                }
            }
            fn reduce_zombie(&mut self, immediate: bool) -> exception_t {
                match (&self.cap.get_cap_type(), &CapTag::CapZombieCap) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                let self_ptr = self as *mut cte_t as usize;
                let ptr = self.cap.get_zombie_ptr();
                let n = self.cap.get_zombie_number();
                let zombie_type = self.cap.get_zombie_type();
                if !(n > 0) {
                    ::core::panicking::panic("assertion failed: n > 0")
                }
                if immediate {
                    let end_slot = unsafe { &mut *((ptr as *mut cte_t).add(n - 1)) };
                    let status = end_slot.delete_all(false);
                    if status != exception_t::EXCEPTION_NONE {
                        return status;
                    }
                    match self.cap.get_cap_type() {
                        CapTag::CapNullCap => {
                            return exception_t::EXCEPTION_NONE;
                        }
                        CapTag::CapZombieCap => {
                            let ptr2 = self.cap.get_zombie_ptr();
                            if ptr == ptr2 && self.cap.get_zombie_number() == n
                                && self.cap.get_zombie_type() == zombie_type
                            {
                                match (&end_slot.cap.get_cap_type(), &CapTag::CapNullCap) {
                                    (left_val, right_val) => {
                                        if !(*left_val == *right_val) {
                                            let kind = ::core::panicking::AssertKind::Eq;
                                            ::core::panicking::assert_failed(
                                                kind,
                                                &*left_val,
                                                &*right_val,
                                                ::core::option::Option::None,
                                            );
                                        }
                                    }
                                };
                                self.cap.set_zombie_number(n - 1);
                            } else {
                                if !(ptr2 == self_ptr && ptr != self_ptr) {
                                    ::core::panicking::panic(
                                        "assertion failed: ptr2 == self_ptr && ptr != self_ptr",
                                    )
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("Expected recursion to result in Zombie."),
                            );
                        }
                    }
                } else {
                    match (&ptr, &self_ptr) {
                        (left_val, right_val) => {
                            if *left_val == *right_val {
                                let kind = ::core::panicking::AssertKind::Ne;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    let next_slot = convert_to_mut_type_ref::<cte_t>(ptr);
                    let cap1 = next_slot.cap;
                    let cap2 = self.cap;
                    cte_swap(&cap1, next_slot, &cap2, self);
                }
                exception_t::EXCEPTION_NONE
            }
            #[inline]
            fn get_volatile_value(&self) -> usize {
                unsafe {
                    let raw_value = ptr::read_volatile(
                        (self.get_ptr() + 24) as *const usize,
                    );
                    let mut value = ((raw_value >> 2) & { (1usize << 37) - 1usize })
                        << 2;
                    if (value & (1usize << 38)) != 0 {
                        value |= 0xffffff8000000000;
                    }
                    value
                }
            }
            pub fn revoke(&mut self) -> exception_t {
                while let Some(cte) = convert_to_option_mut_type_ref::<
                    cte_t,
                >(self.get_volatile_value()) {
                    if !self.is_mdb_parent_of(cte) {
                        break;
                    }
                    let mut status = cte.delete_all(true);
                    if status != exception_t::EXCEPTION_NONE {
                        return status;
                    }
                    status = preemptionPoint();
                    if status != exception_t::EXCEPTION_NONE {
                        return status;
                    }
                }
                return exception_t::EXCEPTION_NONE;
            }
        }
        /// 将一个cap插入slot中并维护能力派生树
        ///
        /// 将一个new_cap插入到dest slot中并作为src slot的派生子节点插入派生树中
        pub fn cte_insert(new_cap: &cap_t, src_slot: &mut cte_t, dest_slot: &mut cte_t) {
            let srcMDB = &mut src_slot.cteMDBNode;
            let srcCap = &(src_slot.cap.clone());
            let mut newMDB = srcMDB.clone();
            let newCapIsRevocable = is_cap_revocable(new_cap, srcCap);
            newMDB.set_prev(src_slot as *const cte_t as usize);
            newMDB.set_revocable(newCapIsRevocable as usize);
            newMDB.set_first_badged(newCapIsRevocable as usize);
            match (&dest_slot.cap.get_cap_type(), &CapTag::CapNullCap) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            if !(dest_slot.cteMDBNode.get_next() == 0
                && dest_slot.cteMDBNode.get_prev() == 0)
            {
                ::core::panicking::panic(
                    "assertion failed: dest_slot.cteMDBNode.get_next() == 0 && dest_slot.cteMDBNode.get_prev() == 0",
                )
            }
            setUntypedCapAsFull(srcCap, new_cap, src_slot);
            (*dest_slot).cap = new_cap.clone();
            (*dest_slot).cteMDBNode = newMDB;
            src_slot.cteMDBNode.set_next(dest_slot as *const cte_t as usize);
            if newMDB.get_next() != 0 {
                let cte_ref = convert_to_mut_type_ref::<cte_t>(newMDB.get_next());
                cte_ref.cteMDBNode.set_prev(dest_slot as *const cte_t as usize);
            }
        }
        pub fn insert_new_cap(parent: &mut cte_t, slot: &mut cte_t, cap: &cap_t) {
            let next = parent.cteMDBNode.get_next();
            slot.cap = cap.clone();
            slot.cteMDBNode = mdb_node_t::new(
                next as usize,
                1usize,
                1usize,
                parent as *const cte_t as usize,
            );
            if next != 0 {
                let next_ref = convert_to_mut_type_ref::<cte_t>(next);
                next_ref.cteMDBNode.set_prev(slot as *const cte_t as usize);
            }
            parent.cteMDBNode.set_next(slot as *const cte_t as usize);
        }
        /// 将一个cap插入slot中并删除原节点
        ///
        /// 将一个new_cap插入到dest slot中并作为替代src slot在派生树中的位置
        pub fn cte_move(new_cap: &cap_t, src_slot: &mut cte_t, dest_slot: &mut cte_t) {
            match (&dest_slot.cap.get_cap_type(), &CapTag::CapNullCap) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            if !(dest_slot.cteMDBNode.get_next() == 0
                && dest_slot.cteMDBNode.get_prev() == 0)
            {
                ::core::panicking::panic(
                    "assertion failed: dest_slot.cteMDBNode.get_next() == 0 && dest_slot.cteMDBNode.get_prev() == 0",
                )
            }
            let mdb = src_slot.cteMDBNode;
            dest_slot.cap = new_cap.clone();
            src_slot.cap = cap_t::new_null_cap();
            dest_slot.cteMDBNode = mdb;
            src_slot.cteMDBNode = mdb_node_t::new(0, 0, 0, 0);
            let prev_ptr = mdb.get_prev();
            if prev_ptr != 0 {
                let prev_ref = convert_to_mut_type_ref::<cte_t>(prev_ptr);
                prev_ref.cteMDBNode.set_next(dest_slot as *const cte_t as usize);
            }
            let next_ptr = mdb.get_next();
            if next_ptr != 0 {
                let next_ref = convert_to_mut_type_ref::<cte_t>(next_ptr);
                next_ref.cteMDBNode.set_prev(dest_slot as *const cte_t as usize);
            }
        }
        /// 交换两个slot，并将新的cap数据填入
        pub fn cte_swap(
            cap1: &cap_t,
            slot1: &mut cte_t,
            cap2: &cap_t,
            slot2: &mut cte_t,
        ) {
            let mdb1 = slot1.cteMDBNode;
            let mdb2 = slot2.cteMDBNode;
            {
                let prev_ptr = mdb1.get_prev();
                if prev_ptr != 0 {
                    convert_to_mut_type_ref::<cte_t>(prev_ptr)
                        .cteMDBNode
                        .set_next(slot2 as *const cte_t as usize);
                }
                let next_ptr = mdb1.get_next();
                if next_ptr != 0 {
                    convert_to_mut_type_ref::<cte_t>(next_ptr)
                        .cteMDBNode
                        .set_prev(slot2 as *const cte_t as usize);
                }
            }
            slot1.cap = cap2.clone();
            slot2.cap = cap1.clone();
            slot1.cteMDBNode = mdb2;
            slot2.cteMDBNode = mdb1;
            {
                let prev_ptr = mdb2.get_prev();
                if prev_ptr != 0 {
                    convert_to_mut_type_ref::<cte_t>(prev_ptr)
                        .cteMDBNode
                        .set_next(slot1 as *const cte_t as usize);
                }
                let next_ptr = mdb2.get_next();
                if next_ptr != 0 {
                    convert_to_mut_type_ref::<cte_t>(next_ptr)
                        .cteMDBNode
                        .set_prev(slot1 as *const cte_t as usize);
                }
            }
        }
        #[inline]
        fn cap_removable(cap: &cap_t, slot: *mut cte_t) -> bool {
            match cap.get_cap_type() {
                CapTag::CapNullCap => {
                    return true;
                }
                CapTag::CapZombieCap => {
                    let n = cap.get_zombie_number();
                    let ptr = cap.get_zombie_ptr();
                    let z_slot = ptr as *mut cte_t;
                    return n == 0 || (n == 1 && slot == z_slot);
                }
                _ => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Invalid cap type , finaliseCap should only return Zombie or NullCap",
                            ),
                        );
                    };
                }
            }
        }
        fn setUntypedCapAsFull(srcCap: &cap_t, newCap: &cap_t, srcSlot: &mut cte_t) {
            if srcCap.get_cap_type() == CapTag::CapUntypedCap
                && newCap.get_cap_type() == CapTag::CapUntypedCap
            {
                match (&srcSlot.cap.get_cap_type(), &CapTag::CapUntypedCap) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                if srcCap.get_untyped_ptr() == newCap.get_untyped_ptr()
                    && srcCap.get_untyped_block_size() == newCap.get_untyped_block_size()
                {
                    srcSlot
                        .cap
                        .set_untyped_free_index(
                            MAX_FREE_INDEX(srcCap.get_untyped_block_size()),
                        );
                }
            }
        }
        #[repr(C)]
        pub struct resolveAddressBits_ret_t {
            pub status: exception_t,
            pub slot: *mut cte_t,
            pub bitsRemaining: usize,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for resolveAddressBits_ret_t {
            #[inline]
            fn clone(&self) -> resolveAddressBits_ret_t {
                let _: ::core::clone::AssertParamIsClone<exception_t>;
                let _: ::core::clone::AssertParamIsClone<*mut cte_t>;
                let _: ::core::clone::AssertParamIsClone<usize>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for resolveAddressBits_ret_t {}
        #[automatically_derived]
        impl ::core::fmt::Debug for resolveAddressBits_ret_t {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "resolveAddressBits_ret_t",
                    "status",
                    &self.status,
                    "slot",
                    &self.slot,
                    "bitsRemaining",
                    &&self.bitsRemaining,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for resolveAddressBits_ret_t {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for resolveAddressBits_ret_t {
            #[inline]
            fn eq(&self, other: &resolveAddressBits_ret_t) -> bool {
                self.status == other.status && self.slot == other.slot
                    && self.bitsRemaining == other.bitsRemaining
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for resolveAddressBits_ret_t {}
        #[automatically_derived]
        impl ::core::cmp::Eq for resolveAddressBits_ret_t {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<exception_t>;
                let _: ::core::cmp::AssertParamIsEq<*mut cte_t>;
                let _: ::core::cmp::AssertParamIsEq<usize>;
            }
        }
        impl Default for resolveAddressBits_ret_t {
            #[inline]
            fn default() -> Self {
                resolveAddressBits_ret_t {
                    status: exception_t::EXCEPTION_NONE,
                    slot: 0 as *mut cte_t,
                    bitsRemaining: 0,
                }
            }
        }
        /// 从cspace寻址特定的slot
        ///
        /// 从给定的cnode、cap index、和depth中找到对应cap的slot，成功则返回slot指针，失败返回找到的最深的cnode
        #[allow(unreachable_code)]
        pub fn resolve_address_bits(
            node_cap: &cap_t,
            cap_ptr: usize,
            _n_bits: usize,
        ) -> resolveAddressBits_ret_t {
            let mut ret = resolveAddressBits_ret_t::default();
            let mut n_bits = _n_bits;
            ret.bitsRemaining = n_bits;
            let mut nodeCap = node_cap.clone();
            if unlikely(nodeCap.get_cap_type() != CapTag::CapCNodeCap) {
                ret.status = exception_t::EXCEPTION_LOOKUP_FAULT;
                return ret;
            }
            loop {
                let radixBits = nodeCap.get_cnode_radix();
                let guardBits = nodeCap.get_cnode_guard_size();
                let levelBits = radixBits + guardBits;
                match (&levelBits, &0) {
                    (left_val, right_val) => {
                        if *left_val == *right_val {
                            let kind = ::core::panicking::AssertKind::Ne;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                let capGuard = nodeCap.get_cnode_guard();
                let guard = (cap_ptr
                    >> ((n_bits - guardBits) & { (1usize << wordRadix) - 1usize }))
                    & { (1usize << guardBits) - 1usize };
                if unlikely(guardBits > n_bits || guard != capGuard) {
                    ret.status = exception_t::EXCEPTION_LOOKUP_FAULT;
                    return ret;
                }
                if unlikely(levelBits > n_bits) {
                    ret.status = exception_t::EXCEPTION_LOOKUP_FAULT;
                    return ret;
                }
                let offset = (cap_ptr >> (n_bits - levelBits))
                    & { (1usize << radixBits) - 1usize };
                let slot = unsafe {
                    (nodeCap.get_cnode_ptr() as *mut cte_t).add(offset)
                };
                if likely(n_bits == levelBits) {
                    ret.slot = slot;
                    ret.bitsRemaining = 0;
                    return ret;
                }
                n_bits -= levelBits;
                nodeCap = unsafe { (*slot).cap.clone() };
                if unlikely(nodeCap.get_cap_type() != CapTag::CapCNodeCap) {
                    ret.slot = slot;
                    ret.bitsRemaining = n_bits;
                    return ret;
                }
            }
            {
                ::core::panicking::panic_fmt(format_args!("UNREACHABLE"));
            };
        }
    }
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
                mdbNext: usize,
                mdbRevocable: usize,
                mdbFirstBadged: usize,
                mdbPrev: usize,
            ) -> Self {
                let mut value = mdb_node_t::default();
                let mask = (((1u128 << 37) - 1)) as usize;
                value.words[1] |= (((mdbNext >> 2) & mask) << 2);
                let mask = (((1u128 << 1) - 1)) as usize;
                value.words[1] |= (((mdbRevocable >> 0) & mask) << 1);
                let mask = (((1u128 << 0) - 1)) as usize;
                value.words[1] |= (((mdbFirstBadged >> 0) & mask) << 0);
                let mask = (((1u128 << 64) - 1)) as usize;
                value.words[0] |= (((mdbPrev >> 0) & mask) << 0);
                value.words[0] |= ((0 & ((1usize << 0) - 1)) << 0);
                value
            }
            #[inline]
            pub fn get_next(&self) -> usize {
                let mask = ((1u128 << 37) - 1) as usize;
                let mut ret = ((self.words[1] >> 2) & mask) << 2;
                if true && (ret & (1usize << 47)) != 0 {
                    ret |= 0xffff000000000000;
                }
                ret
            }
            #[inline]
            pub fn set_next(&mut self, new_field: usize) {
                let mask = ((1u128 << 37) - 1) as usize;
                self.words[1] &= !(mask << 2);
                self.words[1] |= (((new_field >> 2) & mask) << 2);
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
                let mask = ((1u128 << 0) - 1) as usize;
                let mut ret = ((self.words[1] >> 0) & mask) << 0;
                if false && (ret & (1usize << 47)) != 0 {
                    ret |= 0xffff000000000000;
                }
                ret
            }
            #[inline]
            pub fn set_first_badged(&mut self, new_field: usize) {
                let mask = ((1u128 << 0) - 1) as usize;
                self.words[1] &= !(mask << 0);
                self.words[1] |= (((new_field >> 0) & mask) << 0);
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
            pub fn get_type(&self) -> usize {
                (self.words[0] >> 0) & ((1usize << 0) - 1)
            }
        }
    }
    pub mod zombie {
        use crate::{sel4::wordRadix, MASK};
        use super::{cap_t, cte::cte_t, CapTag};
        pub const ZombieType_ZombieTCB: usize = 1usize << wordRadix;
        pub const TCB_CNODE_RADIX: usize = 4;
        /// zombie cap相关字段和方法
        impl cap_t {
            #[inline]
            pub fn get_zombie_bit(&self) -> usize {
                let _type = self.get_zombie_type();
                if _type == ZombieType_ZombieTCB {
                    return TCB_CNODE_RADIX;
                }
                return ZombieType_ZombieCNode(_type);
            }
            #[inline]
            pub fn get_zombie_ptr(&self) -> usize {
                let radix = self.get_zombie_bit();
                return self.get_zombie_id() & !{ (1usize << radix + 1) - 1usize };
            }
            #[inline]
            pub fn get_zombie_number(&self) -> usize {
                let radix = self.get_zombie_bit();
                return self.get_zombie_id() & { (1usize << radix + 1) - 1usize };
            }
            #[inline]
            pub fn set_zombie_number(&mut self, n: usize) {
                let radix = self.get_zombie_bit();
                let ptr = self.get_zombie_id() & !{ (1usize << radix + 1) - 1usize };
                self.set_zombie_id(ptr | (n & { (1usize << radix + 1) - 1usize }));
            }
        }
        #[inline]
        pub fn Zombie_new(number: usize, _type: usize, ptr: usize) -> cap_t {
            let mask: usize;
            if _type == ZombieType_ZombieTCB {
                mask = { (1usize << TCB_CNODE_RADIX + 1) - 1usize };
            } else {
                mask = { (1usize << _type + 1) - 1usize };
            }
            return cap_t::new_zombie_cap((ptr & !mask) | (number & mask), _type);
        }
        pub fn ZombieType_ZombieCNode(n: usize) -> usize {
            return n & { (1usize << wordRadix) - 1usize };
        }
        #[inline]
        #[no_mangle]
        pub fn capCyclicZombie(cap: &cap_t, slot: *mut cte_t) -> bool {
            let ptr = cap.get_zombie_ptr() as *mut cte_t;
            (cap.get_cap_type() == CapTag::CapZombieCap) && (ptr == slot)
        }
    }
    #[repr(C)]
    pub struct deriveCap_ret {
        pub status: exception_t,
        pub cap: cap_t,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for deriveCap_ret {
        #[inline]
        fn clone(&self) -> deriveCap_ret {
            let _: ::core::clone::AssertParamIsClone<exception_t>;
            let _: ::core::clone::AssertParamIsClone<cap_t>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for deriveCap_ret {}
    #[repr(C)]
    pub struct extra_caps_t {
        pub excaprefs: [pptr_t; seL4_MsgMaxExtraCaps],
    }
    #[automatically_derived]
    impl ::core::clone::Clone for extra_caps_t {
        #[inline]
        fn clone(&self) -> extra_caps_t {
            let _: ::core::clone::AssertParamIsClone<[pptr_t; seL4_MsgMaxExtraCaps]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for extra_caps_t {}
    #[automatically_derived]
    impl ::core::fmt::Debug for extra_caps_t {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "extra_caps_t",
                "excaprefs",
                &&self.excaprefs,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for extra_caps_t {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for extra_caps_t {
        #[inline]
        fn eq(&self, other: &extra_caps_t) -> bool {
            self.excaprefs == other.excaprefs
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for extra_caps_t {}
    #[automatically_derived]
    impl ::core::cmp::Eq for extra_caps_t {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<[pptr_t; seL4_MsgMaxExtraCaps]>;
        }
    }
    #[repr(C)]
    struct CNodeCapData {
        pub words: [usize; 1],
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CNodeCapData {
        #[inline]
        fn clone(&self) -> CNodeCapData {
            let _: ::core::clone::AssertParamIsClone<[usize; 1]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CNodeCapData {}
    #[automatically_derived]
    impl ::core::fmt::Debug for CNodeCapData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "CNodeCapData",
                "words",
                &&self.words,
            )
        }
    }
    impl CNodeCapData {
        #[inline]
        pub fn new(data: usize) -> Self {
            CNodeCapData { words: [data] }
        }
        #[inline]
        pub fn get_guard(&self) -> usize {
            (self.words[0] & 0xffffffffffffffc0usize) >> 6
        }
        #[inline]
        pub fn get_guard_size(&self) -> usize {
            self.words[0] & 0x3fusize
        }
    }
    /// Cap 在内核态中的种类枚举
    pub enum CapTag {
        CapNullCap = 0,
        CapUntypedCap = 2,
        CapEndpointCap = 4,
        CapNotificationCap = 6,
        CapReplyCap = 8,
        CapCNodeCap = 10,
        CapThreadCap = 12,
        CapIrqControlCap = 14,
        CapIrqHandlerCap = 16,
        CapZombieCap = 18,
        CapDomainCap = 20,
        CapFrameCap = 1,
        CapPageTableCap = 3,
        CapPageDirectoryCap = 5,
        CapPDPTCap = 7,
        CapPML4Cap = 9,
        CapASIDControlCap = 11,
        CapASIDPoolCap = 13,
        CapIOPortCap = 19,
        CapIOPortControlCap = 31,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for CapTag {}
    #[automatically_derived]
    impl ::core::cmp::Eq for CapTag {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CapTag {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CapTag {
        #[inline]
        fn eq(&self, other: &CapTag) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CapTag {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    CapTag::CapNullCap => "CapNullCap",
                    CapTag::CapUntypedCap => "CapUntypedCap",
                    CapTag::CapEndpointCap => "CapEndpointCap",
                    CapTag::CapNotificationCap => "CapNotificationCap",
                    CapTag::CapReplyCap => "CapReplyCap",
                    CapTag::CapCNodeCap => "CapCNodeCap",
                    CapTag::CapThreadCap => "CapThreadCap",
                    CapTag::CapIrqControlCap => "CapIrqControlCap",
                    CapTag::CapIrqHandlerCap => "CapIrqHandlerCap",
                    CapTag::CapZombieCap => "CapZombieCap",
                    CapTag::CapDomainCap => "CapDomainCap",
                    CapTag::CapFrameCap => "CapFrameCap",
                    CapTag::CapPageTableCap => "CapPageTableCap",
                    CapTag::CapPageDirectoryCap => "CapPageDirectoryCap",
                    CapTag::CapPDPTCap => "CapPDPTCap",
                    CapTag::CapPML4Cap => "CapPML4Cap",
                    CapTag::CapASIDControlCap => "CapASIDControlCap",
                    CapTag::CapASIDPoolCap => "CapASIDPoolCap",
                    CapTag::CapIOPortCap => "CapIOPortCap",
                    CapTag::CapIOPortControlCap => "CapIOPortControlCap",
                },
            )
        }
    }
    #[repr(C)]
    pub struct cap_t {
        pub words: [usize; 2],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for cap_t {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "cap_t",
                "words",
                &&self.words,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for cap_t {}
    #[automatically_derived]
    impl ::core::clone::Clone for cap_t {
        #[inline]
        fn clone(&self) -> cap_t {
            let _: ::core::clone::AssertParamIsClone<[usize; 2]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for cap_t {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for cap_t {
        #[inline]
        fn eq(&self, other: &cap_t) -> bool {
            self.words == other.words
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for cap_t {}
    #[automatically_derived]
    impl ::core::cmp::Eq for cap_t {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<[usize; 2]>;
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for cap_t {
        #[inline]
        fn default() -> cap_t {
            cap_t {
                words: ::core::default::Default::default(),
            }
        }
    }
    impl cap_t {
        #[inline]
        pub fn new_null_cap() -> Self {
            let mut value = cap_t::default();
            value.words[0]
                |= ((CapTag::CapNullCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn new_untyped_cap(
            capFreeIndex: usize,
            capIsDevice: usize,
            capBlockSize: usize,
            capPtr: usize,
        ) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 48) - 1)) as usize;
            value.words[1] |= (((capFreeIndex >> 0) & mask) << 16);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[1] |= (((capIsDevice >> 0) & mask) << 6);
            let mask = (((1u128 << 6) - 1)) as usize;
            value.words[1] |= (((capBlockSize >> 0) & mask) << 0);
            let mask = (((1u128 << 48) - 1)) as usize;
            value.words[0] |= (((capPtr >> 0) & mask) << 0);
            value.words[0]
                |= ((CapTag::CapUntypedCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_untyped_free_index(&self) -> usize {
            let mask = ((1u128 << 48) - 1) as usize;
            let mut ret = ((self.words[1] >> 16) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_untyped_free_index(&mut self, new_field: usize) {
            let mask = ((1u128 << 48) - 1) as usize;
            self.words[1] &= !(mask << 16);
            self.words[1] |= (((new_field >> 0) & mask) << 16);
        }
        #[inline]
        pub fn get_untyped_is_device(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[1] >> 6) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_untyped_is_device(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[1] &= !(mask << 6);
            self.words[1] |= (((new_field >> 0) & mask) << 6);
        }
        #[inline]
        pub fn get_untyped_block_size(&self) -> usize {
            let mask = ((1u128 << 6) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_untyped_block_size(&mut self, new_field: usize) {
            let mask = ((1u128 << 6) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_untyped_ptr(&self) -> usize {
            let mask = ((1u128 << 48) - 1) as usize;
            let mut ret = ((self.words[0] >> 0) & mask) << 0;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_untyped_ptr(&mut self, new_field: usize) {
            let mask = ((1u128 << 48) - 1) as usize;
            self.words[0] &= !(mask << 0);
            self.words[0] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn new_endpoint_cap(
            capEPBadge: usize,
            capCanGrantReply: usize,
            capCanGrant: usize,
            capCanSend: usize,
            capCanReceive: usize,
            capEPPtr: usize,
        ) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 64) - 1)) as usize;
            value.words[1] |= (((capEPBadge >> 0) & mask) << 0);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capCanGrantReply >> 0) & mask) << 58);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capCanGrant >> 0) & mask) << 57);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capCanSend >> 0) & mask) << 55);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capCanReceive >> 0) & mask) << 56);
            let mask = (((1u128 << 48) - 1)) as usize;
            value.words[0] |= (((capEPPtr >> 0) & mask) << 0);
            value.words[0]
                |= ((CapTag::CapEndpointCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_ep_badge(&self) -> usize {
            let mask = ((1u128 << 64) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_ep_badge(&mut self, new_field: usize) {
            let mask = ((1u128 << 64) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_ep_can_grant_reply(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 58) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_ep_can_grant_reply(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 58);
            self.words[0] |= (((new_field >> 0) & mask) << 58);
        }
        #[inline]
        pub fn get_ep_can_grant(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 57) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_ep_can_grant(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 57);
            self.words[0] |= (((new_field >> 0) & mask) << 57);
        }
        #[inline]
        pub fn get_ep_can_send(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 55) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_ep_can_send(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 55);
            self.words[0] |= (((new_field >> 0) & mask) << 55);
        }
        #[inline]
        pub fn get_ep_can_receive(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 56) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_ep_can_receive(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 56);
            self.words[0] |= (((new_field >> 0) & mask) << 56);
        }
        #[inline]
        pub fn get_ep_ptr(&self) -> usize {
            let mask = ((1u128 << 48) - 1) as usize;
            let mut ret = ((self.words[0] >> 0) & mask) << 0;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_ep_ptr(&mut self, new_field: usize) {
            let mask = ((1u128 << 48) - 1) as usize;
            self.words[0] &= !(mask << 0);
            self.words[0] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn new_notification_cap(
            capNtfnBadge: usize,
            capNtfnCanReceive: usize,
            capNtfnCanSend: usize,
            capNtfnPtr: usize,
        ) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 64) - 1)) as usize;
            value.words[1] |= (((capNtfnBadge >> 0) & mask) << 0);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capNtfnCanReceive >> 0) & mask) << 58);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capNtfnCanSend >> 0) & mask) << 57);
            let mask = (((1u128 << 48) - 1)) as usize;
            value.words[0] |= (((capNtfnPtr >> 0) & mask) << 0);
            value.words[0]
                |= ((CapTag::CapNotificationCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_nf_badge(&self) -> usize {
            let mask = ((1u128 << 64) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_nf_badge(&mut self, new_field: usize) {
            let mask = ((1u128 << 64) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_nf_can_receive(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 58) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_nf_can_receive(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 58);
            self.words[0] |= (((new_field >> 0) & mask) << 58);
        }
        #[inline]
        pub fn get_nf_can_send(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 57) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_nf_can_send(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 57);
            self.words[0] |= (((new_field >> 0) & mask) << 57);
        }
        #[inline]
        pub fn get_nf_ptr(&self) -> usize {
            let mask = ((1u128 << 48) - 1) as usize;
            let mut ret = ((self.words[0] >> 0) & mask) << 0;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_nf_ptr(&mut self, new_field: usize) {
            let mask = ((1u128 << 48) - 1) as usize;
            self.words[0] &= !(mask << 0);
            self.words[0] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn new_reply_cap(
            capReplyCanGrant: usize,
            capReplyMaster: usize,
            capTCBPtr: usize,
        ) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capReplyCanGrant >> 0) & mask) << 1);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capReplyMaster >> 0) & mask) << 0);
            let mask = (((1u128 << 64) - 1)) as usize;
            value.words[1] |= (((capTCBPtr >> 0) & mask) << 0);
            value.words[0]
                |= ((CapTag::CapReplyCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_reply_can_grant(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 1) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_reply_can_grant(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 1);
            self.words[0] |= (((new_field >> 0) & mask) << 1);
        }
        #[inline]
        pub fn get_reply_master(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_reply_master(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 0);
            self.words[0] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_reply_tcb_ptr(&self) -> usize {
            let mask = ((1u128 << 64) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_reply_tcb_ptr(&mut self, new_field: usize) {
            let mask = ((1u128 << 64) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn new_cnode_cap(
            capCNodeRadix: usize,
            capCNodeGuardSize: usize,
            capCNodeGuard: usize,
            capCNodePtr: usize,
        ) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 6) - 1)) as usize;
            value.words[0] |= (((capCNodeRadix >> 0) & mask) << 47);
            let mask = (((1u128 << 6) - 1)) as usize;
            value.words[0] |= (((capCNodeGuardSize >> 0) & mask) << 53);
            let mask = (((1u128 << 64) - 1)) as usize;
            value.words[1] |= (((capCNodeGuard >> 0) & mask) << 0);
            let mask = (((1u128 << 47) - 1)) as usize;
            value.words[0] |= (((capCNodePtr >> 1) & mask) << 0);
            value.words[0]
                |= ((CapTag::CapCNodeCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_cnode_radix(&self) -> usize {
            let mask = ((1u128 << 6) - 1) as usize;
            let mut ret = ((self.words[0] >> 47) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_cnode_radix(&mut self, new_field: usize) {
            let mask = ((1u128 << 6) - 1) as usize;
            self.words[0] &= !(mask << 47);
            self.words[0] |= (((new_field >> 0) & mask) << 47);
        }
        #[inline]
        pub fn get_cnode_guard_size(&self) -> usize {
            let mask = ((1u128 << 6) - 1) as usize;
            let mut ret = ((self.words[0] >> 53) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_cnode_guard_size(&mut self, new_field: usize) {
            let mask = ((1u128 << 6) - 1) as usize;
            self.words[0] &= !(mask << 53);
            self.words[0] |= (((new_field >> 0) & mask) << 53);
        }
        #[inline]
        pub fn get_cnode_guard(&self) -> usize {
            let mask = ((1u128 << 64) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_cnode_guard(&mut self, new_field: usize) {
            let mask = ((1u128 << 64) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_cnode_ptr(&self) -> usize {
            let mask = ((1u128 << 47) - 1) as usize;
            let mut ret = ((self.words[0] >> 0) & mask) << 1;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_cnode_ptr(&mut self, new_field: usize) {
            let mask = ((1u128 << 47) - 1) as usize;
            self.words[0] &= !(mask << 0);
            self.words[0] |= (((new_field >> 1) & mask) << 0);
        }
        #[inline]
        pub fn new_thread_cap(capTCBPtr: usize) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 48) - 1)) as usize;
            value.words[0] |= (((capTCBPtr >> 0) & mask) << 0);
            value.words[0]
                |= ((CapTag::CapThreadCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_tcb_ptr(&self) -> usize {
            let mask = ((1u128 << 48) - 1) as usize;
            let mut ret = ((self.words[0] >> 0) & mask) << 0;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_tcb_ptr(&mut self, new_field: usize) {
            let mask = ((1u128 << 48) - 1) as usize;
            self.words[0] &= !(mask << 0);
            self.words[0] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn new_irq_control_cap() -> Self {
            let mut value = cap_t::default();
            value.words[0]
                |= ((CapTag::CapIrqControlCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn new_irq_handler_cap(capIRQ: usize) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 12) - 1)) as usize;
            value.words[1] |= (((capIRQ >> 0) & mask) << 0);
            value.words[0]
                |= ((CapTag::CapIrqHandlerCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_irq_handler(&self) -> usize {
            let mask = ((1u128 << 12) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_irq_handler(&mut self, new_field: usize) {
            let mask = ((1u128 << 12) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn new_zombie_cap(capZombieID: usize, capZombieType: usize) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 64) - 1)) as usize;
            value.words[1] |= (((capZombieID >> 0) & mask) << 0);
            let mask = (((1u128 << 7) - 1)) as usize;
            value.words[0] |= (((capZombieType >> 0) & mask) << 0);
            value.words[0]
                |= ((CapTag::CapZombieCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_zombie_id(&self) -> usize {
            let mask = ((1u128 << 64) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_zombie_id(&mut self, new_field: usize) {
            let mask = ((1u128 << 64) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_zombie_type(&self) -> usize {
            let mask = ((1u128 << 7) - 1) as usize;
            let mut ret = ((self.words[0] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_zombie_type(&mut self, new_field: usize) {
            let mask = ((1u128 << 7) - 1) as usize;
            self.words[0] &= !(mask << 0);
            self.words[0] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn new_domain_cap() -> Self {
            let mut value = cap_t::default();
            value.words[0]
                |= ((CapTag::CapDomainCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn new_frame_cap(
            capFMappedASID: usize,
            capFBasePtr: usize,
            capFMapType: usize,
            capFSize: usize,
            capFVMRights: usize,
            capFIsDevice: usize,
            capFMappedAddress: usize,
        ) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 16) - 1)) as usize;
            value.words[1] |= (((capFMappedASID >> 0) & mask) << 48);
            let mask = (((1u128 << 48) - 1)) as usize;
            value.words[1] |= (((capFBasePtr >> 0) & mask) << 0);
            let mask = (((1u128 << 2) - 1)) as usize;
            value.words[0] |= (((capFMapType >> 0) & mask) << 55);
            let mask = (((1u128 << 2) - 1)) as usize;
            value.words[0] |= (((capFSize >> 0) & mask) << 57);
            let mask = (((1u128 << 2) - 1)) as usize;
            value.words[0] |= (((capFVMRights >> 0) & mask) << 5);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capFIsDevice >> 0) & mask) << 4);
            let mask = (((1u128 << 48) - 1)) as usize;
            value.words[0] |= (((capFMappedAddress >> 0) & mask) << 7);
            value.words[0]
                |= ((CapTag::CapFrameCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_frame_mapped_asid(&self) -> usize {
            let mask = ((1u128 << 16) - 1) as usize;
            let mut ret = ((self.words[1] >> 48) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_frame_mapped_asid(&mut self, new_field: usize) {
            let mask = ((1u128 << 16) - 1) as usize;
            self.words[1] &= !(mask << 48);
            self.words[1] |= (((new_field >> 0) & mask) << 48);
        }
        #[inline]
        pub fn get_frame_base_ptr(&self) -> usize {
            let mask = ((1u128 << 48) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_frame_base_ptr(&mut self, new_field: usize) {
            let mask = ((1u128 << 48) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_frame_map_type(&self) -> usize {
            let mask = ((1u128 << 2) - 1) as usize;
            let mut ret = ((self.words[0] >> 55) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_frame_map_type(&mut self, new_field: usize) {
            let mask = ((1u128 << 2) - 1) as usize;
            self.words[0] &= !(mask << 55);
            self.words[0] |= (((new_field >> 0) & mask) << 55);
        }
        #[inline]
        pub fn get_frame_size(&self) -> usize {
            let mask = ((1u128 << 2) - 1) as usize;
            let mut ret = ((self.words[0] >> 57) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_frame_size(&mut self, new_field: usize) {
            let mask = ((1u128 << 2) - 1) as usize;
            self.words[0] &= !(mask << 57);
            self.words[0] |= (((new_field >> 0) & mask) << 57);
        }
        #[inline]
        pub fn get_frame_vm_rights(&self) -> usize {
            let mask = ((1u128 << 2) - 1) as usize;
            let mut ret = ((self.words[0] >> 5) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_frame_vm_rights(&mut self, new_field: usize) {
            let mask = ((1u128 << 2) - 1) as usize;
            self.words[0] &= !(mask << 5);
            self.words[0] |= (((new_field >> 0) & mask) << 5);
        }
        #[inline]
        pub fn get_frame_is_device(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 4) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_frame_is_device(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 4);
            self.words[0] |= (((new_field >> 0) & mask) << 4);
        }
        #[inline]
        pub fn get_frame_mapped_address(&self) -> usize {
            let mask = ((1u128 << 48) - 1) as usize;
            let mut ret = ((self.words[0] >> 7) & mask) << 0;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_frame_mapped_address(&mut self, new_field: usize) {
            let mask = ((1u128 << 48) - 1) as usize;
            self.words[0] &= !(mask << 7);
            self.words[0] |= (((new_field >> 0) & mask) << 7);
        }
        #[inline]
        pub fn new_page_table_cap(
            capPTMappedASID: usize,
            capPTBasePtr: usize,
            capPTIsMapped: usize,
            capPTMappedAddress: usize,
        ) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 12) - 1)) as usize;
            value.words[1] |= (((capPTMappedASID >> 0) & mask) << 48);
            let mask = (((1u128 << 48) - 1)) as usize;
            value.words[1] |= (((capPTBasePtr >> 0) & mask) << 0);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((capPTIsMapped >> 0) & mask) << 49);
            let mask = (((1u128 << 28) - 1)) as usize;
            value.words[0] |= (((capPTMappedAddress >> 5) & mask) << 21);
            value.words[0]
                |= ((CapTag::CapPageTableCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_pt_mapped_asid(&self) -> usize {
            let mask = ((1u128 << 12) - 1) as usize;
            let mut ret = ((self.words[1] >> 48) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_pt_mapped_asid(&mut self, new_field: usize) {
            let mask = ((1u128 << 12) - 1) as usize;
            self.words[1] &= !(mask << 48);
            self.words[1] |= (((new_field >> 0) & mask) << 48);
        }
        #[inline]
        pub fn get_pt_base_ptr(&self) -> usize {
            let mask = ((1u128 << 48) - 1) as usize;
            let mut ret = ((self.words[1] >> 0) & mask) << 0;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_pt_base_ptr(&mut self, new_field: usize) {
            let mask = ((1u128 << 48) - 1) as usize;
            self.words[1] &= !(mask << 0);
            self.words[1] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_pt_is_mapped(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 49) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_pt_is_mapped(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 49);
            self.words[0] |= (((new_field >> 0) & mask) << 49);
        }
        #[inline]
        pub fn get_pt_mapped_address(&self) -> usize {
            let mask = ((1u128 << 28) - 1) as usize;
            let mut ret = ((self.words[0] >> 21) & mask) << 5;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_pt_mapped_address(&mut self, new_field: usize) {
            let mask = ((1u128 << 28) - 1) as usize;
            self.words[0] &= !(mask << 21);
            self.words[0] |= (((new_field >> 5) & mask) << 21);
        }
        #[inline]
        pub fn new_asid_control_cap() -> Self {
            let mut value = cap_t::default();
            value.words[0]
                |= ((CapTag::CapASIDControlCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn new_asid_pool_cap(capASIDBase: usize, capASIDPool: usize) -> Self {
            let mut value = cap_t::default();
            let mask = (((1u128 << 12) - 1)) as usize;
            value.words[0] |= (((capASIDBase >> 0) & mask) << 47);
            let mask = (((1u128 << 37) - 1)) as usize;
            value.words[0] |= (((capASIDPool >> 11) & mask) << 0);
            value.words[0]
                |= ((CapTag::CapASIDPoolCap as usize & ((1usize << 5) - 1)) << 59);
            value
        }
        #[inline]
        pub fn get_asid_base(&self) -> usize {
            let mask = ((1u128 << 12) - 1) as usize;
            let mut ret = ((self.words[0] >> 47) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_asid_base(&mut self, new_field: usize) {
            let mask = ((1u128 << 12) - 1) as usize;
            self.words[0] &= !(mask << 47);
            self.words[0] |= (((new_field >> 0) & mask) << 47);
        }
        #[inline]
        pub fn get_asid_pool(&self) -> usize {
            let mask = ((1u128 << 37) - 1) as usize;
            let mut ret = ((self.words[0] >> 0) & mask) << 11;
            if true && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_asid_pool(&mut self, new_field: usize) {
            let mask = ((1u128 << 37) - 1) as usize;
            self.words[0] &= !(mask << 0);
            self.words[0] |= (((new_field >> 11) & mask) << 0);
        }
        #[inline]
        pub fn get_type(&self) -> usize {
            (self.words[0] >> 59) & ((1usize << 5) - 1)
        }
    }
    /// cap 的公用方法
    impl cap_t {
        pub fn update_data(&self, preserve: bool, new_data: usize) -> Self {
            if self.isArchCap() {
                return self.clone();
            }
            match self.get_cap_type() {
                CapTag::CapEndpointCap => {
                    if !preserve && self.get_ep_badge() == 0 {
                        let mut new_cap = self.clone();
                        new_cap.set_ep_badge(new_data);
                        new_cap
                    } else {
                        cap_t::new_null_cap()
                    }
                }
                CapTag::CapNotificationCap => {
                    if !preserve && self.get_nf_badge() == 0 {
                        let mut new_cap = self.clone();
                        new_cap.set_nf_badge(new_data);
                        new_cap
                    } else {
                        cap_t::new_null_cap()
                    }
                }
                CapTag::CapCNodeCap => {
                    let w = CNodeCapData::new(new_data);
                    let guard_size = w.get_guard_size();
                    if guard_size + self.get_cnode_radix() > wordBits {
                        return cap_t::new_null_cap();
                    }
                    let guard = w.get_guard() & { (1usize << guard_size) - 1usize };
                    let mut new_cap = self.clone();
                    new_cap.set_cnode_guard(guard);
                    new_cap.set_cnode_guard_size(guard_size);
                    new_cap
                }
                _ => self.clone(),
            }
        }
        pub fn get_cap_type(&self) -> CapTag {
            unsafe { core::mem::transmute::<u8, CapTag>(self.get_type() as u8) }
        }
        pub fn get_cap_ptr(&self) -> usize {
            match self.get_cap_type() {
                CapTag::CapUntypedCap => self.get_untyped_ptr(),
                CapTag::CapEndpointCap => self.get_ep_ptr(),
                CapTag::CapNotificationCap => self.get_nf_ptr(),
                CapTag::CapCNodeCap => self.get_cnode_ptr(),
                CapTag::CapThreadCap => self.get_tcb_ptr(),
                CapTag::CapZombieCap => self.get_zombie_ptr(),
                CapTag::CapFrameCap => self.get_frame_base_ptr(),
                CapTag::CapPageTableCap => self.get_pt_base_ptr(),
                CapTag::CapASIDPoolCap => self.get_asid_pool(),
                _ => 0,
            }
        }
        pub fn get_cap_size_bits(&self) -> usize {
            match self.get_cap_type() {
                CapTag::CapUntypedCap => self.get_untyped_block_size(),
                CapTag::CapEndpointCap => seL4_EndpointBits,
                CapTag::CapNotificationCap => seL4_NotificationBits,
                CapTag::CapCNodeCap => self.get_cnode_radix() + seL4_SlotBits,
                CapTag::CapPageTableCap => PT_SIZE_BITS,
                CapTag::CapReplyCap => seL4_ReplyBits,
                _ => 0,
            }
        }
        pub fn get_cap_is_physical(&self) -> bool {
            match self.get_cap_type() {
                CapTag::CapUntypedCap
                | CapTag::CapEndpointCap
                | CapTag::CapNotificationCap
                | CapTag::CapCNodeCap
                | CapTag::CapFrameCap
                | CapTag::CapASIDPoolCap
                | CapTag::CapPageTableCap
                | CapTag::CapZombieCap
                | CapTag::CapThreadCap => true,
                _ => false,
            }
        }
        pub fn isArchCap(&self) -> bool {
            self.get_cap_type() as usize % 2 != 0
        }
    }
    pub fn same_region_as(cap1: &cap_t, cap2: &cap_t) -> bool {
        match cap1.get_cap_type() {
            CapTag::CapUntypedCap => {
                if cap2.get_cap_is_physical() {
                    let aBase = cap1.get_untyped_ptr();
                    let bBase = cap2.get_cap_ptr();
                    let aTop = aBase
                        + { (1usize << cap1.get_untyped_block_size()) - 1usize };
                    let bTop = bBase + { (1usize << cap2.get_cap_size_bits()) - 1usize };
                    return (aBase <= bBase) && (bTop <= aTop) && (bBase <= bTop);
                }
                return false;
            }
            CapTag::CapFrameCap => {
                if cap2.get_cap_type() == CapTag::CapFrameCap {
                    let botA = cap1.get_frame_base_ptr();
                    let botB = cap2.get_frame_base_ptr();
                    let topA = botA
                        + {
                            (1usize << pageBitsForSize(cap1.get_frame_size())) - 1usize
                        };
                    let topB = botB
                        + {
                            (1usize << pageBitsForSize(cap2.get_frame_size())) - 1usize
                        };
                    return (botA <= botB) && (topA >= topB) && (botB <= topB);
                }
                false
            }
            CapTag::CapEndpointCap
            | CapTag::CapNotificationCap
            | CapTag::CapPageTableCap
            | CapTag::CapASIDPoolCap
            | CapTag::CapThreadCap => {
                if cap2.get_cap_type() == cap1.get_cap_type() {
                    return cap1.get_cap_ptr() == cap2.get_cap_ptr();
                }
                false
            }
            CapTag::CapASIDControlCap | CapTag::CapDomainCap => {
                if cap2.get_cap_type() == cap1.get_cap_type() {
                    return true;
                }
                false
            }
            CapTag::CapCNodeCap => {
                if cap2.get_cap_type() == CapTag::CapCNodeCap {
                    return (cap1.get_cnode_ptr() == cap2.get_cnode_ptr())
                        && (cap1.get_cnode_radix() == cap2.get_cnode_radix());
                }
                false
            }
            CapTag::CapIrqControlCap => {
                match cap2.get_cap_type() {
                    CapTag::CapIrqControlCap | CapTag::CapIrqHandlerCap => true,
                    _ => false,
                }
            }
            CapTag::CapIrqHandlerCap => {
                if cap2.get_cap_type() == CapTag::CapIrqHandlerCap {
                    return cap1.get_irq_handler() == cap2.get_irq_handler();
                }
                false
            }
            _ => {
                return false;
            }
        }
    }
    /// 判断两个cap指向的内核对象是否是同一个内存区域
    pub fn same_object_as(cap1: &cap_t, cap2: &cap_t) -> bool {
        if cap1.get_cap_type() == CapTag::CapUntypedCap {
            return false;
        }
        if cap1.get_cap_type() == CapTag::CapIrqControlCap
            && cap2.get_cap_type() == CapTag::CapIrqHandlerCap
        {
            return false;
        }
        if cap1.isArchCap() && cap2.isArchCap() {
            return arch_same_object_as(cap1, cap2);
        }
        same_region_as(cap1, cap2)
    }
    fn arch_same_object_as(cap1: &cap_t, cap2: &cap_t) -> bool {
        if cap1.get_cap_type() == CapTag::CapFrameCap
            && cap2.get_cap_type() == CapTag::CapFrameCap
        {
            return cap1.get_frame_base_ptr() == cap2.get_frame_base_ptr()
                && cap1.get_frame_size() == cap2.get_frame_size()
                && (cap1.get_frame_is_device() == 0)
                    == (cap2.get_frame_is_device() == 0);
        }
        same_region_as(cap1, cap2)
    }
    pub fn is_cap_revocable(derived_cap: &cap_t, src_cap: &cap_t) -> bool {
        if derived_cap.isArchCap() {
            return false;
        }
        match derived_cap.get_cap_type() {
            CapTag::CapEndpointCap => {
                match (&src_cap.get_cap_type(), &CapTag::CapEndpointCap) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                return derived_cap.get_ep_badge() != src_cap.get_ep_badge();
            }
            CapTag::CapNotificationCap => {
                match (&src_cap.get_cap_type(), &CapTag::CapNotificationCap) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                return derived_cap.get_nf_badge() != src_cap.get_nf_badge();
            }
            CapTag::CapIrqHandlerCap => {
                return src_cap.get_cap_type() == CapTag::CapIrqControlCap;
            }
            CapTag::CapUntypedCap => {
                return true;
            }
            _ => false,
        }
    }
    #[no_mangle]
    pub fn Arch_finaliseCap(cap: &cap_t, final_: bool) -> finaliseCap_ret {
        let mut fc_ret = finaliseCap_ret::default();
        ::core::panicking::panic("not implemented")
    }
    #[repr(C)]
    pub struct finaliseSlot_ret {
        pub status: exception_t,
        pub success: bool,
        pub cleanupInfo: cap_t,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for finaliseSlot_ret {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "finaliseSlot_ret",
                "status",
                &self.status,
                "success",
                &self.success,
                "cleanupInfo",
                &&self.cleanupInfo,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for finaliseSlot_ret {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for finaliseSlot_ret {
        #[inline]
        fn eq(&self, other: &finaliseSlot_ret) -> bool {
            self.status == other.status && self.success == other.success
                && self.cleanupInfo == other.cleanupInfo
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for finaliseSlot_ret {
        #[inline]
        fn clone(&self) -> finaliseSlot_ret {
            let _: ::core::clone::AssertParamIsClone<exception_t>;
            let _: ::core::clone::AssertParamIsClone<bool>;
            let _: ::core::clone::AssertParamIsClone<cap_t>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for finaliseSlot_ret {}
    impl Default for finaliseSlot_ret {
        fn default() -> Self {
            finaliseSlot_ret {
                status: exception_t::EXCEPTION_NONE,
                success: true,
                cleanupInfo: cap_t::default(),
            }
        }
    }
    #[repr(C)]
    pub struct finaliseCap_ret {
        pub remainder: cap_t,
        pub cleanupInfo: cap_t,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for finaliseCap_ret {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "finaliseCap_ret",
                "remainder",
                &self.remainder,
                "cleanupInfo",
                &&self.cleanupInfo,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for finaliseCap_ret {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for finaliseCap_ret {
        #[inline]
        fn eq(&self, other: &finaliseCap_ret) -> bool {
            self.remainder == other.remainder && self.cleanupInfo == other.cleanupInfo
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for finaliseCap_ret {
        #[inline]
        fn clone(&self) -> finaliseCap_ret {
            let _: ::core::clone::AssertParamIsClone<cap_t>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for finaliseCap_ret {}
    impl Default for finaliseCap_ret {
        fn default() -> Self {
            finaliseCap_ret {
                remainder: cap_t::default(),
                cleanupInfo: cap_t::default(),
            }
        }
    }
    #[no_mangle]
    pub fn finaliseCap(cap: &cap_t, _final: bool, _exposed: bool) -> finaliseCap_ret {
        ::core::panicking::panic("not yet implemented")
    }
    #[no_mangle]
    pub fn post_cap_deletion(cap: &cap_t) {
        if cap.get_cap_type() == CapTag::CapIrqHandlerCap {
            let irq = cap.get_irq_handler();
            ::core::panicking::panic("not implemented")
        }
    }
    #[no_mangle]
    pub fn preemptionPoint() -> exception_t {
        unsafe { ::core::panicking::panic("not implemented") }
    }
    #[repr(C)]
    pub struct seL4_CapRights_t {
        pub words: [usize; 1],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for seL4_CapRights_t {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "seL4_CapRights_t",
                "words",
                &&self.words,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for seL4_CapRights_t {}
    #[automatically_derived]
    impl ::core::clone::Clone for seL4_CapRights_t {
        #[inline]
        fn clone(&self) -> seL4_CapRights_t {
            let _: ::core::clone::AssertParamIsClone<[usize; 1]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for seL4_CapRights_t {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for seL4_CapRights_t {
        #[inline]
        fn eq(&self, other: &seL4_CapRights_t) -> bool {
            self.words == other.words
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for seL4_CapRights_t {}
    #[automatically_derived]
    impl ::core::cmp::Eq for seL4_CapRights_t {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<[usize; 1]>;
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for seL4_CapRights_t {
        #[inline]
        fn default() -> seL4_CapRights_t {
            seL4_CapRights_t {
                words: ::core::default::Default::default(),
            }
        }
    }
    impl seL4_CapRights_t {
        #[inline]
        pub fn new(
            allow_grant_reply: usize,
            allow_grant: usize,
            allow_read: usize,
            allow_write: usize,
        ) -> Self {
            let mut value = seL4_CapRights_t::default();
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((allow_grant_reply >> 0) & mask) << 3);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((allow_grant >> 0) & mask) << 2);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((allow_read >> 0) & mask) << 1);
            let mask = (((1u128 << 1) - 1)) as usize;
            value.words[0] |= (((allow_write >> 0) & mask) << 0);
            value.words[0] |= ((0 & ((1usize << 0) - 1)) << 0);
            value
        }
        #[inline]
        pub fn get_allow_grant_reply(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 3) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_allow_grant_reply(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 3);
            self.words[0] |= (((new_field >> 0) & mask) << 3);
        }
        #[inline]
        pub fn get_allow_grant(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 2) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_allow_grant(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 2);
            self.words[0] |= (((new_field >> 0) & mask) << 2);
        }
        #[inline]
        pub fn get_allow_read(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 1) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_allow_read(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 1);
            self.words[0] |= (((new_field >> 0) & mask) << 1);
        }
        #[inline]
        pub fn get_allow_write(&self) -> usize {
            let mask = ((1u128 << 1) - 1) as usize;
            let mut ret = ((self.words[0] >> 0) & mask) << 0;
            if false && (ret & (1usize << 47)) != 0 {
                ret |= 0xffff000000000000;
            }
            ret
        }
        #[inline]
        pub fn set_allow_write(&mut self, new_field: usize) {
            let mask = ((1u128 << 1) - 1) as usize;
            self.words[0] &= !(mask << 0);
            self.words[0] |= (((new_field >> 0) & mask) << 0);
        }
        #[inline]
        pub fn get_type(&self) -> usize {
            (self.words[0] >> 0) & ((1usize << 0) - 1)
        }
    }
    impl seL4_CapRights_t {
        #[inline]
        pub fn from_word(word: usize) -> Self {
            Self { words: [word] }
        }
    }
}
