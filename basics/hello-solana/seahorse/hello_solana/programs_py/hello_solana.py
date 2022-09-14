from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

class Counter(Account):
    authority: Pubkey
    value: u8


@instruction
def initialize(
    authority: Signer, 
    counter: Empty[Counter]
):
    # Initialize the counter and set the authority
    counter = counter.init(
        payer = authority,
        seeds = ['new_delhi_hh', authority]
    )

    counter.authority = authority.key()
    counter.value = 0
    
    print("Hello, Solana from Seahorse!")


@instruction
def increment(
    authority: Signer, 
    counter: Counter
):
    counter.value += 1