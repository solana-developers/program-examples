.equ NUM_ACCOUNTS, 0x0000

.equ PAYER_HEADER, 0x0008
.equ PAYER_KEY, 0x0010
.equ PAYER_OWNER, 0x0030
.equ PAYER_LAMPORTS, 0x0050
.equ PAYER_DATA_LEN, 0x0058
.equ PAYER_DATA, 0x0060
.equ PAYER_RENT_EPOCH, 0x2860

.equ NEW_ACCOUNT_HEADER, 0x2868
.equ NEW_ACCOUNT_KEY, 0x2870
.equ NEW_ACCOUNT_OWNER, 0x2890
.equ NEW_ACCOUNT_LAMPORTS, 0x28b0
.equ NEW_ACCOUNT_DATA_LEN, 0x28b8
.equ NEW_ACCOUNT_DATA, 0x28c0
.equ NEW_ACCOUNT_RENT_EPOCH, 0x50c0

.equ SYSTEM_PROGRAM_KEY, 0x50d0

.equ PROGRAM_ID, 0x7938

.equ ACCOUNT_STORAGE_OVERHEAD, 0x80    # 128
.equ DEFAULT_RENT_EXEMPTION_THRESHOLD, 0x2

.equ CPI_N_ACCOUNTS, 2
.equ CPI_CREATE_ACCOUNT_INSN_DATA_LEN, 52


.globl entrypoint

entrypoint:

    # Check number of accounts.
    ldxdw r2, [r1 + NUM_ACCOUNTS]
    jne r2, 3, error_invalid_num_accounts

    mov64 r6, r1                    # save input pointer before r1 is clobbered by logging calls

    lddw r1, message_one
    mov64 r2, 45
    call sol_log_

    lddw r1, message_two
    mov64 r2, 25
    call sol_log_

    mov64 r1, r6
    add64 r1, NEW_ACCOUNT_KEY
    call sol_log_pubkey

    # calculate lamports for new accoutnt creation
    mov64 r1, r10
    add64 r1, -16
    call sol_get_rent_sysvar
    ldxdw r3, [r10 + -16]           # r1 is clobbered after the call, use r10 directly
    lddw r4, ACCOUNT_STORAGE_OVERHEAD
    mul64 r4, r3
    mul64 r4, DEFAULT_RENT_EXEMPTION_THRESHOLD

    # build ix data for create account

    mov64 r8, r10
    sub64 r8, 56

    lddw r3, 0
    stxw [r8 + 0], r3               # discriminator = 0 (CreateAccount)
    stxdw [r8 + 4], r4              # lamports (r4 from rent calc)
    lddw r3, 0
    stxdw [r8 + 12], r3             # space = 0

    mov64 r1, r8
    add64 r1, 20                    # set dst in r1
    mov64 r2, r6
    add64 r2, PROGRAM_ID            # set src in r2
    lddw r3, 32                     # set len in r3
    call sol_memcpy_                # call sol_memcpy_(dst, src, len)


    # construct account metas
    mov64 r9, r8
    sub64 r9, 32                    # allocating 32 bytes behind create account ix data

    # payer meta
    mov64 r2, r9
    mov64 r3, r6
    add64 r3, PAYER_KEY
    stxdw [r2 + 0], r3              # pubkey pointer
    ldxb r3, [r6 + PAYER_HEADER + 2]
    stxb [r2 + 8], r3               # is_writable
    ldxb r3, [r6 + PAYER_HEADER + 1]
    stxb [r2 + 9], r3               # is_signer

    # New account meta
    add64 r2, 16
    mov64 r3, r6
    add64 r3, NEW_ACCOUNT_KEY
    stxdw [r2 + 0], r3              # pubkey pointer
    ldxb r3, [r6 + NEW_ACCOUNT_HEADER + 2]
    stxb [r2 + 8], r3               # is_writable
    lddw r3, 1
    stxb [r2 + 9], r3               # is_signer = 1 (new account has to sign)

    # build sol instruction
    mov64 r7, r9
    sub64 r7, 40                    # allocate 40 bytes for SolInstruction

    mov64 r2, r7
    mov64 r3, r6
    add64 r3, SYSTEM_PROGRAM_KEY
    stxdw [r2 + 0], r3              # program_id
    stxdw [r2 + 8], r9              # accounts pointer
    lddw r3, CPI_N_ACCOUNTS
    stxdw [r2 + 16], r3             # account_len
    stxdw [r2 + 24], r8             # data pointer
    lddw r3, CPI_CREATE_ACCOUNT_INSN_DATA_LEN
    stxdw [r2 + 32], r3             # data_len


    # build account infos
    mov64 r5, r7
    sub64 r5, 112                   # allocate 112 bytes for 2 accounts info

    # Payer info
    mov64 r2, r5                    # store our pointer to the stack
    mov64 r3, r6                    # copy input buffer to r3
    add64 r3, PAYER_KEY
    stxdw [r2 + 0], r3              # key
    mov64 r3, r6
    add64 r3, PAYER_LAMPORTS
    stxdw [r2 + 8], r3              # lamports
    ldxdw r3, [r6 + PAYER_DATA_LEN]
    stxdw [r2 + 16], r3             # data_len
    mov64 r3, r6
    add64 r3, PAYER_DATA
    stxdw [r2 + 24], r3             # data
    mov64 r3, r6
    add64 r3, PAYER_OWNER
    stxdw [r2 + 32], r3             # owner
    ldxdw r3, [r6 + PAYER_RENT_EPOCH]
    stxdw [r2 + 40], r3             # rent_epoch
    ldxb r3, [r6 + PAYER_HEADER + 1]
    stxb [r2 + 48], r3              # is_signer
    ldxb r3, [r6 + PAYER_HEADER + 2]
    stxb [r2 + 49], r3              # is_writable
    ldxb r3, [r6 + PAYER_HEADER + 3]
    stxb [r2 + 50], r3              # is_executable

    # New account info
    add64 r2, 56
    mov64 r3, r6
    add64 r3, NEW_ACCOUNT_KEY
    stxdw [r2 + 0], r3              # key
    mov64 r3, r6
    add64 r3, NEW_ACCOUNT_LAMPORTS
    stxdw [r2 + 8], r3              # lamports
    ldxdw r3, [r6 + NEW_ACCOUNT_DATA_LEN]
    stxdw [r2 + 16], r3             # data_len
    mov64 r3, r6
    add64 r3, NEW_ACCOUNT_DATA
    stxdw [r2 + 24], r3             # data
    mov64 r3, r6
    add64 r3, NEW_ACCOUNT_OWNER
    stxdw [r2 + 32], r3             # owner
    ldxdw r3, [r6 + NEW_ACCOUNT_RENT_EPOCH]
    stxdw [r2 + 40], r3             # rent_epoch
    ldxb r3, [r6 + NEW_ACCOUNT_HEADER + 1]
    stxb [r2 + 48], r3              # is_signer
    ldxb r3, [r6 + NEW_ACCOUNT_HEADER + 2]
    stxb [r2 + 49], r3              # is_writable
    ldxb r3, [r6 + NEW_ACCOUNT_HEADER + 3]
    stxb [r2 + 50], r3              # is_executable

    mov64 r1, r7                    # instruction
    mov64 r2, r5                    # account infos
    lddw r3, CPI_N_ACCOUNTS         # account count
    mov64 r4, 0                     # no signer seeds (not a PDA)
    lddw r5, 0                      # signer seeds count
    call sol_invoke_signed_c

    exit


error_invalid_num_accounts:
    lddw r0, 1
    exit

.rodata
    message_one: .ascii "Program invoked. Creating a system account..."

    message_two: .ascii "  New public key will be:"
