
section .text
extern snek_error
global our_code_starts_here
r:

mov rax, [rsp+8]
ret
our_code_starts_here:
  
mov [rsp-16], rdi
mov rax, 6
mov [rsp+24], rax
sub rsp, 8
mov rbx, [rsp+8]
mov [rsp+0], rbx
mov [rsp+16], rdi
call r
mov rdi, [rsp+16]
add rsp, 8
  ret
