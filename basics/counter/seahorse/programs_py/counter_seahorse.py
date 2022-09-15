# counter_seahorse
# Built with Seahorse v0.1.6

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class Counter(Account):
    count: u64


@instruction
def initialize_counter(counter: Empty[Counter], payer: Signer, seed: u8):
    counter.init(
        payer=payer,
        seeds=[seed]
    )


@instruction
def increment(counter: Counter):
    counter.count += 1
