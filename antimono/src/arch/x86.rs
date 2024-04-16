mod arch {
    mod types {
        pub type word_t = u32;
        pub type sword_t = i32;
        pub type vptr_t = word_t;
        pub type paddr_t = word_t;
        pub type pptr_t = word_t;
        pub type cptr_t = word_t;
        pub type dev_id_t = word_t;
        pub type cpu_id_t = word_t;
        pub type node_id_t = word_t;
        pub type dom_t = word_t;
        pub type logical_id_t = u32;
        pub type timestamp_t = u64;
        // exception
        pub type exception_t = word_t;
    }
    pub use types::*;
}

pub use arch::*;
