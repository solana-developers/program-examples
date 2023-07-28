# create_token
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')
  
# This init_token_mint is the function where we create our TokenMint Accounts.This account stores general 
# information about the token and who has permissions over it.
@instruction
def init_token_mint(new_token_mint: Empty[TokenMint], signer: Signer):
  # On top of the regular init args, you need to provide:
  #   - the number of decimals that this token will have
  #   - the account that has authority over this account.
  new_token_mint.init(
    payer = signer,
    seeds = ['token-mint', signer],
    decimals = 0,
    authority = signer
  )

#This function is initialsing and creating token account which holds tokens of a specific "mint" and
#has a specified "owner" of the account
@instruction
def init_token_account(new_token_account: Empty[TokenAccount],mint: TokenMint,signer: Signer):
  # On top of the regular init args, you need to provide:
  #   - the mint that this token account will hold tokens of
  #   - the account that has authority over this account.
  new_token_account.init(payer = signer, seeds = ['token-account1', signer],
                         mint = mint, authority = signer)
  


#This function is used to Mint tokens which is the process of issuing new tokens into circulation.
@instruction
def use_token_mint(mint: TokenMint,recipient: TokenAccount,signer: Signer):
  # Mint 100 tokens from our `mint` to `recipient`.
  # `signer` must be the authority (owner) for `mint`.
  # Note that the amounts here are in *native* token quantities - you need to
  # account for decimals when you make calls to .mint().
  print("Before mint Owner token : ",recipient.amount)
  mint.mint(
    authority = signer,
    to = recipient,
    amount = u64(3000)
  )
  print("After mint Owner token : ",recipient.amount)