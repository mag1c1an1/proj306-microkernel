use aster_frame::cpu::GeneralRegs;

pub mod thread;

pub trait SeL4Regs {
    fn get_cap_reg(&self) -> usize;
    fn get_badge_reg(&self) -> usize;
    fn get_msg_info_reg(&self) -> usize;
    fn set_tls(&mut self, tls: usize);
}

impl SeL4Regs for GeneralRegs {
    fn get_cap_reg(&self) -> usize {
        self.rdi
    }

    fn get_badge_reg(&self) -> usize {
        self.rdi
    }

    fn get_msg_info_reg(&self) -> usize {
        self.rsi
    }

    fn set_tls(&mut self, tls: usize) {
        self.fsbase = tls;
    }
}
pub use sel4::sys::*;
