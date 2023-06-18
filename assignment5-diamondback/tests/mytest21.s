
section .text
global our_code_starts_here
our_code_starts_here:
  
mov [rsp-24], rax
mov [rsp-24], rax
mov rax, 5
mov [rsp-16], rax
mov rax, [rsp-24]
mov rax, [rsp-16]
mov [rsp-16], rax
mov rax, [rsp-24]
mov rax, [rsp-16]
  ret
