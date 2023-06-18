
section .text
extern snek_error
extern snek_print
global our_code_starts_here

our_code_starts_here:
  
mov [rsp-16], rdi
mov r15, rsi
mov rax, 0
mov [rsp-24], rax
mov rax, 0
mov [rsp-32], rax
mov rax, 2
mov [rsp-40], rax
mov rax, 4
mov [rsp-48], rax
mov rax, 6
mov [rsp-56], rax
mov rax, 8
mov [rsp-64], rax
mov rax, 40
mov [r15+0], rax
mov rbx, [rsp-32]
mov [r15+8], rbx
mov rbx, [rsp-40]
mov [r15+16], rbx
mov rbx, [rsp-48]
mov [r15+24], rbx
mov rbx, [rsp-56]
mov [r15+32], rbx
mov rbx, [rsp-64]
mov [r15+40], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 48
mov [rsp-32], rax
mov rax, 0
shl rax, 2
mov [rsp-40], rax
mov rax, [rsp-32]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-40]
mov rdi, 3
jle snek_error
cmp rbx, 0
mov rbx, [rsp-40]
test rbx, 9223372036854775809
mov rdi, 4
jnz snek_error
add rax, 8
mov rbx, [rsp-40]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-40], rax
mov rdi, rax
sub rsp, 40
call snek_print
add rsp, 40
mov rax, [rsp-40]
mov rax, 8
shl rax, 2
mov [rsp-40], rax
mov rax, [rsp-32]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-40]
mov rdi, 3
jle snek_error
cmp rbx, 0
mov rbx, [rsp-40]
test rbx, 9223372036854775809
mov rdi, 4
jnz snek_error
add rax, 8
mov rbx, [rsp-40]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-40], rax
mov rdi, rax
sub rsp, 40
call snek_print
add rsp, 40
mov rax, [rsp-40]
mov rax, 4
shl rax, 2
mov [rsp-40], rax
mov rax, [rsp-32]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-40]
mov rdi, 3
jle snek_error
cmp rbx, 0
mov rbx, [rsp-40]
test rbx, 9223372036854775809
mov rdi, 4
jnz snek_error
add rax, 8
mov rbx, [rsp-40]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-40], rax
mov rax, [rsp-24]
mov [rsp-48], rax
or rax, [rsp-40]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-48]
cmp rax, [rsp-40]
mov rbx, 7
cmovg rax, rbx
mov rbx, 3
cmovle rax, rbx
cmp rax, 3
je elsestart1
mov rax, 7
jmp elseend1
elsestart1:
mov rax, 3
elseend1:
mov [rsp-40], rax
mov rdi, rax
sub rsp, 40
call snek_print
add rsp, 40
mov rax, [rsp-40]
loopstart1:
mov rax, 8
shl rax, 2
mov [rsp-40], rax
mov rax, [rsp-32]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-40]
mov rdi, 3
jle snek_error
cmp rbx, 0
mov rbx, [rsp-40]
test rbx, 9223372036854775809
mov rdi, 4
jnz snek_error
add rax, 8
mov rbx, [rsp-40]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-40], rax
mov rax, [rsp-24]
mov [rsp-48], rax
or rax, [rsp-40]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-48]
cmp rax, [rsp-40]
mov rbx, 7
cmovg rax, rbx
mov rbx, 3
cmovle rax, rbx
cmp rax, 3
je elsestart2
mov rax, [rsp-24]
jmp loopend1
jmp elseend2
elsestart2:
mov rax, [rsp-24]
mov [rsp-48], rax
and rax, 1
cmp rax, 0
mov rdi, 0
jne snek_error
mov rax, [rsp-48]
add rax, 2
mov rdi, 1
jo snek_error
mov [rsp-24], rax
elseend2:
jmp loopstart1
loopend1:
  ret
