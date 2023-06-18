
section .text
global our_code_starts_here
our_code_starts_here:
  
mov rax, 10
mov [rsp-16], rax
mov rax, 10
mov [rsp-24], rax
mov rax, 5
add rax, [rsp-24]
sub rax, [rsp-16]
mov [rsp-16], rax
mov rax, 10
imul rax, [rsp-16]
  ret
