.global _start

.section .text

_start:
    mov $0x1, %rax    
    mov $0x1, %rdi    
    lea msg, %rsi    
    mov $0xd, %rdx   
    syscall

    mov $0x60, %rax   
    xor %rdi, %rdi   
    syscall

.section .rodata
    msg: .asciz "Hello, world!\n"
