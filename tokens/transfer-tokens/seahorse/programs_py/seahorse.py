# seahorse
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id('5KCV219sxBAZMfXWP5EZ57D6K9568krgPKGe1Lq2nkxH')

@instruction
def create_token(
  mint: Empty[TokenMint],
  signer: Signer
):
  mint.init(
    payer = signer,
    decimals = 6,
    authority = signer
  )

@instruction
def mint_token(
  mint: TokenMint,
  recipient: TokenAccount,
  signer: Signer,
  amount: u64
):
  mint.mint(
    authority = signer,
    to = recipient,
    amount = amount * u64(10) ** u32(mint.decimals)
  )


@instruction
def create_associated_token_account(
  token_account: Empty[TokenAccount],
  mint: TokenMint,
  signer: Signer
):
  token_account.init(
    associated = True,
    payer = signer,
    mint = mint,
    authority = signer
  )

@instruction
def transfer(
  signer_token_account: TokenAccount,
  recipient: TokenAccount,
  signer: Signer,
  amount: u64,
  mint: TokenMint
):
  assert signer_token_account.mint() == mint.key(), 'Mint is not the token account mint'
  signer_token_account.transfer(
    authority = signer,
    to = recipient,
    amount = amount * u64(10) ** u32(mint.decimals)
  )