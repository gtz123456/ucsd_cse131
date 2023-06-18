
section .text
global our_code_starts_here
our_code_starts_here:
  
mov rax, 10
mov [rsp-16], rax
mov rax, 15
mov [rsp-24], rax
mov rax, [rsp-24]
mov [rsp-32], rax
mov rax, [rsp-24]
add rax, [rsp-32]
mov [rsp-24], rax
mov rax, [rsp-16]
add rax, [rsp-24]
  ret
