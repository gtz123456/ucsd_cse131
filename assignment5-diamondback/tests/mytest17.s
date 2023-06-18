
section .text
global our_code_starts_here
our_code_starts_here:
  
mov rax, -3
mov [rsp-16], rax
mov rax, -2
mov [rsp-24], rax
mov rax, -1
sub rax, [rsp-24]
sub rax, [rsp-16]
  ret
