.globl entrypoint

entrypoint:

  lddw r1, message # Load message in r1
  mov64 r2, 14 # Move length of message to r2
  call sol_log_ # Call sol_log_(arg0, arg1) 
  exit
  
.rodata
  message: .ascii "Hello, Solana!"
