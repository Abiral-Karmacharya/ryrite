.intel_syntax noprefix
.global _start

.section .data
msg:
    .ascii "Hello, World!\n"   # Data definition in GAS

.section .text
_start:
    mov rax, 1        # syscall: write
    mov rdi, 1        # stdout
    mov rsi, offset msg # string address
    mov rdx, 14       # length
    syscall

    mov rax, 60       # syscall: exit
    mov rdi, 0        # exit code 0
    syscall
