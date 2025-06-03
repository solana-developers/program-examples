from seahorse.prelude import *

declare_id('4uQFcoLe2AWRFShXRk18skBzrHJtg8TUHdgjRKHSwDz3')

class Message(Account):
    owner: Pubkey
    value: str

@instruction
def initialize(
    authority: Signer, 
    message: Empty[Message]
):
    message = message.init(
        payer = authority,
        seeds = ['Message', authority]
    )
    message.owner = authority.key()
    message.value = ""
    

@instruction
def hello(owner: Signer, message:Message):
    assert owner.key() == message.owner, 'This is not your message'
    message.value = "Hello GM!"

