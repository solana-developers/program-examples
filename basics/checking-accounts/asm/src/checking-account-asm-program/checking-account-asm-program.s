.equ NUM_ACCOUNTS,              0x0000

# payer
.equ PAYER_HEADER,              0x0008
.equ PAYER_KEY,                 0x0010
.equ PAYER_OWNER,               0x0030
.equ PAYER_LAMPORTS,            0x0050
.equ PAYER_DATA_LEN,            0x0058
.equ PAYER_DATA,                0x0060
.equ PAYER_RENT_EPOCH,          0x2860

# account_to_create
.equ ACCOUNT_TO_CREATE_HEADER,  0x2868
.equ ACCOUNT_TO_CREATE_KEY,     0x2870
.equ ACCOUNT_TO_CREATE_OWNER,   0x2890
.equ ACCOUNT_TO_CREATE_LAMPORTS,0x28b0
.equ ACCOUNT_TO_CREATE_DATA_LEN,0x28b8
.equ ACCOUNT_TO_CREATE_DATA,    0x28c0
.equ ACCOUNT_TO_CREATE_RENT_EPOCH, 0x50c0

# account_to_change
.equ ACCOUNT_TO_CHANGE_HEADER,  0x50c8
.equ ACCOUNT_TO_CHANGE_KEY,     0x50d0
.equ ACCOUNT_TO_CHANGE_OWNER,   0x50f0
.equ ACCOUNT_TO_CHANGE_LAMPORTS,0x5110
.equ ACCOUNT_TO_CHANGE_DATA_LEN,0x5118
.equ ACCOUNT_TO_CHANGE_DATA,    0x5120
.equ ACCOUNT_TO_CHANGE_RENT_EPOCH, 0x7920

# system_program
.equ SYSTEM_PROGRAM_HEADER,     0x7928
.equ SYSTEM_PROGRAM_KEY,        0x7930
.equ SYSTEM_PROGRAM_OWNER,      0x7950
.equ SYSTEM_PROGRAM_LAMPORTS,   0x7970
.equ SYSTEM_PROGRAM_DATA_LEN,   0x7978
.equ SYSTEM_PROGRAM_DATA,       0x7980
.equ SYSTEM_PROGRAM_RENT_EPOCH, 0xa180

# instruction data
.equ INSTRUCTION_DATA_LEN,      0xa188
.equ INSTRUCTION_DATA,          0xa190
.equ PROGRAM_ID,                0xa198

.globl entrypoint

entrypoint:

    ### Validations

    # Check number of accounts.
    ldxdw r2, [r1 + NUM_ACCOUNTS]
    jne r2, 4, error_invalid_num_accounts

    # Check payer is signer
    ldxb r2, [r1 + PAYER_HEADER + 1]   # load is_signer byte
    jne  r2, 1, error_not_signer        # jump if not a signer

    # Check account_to_create has not been initialized
    ldxdw r2, [r1 + ACCOUNT_TO_CREATE_LAMPORTS]         # load lamports of account to create
    jne r2, 0, error_initialized                        # jump if initialized

    # Check account_to_change is already initialized
    ldxdw r2, [r1 + ACCOUNT_TO_CHANGE_LAMPORTS]     # load lamports of account to change
    jeq r2, 0, error_not_initialized                # jump if lamports == 0 (not initialized)

    # Check system program key is all zeros (11111111111111111111111111111111)

    ldxdw r2, [r1 + SYSTEM_PROGRAM_KEY + 0]
    jne r2, 0, error_invalid_system_program

    ldxdw r2, [r1 + SYSTEM_PROGRAM_KEY + 8]
    jne r2, 0, error_invalid_system_program

    ldxdw r2, [r1 + SYSTEM_PROGRAM_KEY + 16]
    jne r2, 0, error_invalid_system_program

    ldxdw r2, [r1 + SYSTEM_PROGRAM_KEY + 24]
    jne r2, 0, error_invalid_system_program

    lddw r0, 0
    exit

error_invalid_num_accounts:
    lddw r0, 1
    exit

error_not_signer:
    lddw r0, 2
    exit

error_initialized:
    lddw r0, 3
    exit

error_not_initialized:
    lddw r0, 4
    exit

error_invalid_system_program:
    lddw r0, 5
    exit
