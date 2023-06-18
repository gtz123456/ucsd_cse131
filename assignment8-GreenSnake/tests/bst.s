
section .text
extern snek_error
extern snek_print
extern snek_structural_equal
global our_code_starts_here

generate_node:
mov rax, [rsp+8]
mov [rsp-32], rax
mov rax, [rsp+16]
mov [rsp-40], rax
mov rax, [rsp+24]
mov [rsp-48], rax
mov rax, 24
mov [r15+0], rax
mov rbx, [rsp-32]
mov [r15+8], rbx
mov rbx, [rsp-40]
mov [r15+16], rbx
mov rbx, [rsp-48]
mov [r15+24], rbx
mov rax, r15
shl rax, 2
add rax, 1
add r15, 32
ret
element_in_the_tree:
mov rax, [rsp+8]
mov [rsp-24], rax
loopstart1:
mov rax, 0
shl rax, 2
mov [rsp-32], rax
mov rax, [rsp-24]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-32]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-32], rax
mov rax, [rsp+16]
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
mov rax, 7
jmp loopend1
jmp elseend1
elsestart1:
mov rax, 0
shl rax, 2
mov [rsp-32], rax
mov rax, [rsp-24]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-32]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-32], rax
mov rax, [rsp+16]
mov [rsp-40], rax
or rax, [rsp-32]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-40]
cmp rax, [rsp-32]
mov rbx, 7
cmovg rax, rbx
mov rbx, 3
cmovle rax, rbx
cmp rax, 3
je elsestart2
mov rax, 4
shl rax, 2
mov [rsp-32], rax
mov rax, [rsp-24]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-32]
add rbx, rax
mov rax, [rbx+0]
cmp rax, 3
je elsestart3
mov rax, 4
shl rax, 2
mov [rsp-32], rax
mov rax, [rsp-24]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-32]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-24], rax
jmp elseend3
elsestart3:
mov rax, 3
jmp loopend1
elseend3:
jmp elseend2
elsestart2:
mov rax, 2
shl rax, 2
mov [rsp-32], rax
mov rax, [rsp-24]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-32]
add rbx, rax
mov rax, [rbx+0]
cmp rax, 3
je elsestart4
mov rax, 2
shl rax, 2
mov [rsp-32], rax
mov rax, [rsp-24]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-32]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-24], rax
jmp elseend4
elsestart4:
mov rax, 3
jmp loopend1
elseend4:
elseend2:
elseend1:
jmp loopstart1
loopend1:
ret
add:
mov rax, 0
shl rax, 2
mov [rsp-24], rax
mov rax, [rsp+8]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-24]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-24], rax
mov rax, [rsp+16]
mov [rsp-32], rax
or rax, [rsp-24]
and rax, 1
cmp rax, 1
mov rdi, 0
je snek_error
mov rax, [rsp-32]
cmp rax, [rsp-24]
mov rbx, 7
cmovg rax, rbx
mov rbx, 3
cmovle rax, rbx
cmp rax, 3
je elsestart5
mov rax, 4
shl rax, 2
mov [rsp-24], rax
mov rax, [rsp+8]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-24]
add rbx, rax
mov rax, [rbx+0]
cmp rax, 3
je elsestart6
mov rax, 4
shl rax, 2
mov [rsp-24], rax
mov rax, [rsp+8]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-24]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-24], rax
mov rax, [rsp+16]
mov [rsp-32], rax
sub rsp, 56
mov rbx, [rsp+32]
mov [rsp+0], rbx
mov rbx, [rsp+24]
mov [rsp+8], rbx
mov [rsp+32], rdi
call add
mov rdi, [rsp+32]
add rsp, 56
jmp elseend6
elsestart6:
mov rax, 4
shl rax, 2
mov [rsp-24], rax
mov rax, [rsp+16]
mov [rsp-32], rax
mov rax, 3
mov [rsp-40], rax
mov rax, 3
mov [rsp-48], rax
sub rsp, 72
mov rbx, [rsp+40]
mov [rsp+0], rbx
mov rbx, [rsp+32]
mov [rsp+8], rbx
mov rbx, [rsp+24]
mov [rsp+16], rbx
mov [rsp+40], rdi
call generate_node
mov rdi, [rsp+40]
add rsp, 72
mov [rsp-32], rax
mov rax, [rsp+8]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-24]
add rbx, rax
mov rax, [rsp-32]
mov [rbx+0], rax
elseend6:
jmp elseend5
elsestart5:
mov rax, 2
shl rax, 2
mov [rsp-24], rax
mov rax, [rsp+8]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-24]
add rbx, rax
mov rax, [rbx+0]
cmp rax, 3
je elsestart7
mov rax, 2
shl rax, 2
mov [rsp-24], rax
mov rax, [rsp+8]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-24]
add rbx, rax
mov rax, [rbx+0]
mov [rsp-24], rax
mov rax, [rsp+16]
mov [rsp-32], rax
sub rsp, 56
mov rbx, [rsp+32]
mov [rsp+0], rbx
mov rbx, [rsp+24]
mov [rsp+8], rbx
mov [rsp+32], rdi
call add
mov rdi, [rsp+32]
add rsp, 56
jmp elseend7
elsestart7:
mov rax, 2
shl rax, 2
mov [rsp-24], rax
mov rax, [rsp+16]
mov [rsp-32], rax
mov rax, 3
mov [rsp-40], rax
mov rax, 3
mov [rsp-48], rax
sub rsp, 72
mov rbx, [rsp+40]
mov [rsp+0], rbx
mov rbx, [rsp+32]
mov [rsp+8], rbx
mov rbx, [rsp+24]
mov [rsp+16], rbx
mov [rsp+40], rdi
call generate_node
mov rdi, [rsp+40]
add rsp, 72
mov [rsp-32], rax
mov rax, [rsp+8]
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
test rbx, 1
mov rdi, 4
jnz snek_error
cmp rbx, 0
jl snek_error
add rax, 8
mov rbx, [rsp-24]
add rbx, rax
mov rax, [rsp-32]
mov [rbx+0], rax
elseend7:
elseend5:
ret
our_code_starts_here:
  
mov [rsp-16], rdi
mov r15, rsi
mov rax, 24
mov [rsp-24], rax
mov rax, 3
mov [rsp-32], rax
mov rax, 3
mov [rsp-40], rax
sub rsp, 72
mov rbx, [rsp+48]
mov [rsp+0], rbx
mov rbx, [rsp+40]
mov [rsp+8], rbx
mov rbx, [rsp+32]
mov [rsp+16], rbx
mov [rsp+48], rdi
call generate_node
mov rdi, [rsp+48]
add rsp, 72
mov [rsp-24], rax
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 18
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call add
mov rdi, [rsp+24]
add rsp, 56
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 10
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call add
mov rdi, [rsp+24]
add rsp, 56
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 30
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call add
mov rdi, [rsp+24]
add rsp, 56
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 36
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call add
mov rdi, [rsp+24]
add rsp, 56
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 34
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call add
mov rdi, [rsp+24]
add rsp, 56
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 10
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call element_in_the_tree
mov rdi, [rsp+24]
add rsp, 56
mov [rsp-40], rax
mov rdi, rax
sub rsp, 40
call snek_print
add rsp, 40
mov rax, [rsp-40]
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 18
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call element_in_the_tree
mov rdi, [rsp+24]
add rsp, 56
mov [rsp-40], rax
mov rdi, rax
sub rsp, 40
call snek_print
add rsp, 40
mov rax, [rsp-40]
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 34
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call element_in_the_tree
mov rdi, [rsp+24]
add rsp, 56
mov [rsp-40], rax
mov rdi, rax
sub rsp, 40
call snek_print
add rsp, 40
mov rax, [rsp-40]
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 1998
mov [rsp-40], rax
sub rsp, 56
mov rbx, [rsp+24]
mov [rsp+0], rbx
mov rbx, [rsp+16]
mov [rsp+8], rbx
mov [rsp+24], rdi
call element_in_the_tree
mov rdi, [rsp+24]
add rsp, 56
  ret
