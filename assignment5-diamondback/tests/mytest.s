
section .text
extern snek_error
extern snek_print
global our_code_starts_here

fun1:
mov rax, [rsp+16]
mov [rsp-24], rax
mov rax, [rsp+8]
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
ret
our_code_starts_here:
  
mov [rsp-16], rdi
mov rax, 0
mov [rsp-24], rax
loopstart1:
mov rax, [rsp-16]
mov [rsp-32], rax
mov rax, [rsp-24]
mov [rsp-40], rax
or rax, [rsp-32]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-40]
cmp rax, [rsp-32]
mov rbx, 3
cmovg rax, rbx
mov rbx, 1
cmovle rax, rbx
cmp rax, 1
je elsestart1
mov rax, 0
jmp loopend1
jmp elseend1
elsestart1:
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, [rsp-16]
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call fun1
mov rdi, [rsp+24]
add rsp, 56
mov [rsp-40], rax
mov rdi, rax
sub rsp, 40
call snek_print
add rsp, 40
mov rax, [rsp-40]
mov rax, [rsp-24]
mov [rsp-40], rax
and rax, 1
cmp rax, 0
mov rdi, 0
jne snek_error
mov rax, [rsp-40]
add rax, 2
mov rdi, 1
jo snek_error
mov [rsp-24], rax
elseend1:
jmp loopstart1
loopend1:
  ret
