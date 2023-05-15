# seahorse
# Built with Seahorse v0.2.7

from seahorse.prelude import *
from seahorse.pyth import *

declare_id('9USP8f9ooxUxWTyqrQSDfyiXE1FP7Wfsg34NfAbdK1ur')

@instruction
def get_pyth_price(
    pyth_price_account: PriceAccount,
    signer: Signer,
    ):
    price_feed = pyth_price_account.validate_price_feed("SOL/USD")
    price_feed.get_price().num()

    price = price_feed.get_price()

    x: f64 = price.num()
    p: i64 = price.price
    c: u64 = price.conf
    e: i32 = price.expo


    print("Pyth price: ", x)
    print("Pyth price without decimals: ", p)
    print("Pyth confidence interval: ", c)
    print("Pyth account decimal exponent: ", e)




