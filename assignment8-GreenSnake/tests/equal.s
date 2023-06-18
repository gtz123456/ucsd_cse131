
section .text
extern snek_error
extern snek_print
extern snek_structural_equal
global our_code_starts_here

our_code_starts_here:
  
mov [rsp-16], rdi
mov r15, rsi
mov rax, 2
mov [rsp-24], rax
mov rax, 2
mov [rsp-32], rax
mov rax, 2
mov [rsp-40], rax
mov rax, 4
mov [rsp-48], rax
mov rax, 16
mov [r15+0], rax
mov rbx, [rsp-40]
mov [r15+8], rbx
mov rbx, [rsp-48]
mov [r15+16], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 24
mov [rsp-40], rax
mov rax, 2
mov [rsp-48], rax
mov rax, 4
mov [rsp-56], rax
mov rax, 16
mov [r15+0], rax
mov rbx, [rsp-48]
mov [r15+8], rbx
mov rbx, [rsp-56]
mov [r15+16], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 24
mov [rsp-48], rax
mov rax, 2
mov [rsp-56], rax
mov rax, 4
mov [rsp-64], rax
mov rax, 16
mov [r15+0], rax
mov rbx, [rsp-56]
mov [r15+8], rbx
mov rbx, [rsp-64]
mov [r15+16], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 24
mov [rsp-56], rax
mov rax, 2
mov [rsp-64], rax
mov rax, 4
mov [rsp-72], rax
mov rax, 6
mov [rsp-80], rax
mov rax, 24
mov [r15+0], rax
mov rbx, [rsp-64]
mov [r15+8], rbx
mov rbx, [rsp-72]
mov [r15+16], rbx
mov rbx, [rsp-80]
mov [r15+24], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 32
mov [rsp-64], rax
mov rax, 2
mov [rsp-72], rax
mov rax, 4
mov [rsp-80], rax
mov rax, 16
mov [r15+0], rax
mov rbx, [rsp-72]
mov [r15+8], rbx
mov rbx, [rsp-80]
mov [r15+16], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 24
mov [rsp-72], rax
mov rax, 2
mov [rsp-80], rax
mov rax, 4
mov [rsp-88], rax
mov rax, 16
mov [r15+0], rax
mov rbx, [rsp-80]
mov [r15+8], rbx
mov rbx, [rsp-88]
mov [r15+16], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 24
mov [rsp-80], rax
mov rax, 2
shl rax, 2
mov [rsp-88], rax
mov rax, [rsp-40]
mov [rsp-96], rax
mov rax, [rsp-56]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-88]
mov rdi, 3
jle snek_error
mov rbx, [rsp-88]
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-88]
add rbx, rax
mov rax, [rsp-96]
mov [rbx+0], rax
mov rax, 2
shl rax, 2
mov [rsp-88], rax
mov rax, [rsp-48]
mov [rsp-96], rax
mov rax, [rsp-64]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-88]
mov rdi, 3
jle snek_error
mov rbx, [rsp-88]
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-88]
add rbx, rax
mov rax, [rsp-96]
mov [rbx+0], rax
mov rax, 2
shl rax, 2
mov [rsp-88], rax
mov rax, [rsp-72]
mov [rsp-96], rax
mov rax, [rsp-72]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-88]
mov rdi, 3
jle snek_error
mov rbx, [rsp-88]
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-88]
add rbx, rax
mov rax, [rsp-96]
mov [rbx+0], rax
mov rax, 2
shl rax, 2
mov [rsp-88], rax
mov rax, [rsp-80]
mov [rsp-96], rax
mov rax, [rsp-80]
mov rbx, rax
and rbx, 3
cmp rbx, 1
mov rdi, 2
jne snek_error
shr rax, 2
mov rbx, [rax+0]
cmp rbx, [rsp-88]
mov rdi, 3
jle snek_error
mov rbx, [rsp-88]
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-88]
add rbx, rax
mov rax, [rsp-96]
mov [rbx+0], rax
mov rax, [rsp-32]
mov [rsp-88], rax
mov rax, [rsp-24]
mov [rsp-96], rax
xor rax, [rsp-88]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-96]
cmp rax, [rsp-88]
mov rbx, 7
cmove rax, rbx
mov rbx, 3
cmovne rax, rbx
mov [rsp-88], rax
mov rdi, rax
sub rsp, 88
call snek_print
add rsp, 88
mov rax, [rsp-88]
mov rax, [rsp-48]
mov [rsp-88], rax
mov rax, [rsp-40]
mov [rsp-96], rax
xor rax, [rsp-88]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-96]
cmp rax, [rsp-88]
mov rbx, 7
cmove rax, rbx
mov rbx, 3
cmovne rax, rbx
mov [rsp-88], rax
mov rdi, rax
sub rsp, 88
call snek_print
add rsp, 88
mov rax, [rsp-88]
mov rax, [rsp-64]
mov [rsp-88], rax
mov rax, [rsp-56]
mov [rsp-96], rax
xor rax, [rsp-88]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-96]
cmp rax, [rsp-88]
mov rbx, 7
cmove rax, rbx
mov rbx, 3
cmovne rax, rbx
mov [rsp-88], rax
mov rdi, rax
sub rsp, 88
call snek_print
add rsp, 88
mov rax, [rsp-88]
mov rax, [rsp-80]
mov [rsp-88], rax
mov rax, [rsp-72]
mov [rsp-96], rax
xor rax, [rsp-88]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-96]
cmp rax, [rsp-88]
mov rbx, 7
cmove rax, rbx
mov rbx, 3
cmovne rax, rbx
mov [rsp-88], rax
mov rdi, rax
sub rsp, 88
call snek_print
add rsp, 88
mov rax, [rsp-88]
mov rax, [rsp-32]
mov [rsp-88], rax
mov rax, [rsp-24]
mov rdi, rax
mov rsi, [rsp-88]
sub rsp, 88
call snek_structural_equal
add rsp, 88
mov [rsp-88], rax
mov rdi, rax
sub rsp, 88
call snek_print
add rsp, 88
mov rax, [rsp-88]
mov rax, [rsp-48]
mov [rsp-88], rax
mov rax, [rsp-40]
mov rdi, rax
mov rsi, [rsp-88]
sub rsp, 88
call snek_structural_equal
add rsp, 88
mov [rsp-88], rax
mov rdi, rax
sub rsp, 88
call snek_print
add rsp, 88
mov rax, [rsp-88]
mov rax, [rsp-64]
mov [rsp-88], rax
mov rax, [rsp-56]
mov rdi, rax
mov rsi, [rsp-88]
sub rsp, 88
call snek_structural_equal
add rsp, 88
mov [rsp-88], rax
mov rdi, rax
sub rsp, 88
call snek_print
add rsp, 88
mov rax, [rsp-88]
mov rax, [rsp-80]
mov [rsp-88], rax
mov rax, [rsp-72]
mov rdi, rax
mov rsi, [rsp-88]
sub rsp, 88
call snek_structural_equal
add rsp, 88
mov [rsp-88], rax
mov rdi, rax
sub rsp, 88
call snek_print
add rsp, 88
mov rax, [rsp-88]
  ret
