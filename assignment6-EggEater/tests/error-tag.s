
section .text
extern snek_error
extern snek_print
global our_code_starts_here

our_code_starts_here:
  
mov [rsp-16], rdi
mov r15, rsi
mov rax, 0
shl rax, 2
mov [rsp-24], rax
mov rax, 3
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-24]
mov rdi, 3
jle snek_error
cmp rbx, 0
mov rbx, [rsp-24]
cmp rbx, 0
mov rdi, 4
jl snek_error
add rax, 8
add rbx, rax
mov rax, [rbx+0]
  ret
