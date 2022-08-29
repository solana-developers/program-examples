# hello_solana
# Built with Seahorse v0.1.5

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


@instruction
def hello(signer: Signer):
    print("Hello, Solana from Seahorse!")

    print(f"This is the public key of the signer: {signer.key()}")
