// Extend the reserved region down to include the base of the kernel image.
// KERNEL_ELF_PADDR_BASE is the lowest physical load address used
// in the x86 linker script.

#[cfg(target_arch = "x86_64")]
pub fn init() {
// Extend the reserved region down to include the base of the kernel image.
// KERNEL_ELF_PADDR_BASE is the lowest physical load address used
// in the x86 linker script.
}


 fn init_freemem() {}
