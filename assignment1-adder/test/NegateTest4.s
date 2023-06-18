
section .text
global our_code_starts_here
our_code_starts_here:
  mov rax, 999
sub rax, 1
add rax, 1
add rax, 1
sub rax, 1
neg rax
  ret
