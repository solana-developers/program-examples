# seahorse
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id('2RjL4mpTANyGxz7fLWEbQtmdEDti7c4CqsLR96mgvcaV')

class MockAccount(Account):
    mock_value: u8


## We don't care about this instruction, but we need an address to send our SOL to.
@instruction
def init_mock_account(signer: Signer, mock_account: Empty[MockAccount]):
    account = mock_account.init(
        payer = signer,
        seeds = ['mock_account'],
    )
    account.mock_value = u8(0)

@instruction
def transfer_sol_with_cpi(sender: Signer, recipient: MockAccount, amount: u64):
    sender.transfer_lamports(
        to = recipient,
        amount = amount,
    )
