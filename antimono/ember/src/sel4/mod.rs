use aster_frame::cpu::UserContext;

pub use sel4::sys::*;

pub mod register;
pub mod tcb;
pub mod cnode;



pub mod config {
    pub mod consts {
        pub use sel4::config::consts::*;
    }
}


pub trait SeL4ABI {
    fn syscall_num(&self) -> usize;

    fn syscall_ret(&self) -> usize;

    fn set_syscall_ret(&mut self, ret: usize);

    fn syscall_args(&self) -> [usize; 6];

    fn set_tls_pointer(&mut self, tls: usize);

    fn tls_pointer(&self) -> usize;

    fn cap_reg(&self) -> usize;

    fn badge_reg(&self) -> usize;

    fn msg_info_reg(&self) -> usize;
}

impl SeL4ABI for UserContext {
    fn syscall_num(&self) -> usize {
        self.rax()
    }

    fn syscall_ret(&self) -> usize {
        self.rax()
    }

    fn set_syscall_ret(&mut self, ret: usize) {
        self.set_rax(ret);
    }

    fn syscall_args(&self) -> [usize; 6] {
        [
            self.rdi(),
            self.rsi(),
            self.rdx(),
            self.r10(),
            self.r8(),
            self.r9(),
        ]
    }

    fn set_tls_pointer(&mut self, tls: usize) {
        self.set_fsbase(tls);
    }

    fn tls_pointer(&self) -> usize {
        self.fsbase()
    }

    fn cap_reg(&self) -> usize {
        self.rdi()
    }

    fn badge_reg(&self) -> usize {
        self.rdi()
    }

    fn msg_info_reg(&self) -> usize {
        self.rsi()
    }
}
