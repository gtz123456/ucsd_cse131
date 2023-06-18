
section .text
extern snek_error
extern snek_print
global our_code_starts_here

generate_tuple:
mov rax, [rsp+8]
mov [rsp-24], rax
mov rax, [rsp+16]
mov [rsp-32], rax
mov rax, 16
mov [r15+0], rax
mov rbx, [rsp-24]
mov [r15+8], rbx
mov rbx, [rsp-32]
mov [r15+16], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 24
ret
add_tuple:
mov rax, 0
shl rax, 2
mov [rsp-24], rax
mov rax, [rsp+16]
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
mov rbx, [rsp-24]
test rbx, 17293822569102704647
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-24]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-24], rax
mov rax, 0
shl rax, 2
mov [rsp-32], rax
mov rax, [rsp+8]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-32]
mov rdi, 3
jle snek_error
mov rbx, [rsp-32]
test rbx, 17293822569102704647
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-32]
add rbx, rax
mov rax, [rbx+0]
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
mov [rsp-24], rax
mov rax, 2
shl rax, 2
mov [rsp-32], rax
mov rax, [rsp+16]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-32]
mov rdi, 3
jle snek_error
mov rbx, [rsp-32]
test rbx, 17293822569102704647
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-32]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-32], rax
mov rax, 2
shl rax, 2
mov [rsp-40], rax
mov rax, [rsp+8]
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
mov rbx, [rsp-40]
test rbx, 17293822569102704647
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-40]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-40], rax
or rax, [rsp-32]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-40]
add rax, [rsp-32]
mov rdi, 1
jo snek_error
mov [rsp-32], rax
mov rax, 16
mov [r15+0], rax
mov rbx, [rsp-24]
mov [r15+8], rbx
mov rbx, [rsp-32]
mov [r15+16], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 24
ret
our_code_starts_here:
  
mov [rsp-16], rdi
mov r15, rsi
mov rax, 2
mov [rsp-24], rax
mov rax, 4
mov [rsp-32], rax
sub rsp, 56
mov rbx, [rsp+32]
mov [rsp+0], rbx
mov rbx, [rsp+24]
mov [rsp+8], rbx
mov [rsp+32], rdi
call generate_tuple
mov rdi, [rsp+32]
add rsp, 56
mov [rsp-24], rax
mov rax, 6
mov [rsp-32], rax
mov rax, 8
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call generate_tuple
mov rdi, [rsp+24]
add rsp, 56
mov [rsp-32], rax
sub rsp, 56
mov rbx, [rsp+32]
mov [rsp+0], rbx
mov rbx, [rsp+24]
mov [rsp+8], rbx
mov [rsp+32], rdi
call add_tuple
mov rdi, [rsp+32]
add rsp, 56
mov [rsp-24], rax
mov rdi, rax
sub rsp, 24
call snek_print
add rsp, 24
mov rax, [rsp-24]
mov rax, 20
mov [rsp-24], rax
mov rax, 24
mov [rsp-32], rax
mov rax, 16
mov [r15+0], rax
mov rbx, [rsp-24]
mov [r15+8], rbx
mov rbx, [rsp-32]
mov [r15+16], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 24
mov [rsp-24], rax
mov rax, 6
mov [rsp-32], rax
mov rax, 8
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call generate_tuple
mov rdi, [rsp+24]
add rsp, 56
mov [rsp-32], rax
sub rsp, 56
mov rbx, [rsp+32]
mov [rsp+0], rbx
mov rbx, [rsp+24]
mov [rsp+8], rbx
mov [rsp+32], rdi
call add_tuple
mov rdi, [rsp+32]
add rsp, 56
mov [rsp-24], rax
mov rdi, rax
sub rsp, 24
call snek_print
add rsp, 24
mov rax, [rsp-24]
mov rax, 2
mov [rsp-24], rax
mov rax, 4
mov [rsp-32], rax
sub rsp, 56
mov rbx, [rsp+32]
mov [rsp+0], rbx
mov rbx, [rsp+24]
mov [rsp+8], rbx
mov [rsp+32], rdi
call generate_tuple
mov rdi, [rsp+32]
add rsp, 56
mov [rsp-24], rax
mov rax, 14
mov [rsp-32], rax
mov rax, 20
mov [rsp-40], rax
mov rax, 16
mov [r15+0], rax
mov rbx, [rsp-32]
mov [r15+8], rbx
mov rbx, [rsp-40]
mov [r15+16], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 24
mov [rsp-32], rax
sub rsp, 56
mov rbx, [rsp+32]
mov [rsp+0], rbx
mov rbx, [rsp+24]
mov [rsp+8], rbx
mov [rsp+32], rdi
call add_tuple
mov rdi, [rsp+32]
add rsp, 56
  ret
