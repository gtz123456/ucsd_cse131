
section .text
global our_code_starts_here
our_code_starts_here:
  
mov rax, 1
mov [rsp-16], rax
mov rax, 8
sub rax, [rsp-16]
  ret
