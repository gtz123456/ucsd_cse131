
section .text
global our_code_starts_here
our_code_starts_here:
  
mov rax, 3
mov [rsp-16], rax
mov rax, 4
imul rax, [rsp-16]
  ret
