
.intel_syntax noprefix
.section .bss
    .lcomm dat, 30000

.section .text
.global _start

_start:
    lea rsi, [dat]
sub byte ptr [rsi], 1
l1s:
cmp byte ptr [rsi], 0
je l1e
sub byte ptr [rsi], 7
add rsi, 1
add byte ptr [rsi], 1
sub rsi, 1
jmp l1s
l1e:
add rsi, 1
sub byte ptr [rsi], 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 2
l2s:
cmp byte ptr [rsi], 0
je l2e
sub byte ptr [rsi], 2
add rsi, 1
add byte ptr [rsi], 3
sub rsi, 1
jmp l2s
l2e:
add rsi, 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add rsi, 1
add byte ptr [rsi], 10
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall

    // sys_exit
    mov rax, 60
    mov rdi, 69
    syscall

