.section .data
.global isr_table

isr_table:
    .quad isr0
    .quad isr1
    .quad isr2
    .quad isr3
    .quad isr4
    .quad isr5
    .quad isr6
    .quad isr7
    .quad isr8
    .quad isr9
    .quad isr10
    .quad isr11
    .quad isr12
    .quad isr13
    .quad isr14
    .quad isr15
    .quad isr16
    .quad isr17
    .quad isr18
    .quad isr19
    .quad isr20
    .quad isr21
    .quad isr22
    .quad isr23
    .quad isr24
    .quad isr25
    .quad isr26
    .quad isr27
    .quad isr28
    .quad isr29
    .quad isr30
    .quad isr31

.macro ISR_NOERR n
.global isr\n
isr\n:
    pushq $0
    pushq $\n
    jmp isr_common
.endm

.macro ISR_ERR n
.global isr\n
isr\n:
    pushq $\n
    jmp isr_common
.endm

.section .text

ISR_NOERR 0
ISR_NOERR 1
ISR_NOERR 2
ISR_NOERR 3
ISR_NOERR 4
ISR_NOERR 5
ISR_NOERR 6
ISR_NOERR 7
ISR_ERR   8
ISR_NOERR 9
ISR_ERR   10
ISR_ERR   11
ISR_ERR   12
ISR_ERR   13
ISR_ERR   14
ISR_NOERR 15
ISR_NOERR 16
ISR_ERR   17
ISR_NOERR 18
ISR_NOERR 19
ISR_NOERR 20
ISR_ERR   21
ISR_NOERR 22
ISR_NOERR 23
ISR_NOERR 24
ISR_NOERR 25
ISR_NOERR 26
ISR_NOERR 27
ISR_NOERR 28
ISR_ERR   29
ISR_ERR   30
ISR_NOERR 31

isr_common:
    pushq %rax
    pushq %rbx
    pushq %rcx
    pushq %rdx
    pushq %rsi
    pushq %rdi
    pushq %rbp
    pushq %r8
    pushq %r9
    pushq %r10
    pushq %r11
    pushq %r12
    pushq %r13
    pushq %r14
    pushq %r15

    movq %rsp, %rdi
    call isr_handler

    popq %r15
    popq %r14
    popq %r13
    popq %r12
    popq %r11
    popq %r10
    popq %r9
    popq %r8
    popq %rbp
    popq %rdi
    popq %rsi
    popq %rdx
    popq %rcx
    popq %rbx
    popq %rax

    addq $16, %rsp
    iretq
