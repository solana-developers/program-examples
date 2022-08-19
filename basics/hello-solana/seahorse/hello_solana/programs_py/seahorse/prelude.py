# seahorse.prelude: the basis for writing Seahorse programs.
#
# NOTE: this file just contains types and documentation for your editor. This
# is NOT executable code, and you won't be able to change the behavior of your
# Seahorse programs by editing this file.

from typing import *
from math import floor, ceil

T = TypeVar('T')
N = TypeVar('N')


# ===========================================================
# Internal types - here for completeness, but not really used
# ===========================================================
    
class ProgramResult:
    """Result from executing an instruction - either a success, or a failure with an error message."""


# ==========
# Rust types
# ==========

class u8:
    """Single-byte unsigned integer."""

    def __init__(self, _: Any):
        return self

    def __add__(self, _: Any):
        return self

    def __radd__(self, _: Any):
        return self

    def __iadd__(self, _: Any):
        return self

    def __sub__(self, _: Any):
        return self

    def __rsub__(self, _: Any):
        return self

    def __isub__(self, _: Any):
        return self

    def __mul__(self, _: Any):
        return self

    def __rmul__(self, _: Any):
        return self

    def __imul__(self, _: Any):
        return self

    def __div__(self, _: Any):
        return self

    def __rdiv__(self, _: Any):
        return self

    def __idiv__(self, _: Any):
        return self

class u64:
    """64-bit unsigned integer."""

    def __init__(self, _: Any):
        return self

    def __add__(self, _: Any):
        return self

    def __radd__(self, _: Any):
        return self

    def __iadd__(self, _: Any):
        return self

    def __sub__(self, _: Any):
        return self

    def __rsub__(self, _: Any):
        return self

    def __isub__(self, _: Any):
        return self

    def __mul__(self, _: Any):
        return self

    def __rmul__(self, _: Any):
        return self

    def __imul__(self, _: Any):
        return self

    def __div__(self, _: Any):
        return self

    def __rdiv__(self, _: Any):
        return self

    def __idiv__(self, _: Any):
        return self

class i64:
    """64-bit signed integer."""

    def __init__(self, _: Any):
        return self

    def __add__(self, _: Any):
        return self

    def __radd__(self, _: Any):
        return self

    def __iadd__(self, _: Any):
        return self

    def __sub__(self, _: Any):
        return self

    def __rsub__(self, _: Any):
        return self

    def __isub__(self, _: Any):
        return self

    def __mul__(self, _: Any):
        return self

    def __rmul__(self, _: Any):
        return self

    def __imul__(self, _: Any):
        return self

    def __div__(self, _: Any):
        return self

    def __rdiv__(self, _: Any):
        return self

    def __idiv__(self, _: Any):
        return self

class f64:
    """64-bit floating point number."""

    def __add__(self, _: Any):
        return self

    def __radd__(self, _: Any):
        return self

    def __iadd__(self, _: Any):
        return self

    def __sub__(self, _: Any):
        return self

    def __rsub__(self, _: Any):
        return self

    def __isub__(self, _: Any):
        return self

    def __mul__(self, _: Any):
        return self

    def __rmul__(self, _: Any):
        return self

    def __imul__(self, _: Any):
        return self

    def __div__(self, _: Any):
        return self

    def __rdiv__(self, _: Any):
        return self

    def __idiv__(self, _: Any):
        return self


class Array(Generic[T, N]):
    """A fixed-length array: contains type T and has size N.

    Lists (Python builtin type) can coerce to this type. Example:

    ```
    class MyData(Account):
        data: Array[u64, 4]

    @instruction
    def set_data(my_data: MyData):
        # Will successfully set `data` to [0, 1, 2, 3]
        my_data.data = [i for i in range(0, 4)]
        # Will attempt (and fail, crashing the instruction at runtime!) to set `data` to [0, 1, 2, 3, 4]
        my_data.data = [i for i in range(0, 5)]
    ```
    """

class Enum:
    """A type that can have one of multiple named values.

    Note that unlike Rust enums, these cannot contain any data (other than the variant itself). Example:

    ```
    class MyEnum(Enum):
        ONE = 1
        TWO = 2
        THREE = 3

    @instruction
    def use_enum(code: MyEnum):
        if code == MyEnum.ONE:
            print(1)
        # ...
    ```
    """

# ============
# Solana types
# ============

class Pubkey:
    """32-byte account identifier."""

class SolanaAccount:
    """Generic Solana account."""

    def key(self) -> Pubkey:
        """Get this account's key."""

    def transfer_lamports(self, to: SolanaAccount, amount: u64):
        """Transfer some SOL (as an amount of lamports) to another account.

        Note: this will successfully transfer from a program-owned account without needing to
        provide the seeds for a PDA, so no signer field is required (unlike the SPL methods).
        """

class Account(SolanaAccount):
    """User-defined Solana account."""

class Signer(SolanaAccount):
    """Instruction signer."""

class Empty(Generic[T]):
    """An account that needs to be initialized."""

    def bump(self) -> u8:
        """Get this account's bump, needed if you want to use this account to sign CPI calls."""

    def init(self, payer: Signer, seeds: List[Union[str, Account, u8]], mint: TokenMint, authority: Account) -> T:
        """
        Initialize the account.
        
        @param payer: The account that will pay for the rent cost of the initialized account. Must be an instruction signer.
        @param seeds: A list of parameters to uniquely identify this account among all accounts created by your program. These may be string literals or other accounts.
        @param mint: If initializing a TokenAccount, this is the mint that the account belongs to.
        @param decimals: If initializing a TokenMint, this is the number of decimals the new token has.
        @param authority: If initializing a TokenAccount/TokenMint, this is the account that has authority over the account.
        @returns: The new, initialized account. All of the data in this account will be set to 0.
        """

class TokenAccount(SolanaAccount):
    """SPL token account."""

    def authority(self) -> Pubkey:
        """Get the owner of this token account."""

    def amount(self) -> u64:
        """Get the amount of token stored in this account."""

    def transfer(self, authority: SolanaAccount, to: TokenAccount, amount: u64, signer: List[Union[str, Account, u8]] = None):
        """
        Transfer funds from this SPL token account to another.
        
        @param authority: The account that owns this TokenAccount. Must be an instruction signer or the account given by the `signer` param.
        @param to: The recipient TokenAccount.
        @param amount: How much (in *native* token units) to transfer.
        @param signer: (Optional) seeds for the signature of a PDA.
        """

class TokenMint(SolanaAccount):
    """SPL token mint."""

    def authority(self) -> Pubkey:
        """Get the owner of this token account."""

    def mint(self, authority: SolanaAccount, to: TokenAccount, amount: u64, signer: List[Union[str, Account, u8]] = None):
        """
        Mint new tokens to a token account.

        @param authority: The account that owns this TokenMint. Must be an instruction signer or the account given by the `signer` param.
        @param to: The recipient TokenAccount.
        @param amount: How much (in *native* token units) to mint.
        @param signer: (Optional) seeds for the signature of a PDA.
        """

    def burn(self, authority: SolanaAccount, holder: TokenAccount, amount: u64, signer: List[Union[str, Account, u8]] = None):
        """
        Burn tokens from a token account.

        @param authority: The account that owns the `holder` TokenAccount. Must be an instruction signer or the account given by the `signer` param.
        @param holder: The TokenAccount to burn from.
        @param amount: How much (in *native* token units) to burn.
        @param signer: (Optional) seeds for the signature of a PDA.
        """


# ================
# Helper functions
# ================

def declare_id(id: str):
    """Inform Anchor what this program's ID is.

    @param id: The program's ID, generated by Anchor in /target/idl/<program>.json. This must be copied-pasted straight from there as a string literal.
    """

def instruction(function: Callable[..., None]) -> Callable[..., ProgramResult]:
    """Decorator to turn a function into a program instruction."""
