#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_NullFault(pub SeL4Bitfield<u64, 20usize>);
impl seL4_Fault_NullFault {
    pub fn new() -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_NullFault);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_NullFault_Unpacked {
        seL4_Fault_NullFault_Unpacked {}
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_NullFault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_NullFault_Unpacked {}
impl seL4_Fault_NullFault_Unpacked {
    pub fn pack(self) -> seL4_Fault_NullFault {
        match self {
            Self {} => seL4_Fault_NullFault::new(),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_CapFault(pub SeL4Bitfield<u64, 20usize>);
impl seL4_Fault_CapFault {
    pub fn new(
        IP: u64,
        Addr: u64,
        InRecvPhase: u64,
        LookupFailureType: u64,
        MR4: u64,
        MR5: u64,
        MR6: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_IP(IP);
        this.set_Addr(Addr);
        this.set_InRecvPhase(InRecvPhase);
        this.set_LookupFailureType(LookupFailureType);
        this.set_MR4(MR4);
        this.set_MR5(MR5);
        this.set_MR6(MR6);
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_CapFault);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_CapFault_Unpacked {
        seL4_Fault_CapFault_Unpacked {
            IP: self.get_IP(),
            Addr: self.get_Addr(),
            InRecvPhase: self.get_InRecvPhase(),
            LookupFailureType: self.get_LookupFailureType(),
            MR4: self.get_MR4(),
            MR5: self.get_MR5(),
            MR6: self.get_MR6(),
        }
    }
    #[allow(dead_code)]
    pub fn get_IP(&self) -> u64 {
        self.0.get_bits(448usize..512usize)
    }
    pub fn set_IP(&mut self, IP: u64) {
        self.0.set_bits(448usize..512usize, IP)
    }
    #[allow(dead_code)]
    pub const fn width_of_IP() -> usize {
        512usize - 448usize
    }
    #[allow(dead_code)]
    pub fn get_Addr(&self) -> u64 {
        self.0.get_bits(384usize..448usize)
    }
    pub fn set_Addr(&mut self, Addr: u64) {
        self.0.set_bits(384usize..448usize, Addr)
    }
    #[allow(dead_code)]
    pub const fn width_of_Addr() -> usize {
        448usize - 384usize
    }
    #[allow(dead_code)]
    pub fn get_InRecvPhase(&self) -> u64 {
        self.0.get_bits(320usize..384usize)
    }
    pub fn set_InRecvPhase(&mut self, InRecvPhase: u64) {
        self.0.set_bits(320usize..384usize, InRecvPhase)
    }
    #[allow(dead_code)]
    pub const fn width_of_InRecvPhase() -> usize {
        384usize - 320usize
    }
    #[allow(dead_code)]
    pub fn get_LookupFailureType(&self) -> u64 {
        self.0.get_bits(256usize..320usize)
    }
    pub fn set_LookupFailureType(&mut self, LookupFailureType: u64) {
        self.0.set_bits(256usize..320usize, LookupFailureType)
    }
    #[allow(dead_code)]
    pub const fn width_of_LookupFailureType() -> usize {
        320usize - 256usize
    }
    #[allow(dead_code)]
    pub fn get_MR4(&self) -> u64 {
        self.0.get_bits(192usize..256usize)
    }
    pub fn set_MR4(&mut self, MR4: u64) {
        self.0.set_bits(192usize..256usize, MR4)
    }
    #[allow(dead_code)]
    pub const fn width_of_MR4() -> usize {
        256usize - 192usize
    }
    #[allow(dead_code)]
    pub fn get_MR5(&self) -> u64 {
        self.0.get_bits(128usize..192usize)
    }
    pub fn set_MR5(&mut self, MR5: u64) {
        self.0.set_bits(128usize..192usize, MR5)
    }
    #[allow(dead_code)]
    pub const fn width_of_MR5() -> usize {
        192usize - 128usize
    }
    #[allow(dead_code)]
    pub fn get_MR6(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_MR6(&mut self, MR6: u64) {
        self.0.set_bits(64usize..128usize, MR6)
    }
    #[allow(dead_code)]
    pub const fn width_of_MR6() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_CapFault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_CapFault_Unpacked {
    pub IP: u64,
    pub Addr: u64,
    pub InRecvPhase: u64,
    pub LookupFailureType: u64,
    pub MR4: u64,
    pub MR5: u64,
    pub MR6: u64,
}
impl seL4_Fault_CapFault_Unpacked {
    pub fn pack(self) -> seL4_Fault_CapFault {
        match self {
            Self {
                IP,
                Addr,
                InRecvPhase,
                LookupFailureType,
                MR4,
                MR5,
                MR6,
            } => seL4_Fault_CapFault::new(IP, Addr, InRecvPhase, LookupFailureType, MR4, MR5, MR6),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_UnknownSyscall(pub SeL4Bitfield<u64, 20usize>);
impl seL4_Fault_UnknownSyscall {
    pub fn new(
        RAX: u64,
        RBX: u64,
        RCX: u64,
        RDX: u64,
        RSI: u64,
        RDI: u64,
        RBP: u64,
        R8: u64,
        R9: u64,
        R10: u64,
        R11: u64,
        R12: u64,
        R13: u64,
        R14: u64,
        R15: u64,
        FaultIP: u64,
        RSP: u64,
        FLAGS: u64,
        Syscall: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_RAX(RAX);
        this.set_RBX(RBX);
        this.set_RCX(RCX);
        this.set_RDX(RDX);
        this.set_RSI(RSI);
        this.set_RDI(RDI);
        this.set_RBP(RBP);
        this.set_R8(R8);
        this.set_R9(R9);
        this.set_R10(R10);
        this.set_R11(R11);
        this.set_R12(R12);
        this.set_R13(R13);
        this.set_R14(R14);
        this.set_R15(R15);
        this.set_FaultIP(FaultIP);
        this.set_RSP(RSP);
        this.set_FLAGS(FLAGS);
        this.set_Syscall(Syscall);
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_UnknownSyscall);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_UnknownSyscall_Unpacked {
        seL4_Fault_UnknownSyscall_Unpacked {
            RAX: self.get_RAX(),
            RBX: self.get_RBX(),
            RCX: self.get_RCX(),
            RDX: self.get_RDX(),
            RSI: self.get_RSI(),
            RDI: self.get_RDI(),
            RBP: self.get_RBP(),
            R8: self.get_R8(),
            R9: self.get_R9(),
            R10: self.get_R10(),
            R11: self.get_R11(),
            R12: self.get_R12(),
            R13: self.get_R13(),
            R14: self.get_R14(),
            R15: self.get_R15(),
            FaultIP: self.get_FaultIP(),
            RSP: self.get_RSP(),
            FLAGS: self.get_FLAGS(),
            Syscall: self.get_Syscall(),
        }
    }
    #[allow(dead_code)]
    pub fn get_RAX(&self) -> u64 {
        self.0.get_bits(1216usize..1280usize)
    }
    pub fn set_RAX(&mut self, RAX: u64) {
        self.0.set_bits(1216usize..1280usize, RAX)
    }
    #[allow(dead_code)]
    pub const fn width_of_RAX() -> usize {
        1280usize - 1216usize
    }
    #[allow(dead_code)]
    pub fn get_RBX(&self) -> u64 {
        self.0.get_bits(1152usize..1216usize)
    }
    pub fn set_RBX(&mut self, RBX: u64) {
        self.0.set_bits(1152usize..1216usize, RBX)
    }
    #[allow(dead_code)]
    pub const fn width_of_RBX() -> usize {
        1216usize - 1152usize
    }
    #[allow(dead_code)]
    pub fn get_RCX(&self) -> u64 {
        self.0.get_bits(1088usize..1152usize)
    }
    pub fn set_RCX(&mut self, RCX: u64) {
        self.0.set_bits(1088usize..1152usize, RCX)
    }
    #[allow(dead_code)]
    pub const fn width_of_RCX() -> usize {
        1152usize - 1088usize
    }
    #[allow(dead_code)]
    pub fn get_RDX(&self) -> u64 {
        self.0.get_bits(1024usize..1088usize)
    }
    pub fn set_RDX(&mut self, RDX: u64) {
        self.0.set_bits(1024usize..1088usize, RDX)
    }
    #[allow(dead_code)]
    pub const fn width_of_RDX() -> usize {
        1088usize - 1024usize
    }
    #[allow(dead_code)]
    pub fn get_RSI(&self) -> u64 {
        self.0.get_bits(960usize..1024usize)
    }
    pub fn set_RSI(&mut self, RSI: u64) {
        self.0.set_bits(960usize..1024usize, RSI)
    }
    #[allow(dead_code)]
    pub const fn width_of_RSI() -> usize {
        1024usize - 960usize
    }
    #[allow(dead_code)]
    pub fn get_RDI(&self) -> u64 {
        self.0.get_bits(896usize..960usize)
    }
    pub fn set_RDI(&mut self, RDI: u64) {
        self.0.set_bits(896usize..960usize, RDI)
    }
    #[allow(dead_code)]
    pub const fn width_of_RDI() -> usize {
        960usize - 896usize
    }
    #[allow(dead_code)]
    pub fn get_RBP(&self) -> u64 {
        self.0.get_bits(832usize..896usize)
    }
    pub fn set_RBP(&mut self, RBP: u64) {
        self.0.set_bits(832usize..896usize, RBP)
    }
    #[allow(dead_code)]
    pub const fn width_of_RBP() -> usize {
        896usize - 832usize
    }
    #[allow(dead_code)]
    pub fn get_R8(&self) -> u64 {
        self.0.get_bits(768usize..832usize)
    }
    pub fn set_R8(&mut self, R8: u64) {
        self.0.set_bits(768usize..832usize, R8)
    }
    #[allow(dead_code)]
    pub const fn width_of_R8() -> usize {
        832usize - 768usize
    }
    #[allow(dead_code)]
    pub fn get_R9(&self) -> u64 {
        self.0.get_bits(704usize..768usize)
    }
    pub fn set_R9(&mut self, R9: u64) {
        self.0.set_bits(704usize..768usize, R9)
    }
    #[allow(dead_code)]
    pub const fn width_of_R9() -> usize {
        768usize - 704usize
    }
    #[allow(dead_code)]
    pub fn get_R10(&self) -> u64 {
        self.0.get_bits(640usize..704usize)
    }
    pub fn set_R10(&mut self, R10: u64) {
        self.0.set_bits(640usize..704usize, R10)
    }
    #[allow(dead_code)]
    pub const fn width_of_R10() -> usize {
        704usize - 640usize
    }
    #[allow(dead_code)]
    pub fn get_R11(&self) -> u64 {
        self.0.get_bits(576usize..640usize)
    }
    pub fn set_R11(&mut self, R11: u64) {
        self.0.set_bits(576usize..640usize, R11)
    }
    #[allow(dead_code)]
    pub const fn width_of_R11() -> usize {
        640usize - 576usize
    }
    #[allow(dead_code)]
    pub fn get_R12(&self) -> u64 {
        self.0.get_bits(512usize..576usize)
    }
    pub fn set_R12(&mut self, R12: u64) {
        self.0.set_bits(512usize..576usize, R12)
    }
    #[allow(dead_code)]
    pub const fn width_of_R12() -> usize {
        576usize - 512usize
    }
    #[allow(dead_code)]
    pub fn get_R13(&self) -> u64 {
        self.0.get_bits(448usize..512usize)
    }
    pub fn set_R13(&mut self, R13: u64) {
        self.0.set_bits(448usize..512usize, R13)
    }
    #[allow(dead_code)]
    pub const fn width_of_R13() -> usize {
        512usize - 448usize
    }
    #[allow(dead_code)]
    pub fn get_R14(&self) -> u64 {
        self.0.get_bits(384usize..448usize)
    }
    pub fn set_R14(&mut self, R14: u64) {
        self.0.set_bits(384usize..448usize, R14)
    }
    #[allow(dead_code)]
    pub const fn width_of_R14() -> usize {
        448usize - 384usize
    }
    #[allow(dead_code)]
    pub fn get_R15(&self) -> u64 {
        self.0.get_bits(320usize..384usize)
    }
    pub fn set_R15(&mut self, R15: u64) {
        self.0.set_bits(320usize..384usize, R15)
    }
    #[allow(dead_code)]
    pub const fn width_of_R15() -> usize {
        384usize - 320usize
    }
    #[allow(dead_code)]
    pub fn get_FaultIP(&self) -> u64 {
        self.0.get_bits(256usize..320usize)
    }
    pub fn set_FaultIP(&mut self, FaultIP: u64) {
        self.0.set_bits(256usize..320usize, FaultIP)
    }
    #[allow(dead_code)]
    pub const fn width_of_FaultIP() -> usize {
        320usize - 256usize
    }
    #[allow(dead_code)]
    pub fn get_RSP(&self) -> u64 {
        self.0.get_bits(192usize..256usize)
    }
    pub fn set_RSP(&mut self, RSP: u64) {
        self.0.set_bits(192usize..256usize, RSP)
    }
    #[allow(dead_code)]
    pub const fn width_of_RSP() -> usize {
        256usize - 192usize
    }
    #[allow(dead_code)]
    pub fn get_FLAGS(&self) -> u64 {
        self.0.get_bits(128usize..192usize)
    }
    pub fn set_FLAGS(&mut self, FLAGS: u64) {
        self.0.set_bits(128usize..192usize, FLAGS)
    }
    #[allow(dead_code)]
    pub const fn width_of_FLAGS() -> usize {
        192usize - 128usize
    }
    #[allow(dead_code)]
    pub fn get_Syscall(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_Syscall(&mut self, Syscall: u64) {
        self.0.set_bits(64usize..128usize, Syscall)
    }
    #[allow(dead_code)]
    pub const fn width_of_Syscall() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_UnknownSyscall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_UnknownSyscall_Unpacked {
    pub RAX: u64,
    pub RBX: u64,
    pub RCX: u64,
    pub RDX: u64,
    pub RSI: u64,
    pub RDI: u64,
    pub RBP: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub FaultIP: u64,
    pub RSP: u64,
    pub FLAGS: u64,
    pub Syscall: u64,
}
impl seL4_Fault_UnknownSyscall_Unpacked {
    pub fn pack(self) -> seL4_Fault_UnknownSyscall {
        match self {
            Self {
                RAX,
                RBX,
                RCX,
                RDX,
                RSI,
                RDI,
                RBP,
                R8,
                R9,
                R10,
                R11,
                R12,
                R13,
                R14,
                R15,
                FaultIP,
                RSP,
                FLAGS,
                Syscall,
            } => seL4_Fault_UnknownSyscall::new(
                RAX, RBX, RCX, RDX, RSI, RDI, RBP, R8, R9, R10, R11, R12, R13, R14, R15, FaultIP,
                RSP, FLAGS, Syscall,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_UserException(pub SeL4Bitfield<u64, 20usize>);
impl seL4_Fault_UserException {
    pub fn new(FaultIP: u64, Stack: u64, FLAGS: u64, Number: u64, Code: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_FaultIP(FaultIP);
        this.set_Stack(Stack);
        this.set_FLAGS(FLAGS);
        this.set_Number(Number);
        this.set_Code(Code);
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_UserException);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_UserException_Unpacked {
        seL4_Fault_UserException_Unpacked {
            FaultIP: self.get_FaultIP(),
            Stack: self.get_Stack(),
            FLAGS: self.get_FLAGS(),
            Number: self.get_Number(),
            Code: self.get_Code(),
        }
    }
    #[allow(dead_code)]
    pub fn get_FaultIP(&self) -> u64 {
        self.0.get_bits(320usize..384usize)
    }
    pub fn set_FaultIP(&mut self, FaultIP: u64) {
        self.0.set_bits(320usize..384usize, FaultIP)
    }
    #[allow(dead_code)]
    pub const fn width_of_FaultIP() -> usize {
        384usize - 320usize
    }
    #[allow(dead_code)]
    pub fn get_Stack(&self) -> u64 {
        self.0.get_bits(256usize..320usize)
    }
    pub fn set_Stack(&mut self, Stack: u64) {
        self.0.set_bits(256usize..320usize, Stack)
    }
    #[allow(dead_code)]
    pub const fn width_of_Stack() -> usize {
        320usize - 256usize
    }
    #[allow(dead_code)]
    pub fn get_FLAGS(&self) -> u64 {
        self.0.get_bits(192usize..256usize)
    }
    pub fn set_FLAGS(&mut self, FLAGS: u64) {
        self.0.set_bits(192usize..256usize, FLAGS)
    }
    #[allow(dead_code)]
    pub const fn width_of_FLAGS() -> usize {
        256usize - 192usize
    }
    #[allow(dead_code)]
    pub fn get_Number(&self) -> u64 {
        self.0.get_bits(128usize..192usize)
    }
    pub fn set_Number(&mut self, Number: u64) {
        self.0.set_bits(128usize..192usize, Number)
    }
    #[allow(dead_code)]
    pub const fn width_of_Number() -> usize {
        192usize - 128usize
    }
    #[allow(dead_code)]
    pub fn get_Code(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_Code(&mut self, Code: u64) {
        self.0.set_bits(64usize..128usize, Code)
    }
    #[allow(dead_code)]
    pub const fn width_of_Code() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_UserException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_UserException_Unpacked {
    pub FaultIP: u64,
    pub Stack: u64,
    pub FLAGS: u64,
    pub Number: u64,
    pub Code: u64,
}
impl seL4_Fault_UserException_Unpacked {
    pub fn pack(self) -> seL4_Fault_UserException {
        match self {
            Self {
                FaultIP,
                Stack,
                FLAGS,
                Number,
                Code,
            } => seL4_Fault_UserException::new(FaultIP, Stack, FLAGS, Number, Code),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_VMFault(pub SeL4Bitfield<u64, 20usize>);
impl seL4_Fault_VMFault {
    pub fn new(IP: u64, Addr: u64, PrefetchFault: u64, FSR: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_IP(IP);
        this.set_Addr(Addr);
        this.set_PrefetchFault(PrefetchFault);
        this.set_FSR(FSR);
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_VMFault);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_VMFault_Unpacked {
        seL4_Fault_VMFault_Unpacked {
            IP: self.get_IP(),
            Addr: self.get_Addr(),
            PrefetchFault: self.get_PrefetchFault(),
            FSR: self.get_FSR(),
        }
    }
    #[allow(dead_code)]
    pub fn get_IP(&self) -> u64 {
        self.0.get_bits(256usize..320usize)
    }
    pub fn set_IP(&mut self, IP: u64) {
        self.0.set_bits(256usize..320usize, IP)
    }
    #[allow(dead_code)]
    pub const fn width_of_IP() -> usize {
        320usize - 256usize
    }
    #[allow(dead_code)]
    pub fn get_Addr(&self) -> u64 {
        self.0.get_bits(192usize..256usize)
    }
    pub fn set_Addr(&mut self, Addr: u64) {
        self.0.set_bits(192usize..256usize, Addr)
    }
    #[allow(dead_code)]
    pub const fn width_of_Addr() -> usize {
        256usize - 192usize
    }
    #[allow(dead_code)]
    pub fn get_PrefetchFault(&self) -> u64 {
        self.0.get_bits(128usize..192usize)
    }
    pub fn set_PrefetchFault(&mut self, PrefetchFault: u64) {
        self.0.set_bits(128usize..192usize, PrefetchFault)
    }
    #[allow(dead_code)]
    pub const fn width_of_PrefetchFault() -> usize {
        192usize - 128usize
    }
    #[allow(dead_code)]
    pub fn get_FSR(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_FSR(&mut self, FSR: u64) {
        self.0.set_bits(64usize..128usize, FSR)
    }
    #[allow(dead_code)]
    pub const fn width_of_FSR() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_VMFault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_VMFault_Unpacked {
    pub IP: u64,
    pub Addr: u64,
    pub PrefetchFault: u64,
    pub FSR: u64,
}
impl seL4_Fault_VMFault_Unpacked {
    pub fn pack(self) -> seL4_Fault_VMFault {
        match self {
            Self {
                IP,
                Addr,
                PrefetchFault,
                FSR,
            } => seL4_Fault_VMFault::new(IP, Addr, PrefetchFault, FSR),
        }
    }
}
pub mod seL4_Fault_tag {
    pub const seL4_Fault_NullFault: u64 = 0;
    pub const seL4_Fault_CapFault: u64 = 1;
    pub const seL4_Fault_UnknownSyscall: u64 = 2;
    pub const seL4_Fault_UserException: u64 = 3;
    pub const seL4_Fault_VMFault: u64 = 5;
}
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct seL4_Fault(pub SeL4Bitfield<u64, 20usize>);
impl seL4_Fault {
    pub fn splay(self) -> seL4_Fault_Splayed {
        match self.get_tag() {
            seL4_Fault_tag::seL4_Fault_NullFault => {
                seL4_Fault_Splayed::NullFault(seL4_Fault_NullFault(self.0))
            }
            seL4_Fault_tag::seL4_Fault_CapFault => {
                seL4_Fault_Splayed::CapFault(seL4_Fault_CapFault(self.0))
            }
            seL4_Fault_tag::seL4_Fault_UnknownSyscall => {
                seL4_Fault_Splayed::UnknownSyscall(seL4_Fault_UnknownSyscall(self.0))
            }
            seL4_Fault_tag::seL4_Fault_UserException => {
                seL4_Fault_Splayed::UserException(seL4_Fault_UserException(self.0))
            }
            seL4_Fault_tag::seL4_Fault_VMFault => {
                seL4_Fault_Splayed::VMFault(seL4_Fault_VMFault(self.0))
            }
            _ => panic!(),
        }
    }
    pub fn get_tag(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
}
impl fmt::Debug for seL4_Fault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.clone().splay().fmt(f)?;
        write!(f, ".unsplay()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum seL4_Fault_Splayed {
    NullFault(seL4_Fault_NullFault),
    CapFault(seL4_Fault_CapFault),
    UnknownSyscall(seL4_Fault_UnknownSyscall),
    UserException(seL4_Fault_UserException),
    VMFault(seL4_Fault_VMFault),
}
impl seL4_Fault_Splayed {
    pub fn unsplay(self) -> seL4_Fault {
        match self {
            seL4_Fault_Splayed::NullFault(seL4_Fault_NullFault(bitfield)) => seL4_Fault(bitfield),
            seL4_Fault_Splayed::CapFault(seL4_Fault_CapFault(bitfield)) => seL4_Fault(bitfield),
            seL4_Fault_Splayed::UnknownSyscall(seL4_Fault_UnknownSyscall(bitfield)) => {
                seL4_Fault(bitfield)
            }
            seL4_Fault_Splayed::UserException(seL4_Fault_UserException(bitfield)) => {
                seL4_Fault(bitfield)
            }
            seL4_Fault_Splayed::VMFault(seL4_Fault_VMFault(bitfield)) => seL4_Fault(bitfield),
        }
    }
}
impl seL4_Fault_NullFault {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_NullFault_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
impl seL4_Fault_CapFault {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_CapFault_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
impl seL4_Fault_UnknownSyscall {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_UnknownSyscall_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
impl seL4_Fault_UserException {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_UserException_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
impl seL4_Fault_VMFault {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_VMFault_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
