
section .text
global our_code_starts_here
our_code_starts_here:
  
mov [rsp-24], rax
mov [rsp-32], rax
mov rax, 5
mov [rsp-16], rax
mov rax, [rsp-16]
mov [rsp-24], rax
mov rax, [rsp-32]
mov rax, [rsp-16]
mov [rsp-16], rax
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, 1
mov [rsp-24], rax
mov rax, 0
add rax, [rsp-24]
mov [rsp-24], rax
mov rax, [rsp-32]
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, [rsp-16]
sub rax, [rsp-32]
  ret
