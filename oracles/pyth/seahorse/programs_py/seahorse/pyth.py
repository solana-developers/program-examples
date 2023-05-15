# seahorse.pyth: support for the Pyth oracle in Seahorse.
#
# NOTE: this file just contains types and documentation for your editor. This
# is NOT executable code, and you won't be able to change the behavior of your
# Seahorse programs by editing this file.

from typing import *
from prelude import *

T = TypeVar('T')


class Price:
    """
    Pyth `Price` struct with some extra convenience functions. You can access the raw data of the struct through its fields.

    "A price with a degree of uncertainty, represented as a price +- a confidence interval." (from https://docs.rs/pyth-sdk-solana/0.7.1/pyth_sdk_solana/struct.Price.html)
    """

    price: i64
    conf: u64
    expo: i32
    publish_time: i64

    def num(self) -> f64:
        """Simply get price as a floating-point number. Does not take confidence into account, instead reporting the average estimated price."""


class PriceFeed:
    """
    Pyth `PriceFeed` struct with some extra convience functions.

    "Represents a current aggregation price from pyth publisher feeds." (from https://docs.rs/pyth-sdk-solana/0.7.1/pyth_sdk_solana/struct.PriceFeed.html)
    """

    def get_price(self) -> Price:
        """Get the price. Throws an error if the product is not currently trading."""
        pass


class PriceAccount(AccountWithKey):
    """Raw Pyth price account. Needs to be validated before use."""

    def validate_price_feed(self, product: str) -> PriceFeed:
        """
        Validate the price account - checks the pubkey against the known Pyth price pubkey list.

        Without checking the account key against a known Pyth key, you can not guarantee that the account actually comes from Pyth. Therefore, this step is mandatory, otherwise your program would be at risk of an easy exploit.

        The `product` parameter must be a string literal with the following format: `[cluster-]BASE/QUOTE`. The cluster is optional, and defaults to mainnet.

        For example, mainnet SOL/USD is just 'SOL/USD'. Devnet USD/JPY is 'devnet-USD/JPY'.

        @param product: The symbol of the product whose price you want. Must be a string literal in the format discussed above.
        """
        pass
