
section .text
extern snek_error
global our_code_starts_here
our_code_starts_here:
  fact:

mov rax, 2
mov [rsp-8], rax
mov rax, 2
mov [rsp-16], rax
loopstart1:
mov rax, [rsp--8]
mov [rsp-24], rax
mov rax, [rsp-8]
mov [rsp-32], rax
or rax, [rsp-24]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-32]
cmp rax, [rsp-24]
mov rbx, 3
cmovg rax, rbx
mov rbx, 1
cmovle rax, rbx
cmp rax, 1
je elsestart1
mov rax, [rsp-16]
jmp loopend1
jmp elseend1
elsestart1:
mov rax, [rsp-8]
mov [rsp-24], rax
mov rax, [rsp-16]
mov [rsp-32], rax
or rax, [rsp-24]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-32]
sar rax, 1
imul rax, [rsp-24]
mov rdi, 1
jo snek_error
mov [rsp-16], rax
mov rax, 2
mov [rsp-24], rax
mov rax, [rsp-8]
mov [rsp-32], rax
or rax, [rsp-24]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-32]
add rax, [rsp-24]
mov rdi, 1
jo snek_error
mov [rsp-8], rax
elseend1:
jmp loopstart1
loopend1:ret

mov [rsp-16], rdi
mov rax, [rsp-16]
mov [rsp+3], rax
sub rsp, 1
mov rbx, [rsp+1]
mov [rsp+0], rbx
mov [rsp+2], rdi
call fact
mov rdi, [rsp+2]
add rsp, 1
  ret
