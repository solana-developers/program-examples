## Using Seahorse with Pyth

Make sure to import the Pyth module for Seahorse. 

`from seahorse.pyth import *` 

You have to validate a price feed's ID for security reasons, if you want to read the SOL/USD price feed, you can verify the account in the context like this:

`price_account.validate_price_feed("SOL/USD")`

For more information, visit the [Seahorse documentation on Pyth](https://seahorse-lang.org/docs/pyth).


