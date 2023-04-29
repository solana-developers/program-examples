import asyncio
from anchorpy import create_workspace, close_workspace, Context
from solders.system_program import ID as SYS_PROGRAM_ID
from solders.pubkey import Pubkey
from solders.keypair import Keypair
from solders.sysvar import RENT



async def main():
    # Read the deployed program from the workspace.
    workspace = create_workspace()
    program = workspace["seahorse"]

    TOKEN_PROGRAM_ID = Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
    ASSOCITAED_TOKEN_PROGRAM_ID = Pubkey.from_string("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")

    # Create a Mint keypair. This will be our token mint.
    mint = Keypair()


    # Execute the instructions

    # Create a token
    create_token = await program.rpc["create_token"](ctx=Context(accounts={
        "mint": mint.pubkey(),
        "signer": program.provider.wallet.payer.pubkey(),
        "system_program": SYS_PROGRAM_ID,
        "rent": RENT,
        "token_program": TOKEN_PROGRAM_ID
    }, signers=[program.provider.wallet.payer, mint]))

    print("Create token signature: ", create_token)

    # Create a token account
    associated_token_account_pubkey, nonce = Pubkey.find_program_address([bytes(program.provider.wallet.payer.pubkey()), bytes(TOKEN_PROGRAM_ID), bytes(mint.pubkey())], ASSOCITAED_TOKEN_PROGRAM_ID)
    create_associated_token_account = await program.rpc["create_associated_token_account"](ctx=Context(accounts={
        "mint": mint.pubkey(),
        "token_account" : associated_token_account_pubkey,
        "signer": program.provider.wallet.payer.pubkey(),
        "system_program": SYS_PROGRAM_ID,
        "rent": RENT,
        "token_program": TOKEN_PROGRAM_ID,
        "associated_token_program" : ASSOCITAED_TOKEN_PROGRAM_ID
    }, signers=[program.provider.wallet.payer]))

    print("Create associated token account signature: ", create_associated_token_account)

    # Mint tokens
    mint_token = await program.rpc["mint_token"](1000, ctx=Context(accounts={
        "mint": mint.pubkey(),
        "recipient" : associated_token_account_pubkey,
        "signer": program.provider.wallet.payer.pubkey(),
        "system_program": SYS_PROGRAM_ID,
        "rent": RENT,
        "token_program": TOKEN_PROGRAM_ID,
    }, signers=[program.provider.wallet.payer]))

    print("Mint token signature: ", mint_token)

    # Transfer tokens (the tokens are transfered to the sender here, but you can of course create a new associated token account for the recipient)
    transfer = await program.rpc["transfer"](1000, ctx=Context(accounts={
        "signer_token_account": associated_token_account_pubkey,
        "recipient" : associated_token_account_pubkey,
        "signer": program.provider.wallet.payer.pubkey(),
        "system_program": SYS_PROGRAM_ID,
        "rent": RENT,
        "token_program": TOKEN_PROGRAM_ID,
        "mint": mint.pubkey()
    }, signers=[program.provider.wallet.payer]))

    print("Transfer signature: ", transfer)


    # Close all HTTP clients in the workspace.
    await close_workspace(workspace)


asyncio.run(main())