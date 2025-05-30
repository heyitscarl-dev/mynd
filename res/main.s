
.intel_syntax noprefix
.section .bss
    .lcomm dat, 30000

.section .text
.global _start

_start:
    lea rsi, [dat]
add byte ptr [rsi], 1
l1s:
cmp byte ptr [rsi], 0
je l1e
sub byte ptr [rsi], 5
add rsi, 1
add byte ptr [rsi], 3
sub rsi, 1
jmp l1s
l1e:
add rsi, 1
add byte ptr [rsi], 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 3
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 7
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 3
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
l2s:
cmp byte ptr [rsi], 0
je l2e
sub byte ptr [rsi], 1
add rsi, 1
add byte ptr [rsi], 5
sub rsi, 1
jmp l2s
l2e:
add rsi, 1
add byte ptr [rsi], 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 12
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
l3s:
cmp byte ptr [rsi], 0
je l3e
sub byte ptr [rsi], 1
add rsi, 1
add byte ptr [rsi], 3
sub rsi, 1
jmp l3s
l3e:
add rsi, 1
add byte ptr [rsi], 2
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
l4s:
cmp byte ptr [rsi], 0
je l4e
sub byte ptr [rsi], 3
add rsi, 1
add byte ptr [rsi], 1
sub rsi, 1
jmp l4s
l4e:
add rsi, 1
sub byte ptr [rsi], 4
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 3
l5s:
cmp byte ptr [rsi], 0
je l5e
sub byte ptr [rsi], 1
add rsi, 1
add byte ptr [rsi], 3
sub rsi, 1
jmp l5s
l5e:
add rsi, 1
add byte ptr [rsi], 2
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 8
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 5
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 8
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 3
l6s:
cmp byte ptr [rsi], 0
je l6e
sub byte ptr [rsi], 1
add rsi, 1
add byte ptr [rsi], 3
sub rsi, 1
jmp l6s
l6e:
add rsi, 1
add byte ptr [rsi], 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 1
l7s:
cmp byte ptr [rsi], 0
je l7e
sub byte ptr [rsi], 3
add rsi, 1
add byte ptr [rsi], 1
sub rsi, 1
jmp l7s
l7e:
add rsi, 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 8
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 1
l8s:
cmp byte ptr [rsi], 0
je l8e
add byte ptr [rsi], 2
add rsi, 1
sub byte ptr [rsi], 3
sub rsi, 1
jmp l8s
l8e:
add rsi, 1
sub byte ptr [rsi], 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 8
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 2
l9s:
cmp byte ptr [rsi], 0
je l9e
sub byte ptr [rsi], 3
add rsi, 1
add byte ptr [rsi], 1
sub rsi, 1
jmp l9s
l9e:
add rsi, 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
l10s:
cmp byte ptr [rsi], 0
je l10e
sub byte ptr [rsi], 3
add rsi, 1
add byte ptr [rsi], 1
sub rsi, 1
jmp l10s
l10e:
add rsi, 1
add byte ptr [rsi], 3
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 1
l11s:
cmp byte ptr [rsi], 0
je l11e
sub byte ptr [rsi], 4
add rsi, 1
add byte ptr [rsi], 1
sub rsi, 1
jmp l11s
l11e:
add rsi, 1
add byte ptr [rsi], 2
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 1
l12s:
cmp byte ptr [rsi], 0
je l12e
sub byte ptr [rsi], 3
add rsi, 1
add byte ptr [rsi], 2
sub rsi, 1
jmp l12s
l12e:
add rsi, 1
sub byte ptr [rsi], 2
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 3
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
l13s:
cmp byte ptr [rsi], 0
je l13e
sub byte ptr [rsi], 3
add rsi, 1
add byte ptr [rsi], 1
sub rsi, 1
jmp l13s
l13e:
add rsi, 1
add byte ptr [rsi], 2
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
l14s:
cmp byte ptr [rsi], 0
je l14e
sub byte ptr [rsi], 1
add rsi, 1
add byte ptr [rsi], 3
sub rsi, 1
jmp l14s
l14e:
add rsi, 1
sub byte ptr [rsi], 2
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 11
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
add byte ptr [rsi], 3
l15s:
cmp byte ptr [rsi], 0
je l15e
sub byte ptr [rsi], 1
add rsi, 1
add byte ptr [rsi], 3
sub rsi, 1
jmp l15s
l15e:
add rsi, 1
add byte ptr [rsi], 1
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 2
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 2
l16s:
cmp byte ptr [rsi], 0
je l16e
sub byte ptr [rsi], 3
add rsi, 1
add byte ptr [rsi], 1
sub rsi, 1
jmp l16s
l16e:
add rsi, 1
sub byte ptr [rsi], 3
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
sub byte ptr [rsi], 6
mov rax, 1
mov rdi, 1
mov rsi, rsi
mov rdx, 1
syscall
l17s:
cmp byte ptr [rsi], 0
je l17e
sub byte ptr [rsi], 3
add rsi, 1
add byte ptr [rsi], 1
sub rsi, 1
jmp l17s
l17e:
add rsi, 1
add byte ptr [rsi], 5
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

