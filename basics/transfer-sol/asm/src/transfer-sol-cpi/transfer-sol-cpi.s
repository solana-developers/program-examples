.equ NUM_ACCOUNTS, 0x0000

.equ SENDER_HEADER, 0x0008
.equ SENDER_KEY, 0x0010
.equ SENDER_OWNER, 0x0030
.equ SENDER_LAMPORTS, 0x0050
.equ SENDER_DATA_LEN, 0x0058
.equ SENDER_DATA, 0x0060
.equ SENDER_RENT_EPOCH, 0x2860

.equ RECEIVER_HEADER, 0x2868
.equ RECEIVER_KEY, 0x2870
.equ RECEIVER_OWNER, 0x2890
.equ RECEIVER_LAMPORTS, 0x28b0
.equ RECEIVER_DATA_LEN, 0x28b8
.equ RECEIVER_DATA, 0x28c0
.equ RECEIVER_RENT_EPOCH, 0x50c0

.equ SYSTEM_PROGRAM_HEADER, 0x50c8
.equ SYSTEM_PROGRAM_KEY, 0x50d0
.equ SYSTEM_PROGRAM_OWNER, 0x50f0
.equ SYSTEM_PROGRAM_LAMPORTS, 0x5110
.equ SYSTEM_PROGRAM_DATA_LEN, 0x5118
.equ SYSTEM_PROGRAM_DATA, 0x5120
.equ SYSTEM_PROGRAM_RENT_EPOCH, 0x7930

.equ INSTRUCTION_DATA_LEN, 0x7938
.equ INSTRUCTION_DATA, 0x7940
.equ PROGRAM_ID, 0x7940


.globl entrypoint

entrypoint:

  #################
  ## Validations ##
  #################

  # Check number of accounts.
  ldxdw r2, [r1 + NUM_ACCOUNTS]
  jne r2, 3, error_invalid_num_accounts

  # Check duplicate accounts.
  ldxb r2, [r1 + RECEIVER_HEADER]
  jne r2, 0xff, error_duplicate_accounts
  ldxb r2, [r1 + SYSTEM_PROGRAM_HEADER]
  jne r2, 0xff, error_duplicate_accounts

  # Check instruction data.
  ldxdw r4, [r1 + INSTRUCTION_DATA_LEN]
  jne r4, 8, error_invalid_instruction_data
  ldxdw r4, [r1 + INSTRUCTION_DATA]

  # Check sender lamports.
  ldxdw r2, [r1 + SENDER_LAMPORTS]
  jlt r2, r4, error_insufficient_lamports

  ##############################
  ##    Stack allocations     ##
  ##############################

  mov64 r9, r10
  sub64 r9, 288                                                   # Stores transfer instruction data
  mov64 r8, r10
  sub64 r8, 272                                                   # Stores instruction
  mov64 r7, r10
  sub64 r7, 224                                                   # Stores account metas
  mov64 r6, r10
  sub64 r6, 176                                                   # Stores account infos


  #############################
  ## Set up instruction data ##
  #############################

  mov64 r2, r9
  lddw r3, 2                                                      # Instruction discriminator (2 = Transfer)
  stxw [r2 + 0], r3
  stxdw [r2 + 4], r4                                              # Lamports to transfer


  ##########################
  ## Set up account metas ##
  ##########################

  # Sender
  mov64 r2, r7
  mov64 r3, r1
  add64 r3, SENDER_KEY
  stxdw [r2 + 0], r3                                              # pubkey
  ldxb r5, [r1 + SENDER_HEADER + 2]
  stxb [r2 + 8], r5                                               # is_writable
  ldxb r5, [r1 + SENDER_HEADER + 1]
  stxb [r2 + 9], r5                                               # is_signer

  # Receiver
  add64 r2, 16
  mov64 r3, r1
  add64 r3, RECEIVER_KEY
  stxdw [r2 + 0], r3                                              # pubkey
  ldxb r3, [r1 + RECEIVER_HEADER + 2]
  stxb [r2 + 8], r3                                               # is_writable
  ldxb r3, [r1 + RECEIVER_HEADER + 1]
  stxb [r2 + 9], r3                                               # is_signer


  ############################
  ## Set up the instruction ##
  ############################

  mov64 r2, r8
  mov64 r3, r1
  add64 r3, SYSTEM_PROGRAM_KEY
  stxdw [r2 + 0], r3                                              # program_id
  mov64 r3, r7
  stxdw [r2+8], r3                                                # accounts
  lddw r3, 2
  stxdw [r2+16], r3                                               # account_len
  mov64 r3, r9
  stxdw [r2+24], r3                                               # data
  lddw r3, 12
  stxdw [r2+32], r3                                               # data_len


  ##########################
  ## Set up account infos ##
  ##########################

  # Sender
  mov64 r2, r6
  mov64 r3, r1
  add64 r3, SENDER_KEY
  stxdw [r2 + 0], r3                                              # key
  mov64 r3, r1
  add64 r3, SENDER_LAMPORTS
  stxdw [r2 + 8], r3                                              # lamports
  ldxdw r3, [r1 + SENDER_DATA_LEN]
  stxdw [r2+16], r3                                               # data_len
  mov64 r3, r1
  add64 r3, SENDER_DATA
  stxdw [r2+24], r3                                               # data
  mov64 r3, r1
  add64 r3, SENDER_OWNER
  stxdw [r2+32], r3                                               # owner
  ldxdw r3, [r1 + SENDER_RENT_EPOCH]
  stxdw [r2+40], r3                                               # rent_epoch
  ldxb r3, [r1 + SENDER_HEADER + 1]
  stxb [r2+48], r3                                                # is_signer
  ldxb r3, [r1 + SENDER_HEADER + 2]
  stxb [r2+49], r3                                                # is_writable
  ldxb r3, [r1 + SENDER_HEADER + 3]
  stxb [r2+50], r3                                                # is_executable

  # Receiver
  add64 r2, 56
  mov64 r3, r1
  add64 r3, RECEIVER_KEY
  stxdw [r2+0], r3
  mov64 r3, r1                                                    # key
  add64 r3, RECEIVER_LAMPORTS
  stxdw [r2+8], r3                                                # lamports
  ldxdw r3, [r1 + RECEIVER_DATA_LEN]
  stxdw [r2+16], r3                                               # data_len
  mov64 r3, r1
  add64 r3, RECEIVER_DATA
  stxdw [r2+24], r3                                               # data
  mov64 r3, r1
  add64 r3, RECEIVER_OWNER
  stxdw [r2+32], r3                                               # owner
  ldxdw r3, [r1 + RECEIVER_RENT_EPOCH]
  stxdw [r2+40], r3                                               # rent_epoch
  ldxb r3, [r1 + RECEIVER_HEADER + 1]
  stxb [r2+48], r3                                                # is_signer
  ldxb r3, [r1 + RECEIVER_HEADER + 2]
  stxb [r2+49], r3                                                # is_writable
  ldxb r3, [r1 + RECEIVER_HEADER + 3]
  stxb [r2+50], r3                                                # is_executable


  ####################
  ## Invoke the CPI ##
  ####################

  mov64 r1, r8                                                    # Instruction
  mov64 r2, r6                                                    # Account infos
  lddw r3, 2                                                      # Number of account infos
  lddw r4, 0                                                      # No seeds required
  lddw r5, 0                                                      # Seed count 0
  call sol_invoke_signed_c

  lddw r0, 0
  exit

error_invalid_num_accounts:
  lddw r0, 1
  exit

error_duplicate_accounts:
  lddw r0, 2
  exit

error_invalid_instruction_data:
  lddw r0, 3
  exit

error_insufficient_lamports:
  lddw r0, 4
  exit
