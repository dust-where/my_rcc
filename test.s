.inter_syntax noprefix
.global add
add
  push rbp
  mov rbp,rsp
  mov rax, [rbp+24]
  push rax
  mov rax, [rbp+16]
  pop rdi
  add rax,rdi
  mov rsp, rbp
  pop rbp
  ret
  mov rax,0
  mov rsp, rbp
  pop rbp
  ret
.global main
main
  push rbp
  mov rbp,rsp
  mov rax,4
  push rax
  mov rax,2
  push rax
  mov rax,1
  pop rdi
  add rax,rdi
  push rax
  call add
  add rsp, 16
  push rax
  mov rax, [rbp-8]
  push rax
  mov rax, [rbp-8]
  pop rdi
  add rax,rdi
  mov rsp, rbp
  pop rbp
  ret
  mov rax,0
  mov rsp, rbp
  pop rbp
  ret
