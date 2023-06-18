
section .text
extern snek_error
extern snek_print
global our_code_starts_here

our_code_starts_here:
  
mov [rsp-16], rdi
mov r15, rsi
loopstart1:
mov rax, 0
mov [rsp-24], rax
mov rax, 20
mov [rsp-32], rax
mov rax, [rsp-24]
mov [rsp-40], rax
xor rax, [rsp-32]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-40]
cmp rax, [rsp-32]
mov rbx, 7
cmove rax, rbx
mov rbx, 3
cmovne rax, rbx
cmp rax, 3
je elsestart1
mov rax, 0
jmp loopend1
jmp elseend1
elsestart1:
mov rax, [rsp-24]
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
elseend1:
jmp loopstart1
loopend1:
  ret
