# Solana Program: Unlocking a Vault Based on SOL Price and Signature Verification

This Solana escrow program allows users to withdraw funds only when the SOL price reaches a certain target and after verifying their Ed25519 signature.

## Ed25519 Signature Verification

In Solana, programs cannot directly call the Ed25519 program using a CPI (Cross-Program Invocation) because signature verification is computationally expensive. Instead, the Ed25519 signature verification program exists as a precompiled instruction outside the Solana Virtual Machine (SVM).
we do verification by passing two instructions one [Ed25519 program](https://github.com/anza-xyz/agave/blob/master/sdk/ed25519-program/src/lib.rs) ix and second our custom logic ix (it mush have a sysvar ix to get current chain state)

The sysvar instructions account provides access to all instructions within the same transaction.
This allows our program to fetch and verify the arguments passed to the Ed25519 program, ensuring they were correctly signed before unlocking funds.

## Running test

To run this test on devnet, you can do the following:

```bash
$ pnpm install
$ anchor test --skip-deploy
```

# Vault Unlock Conditions

The SOL price must meet or exceed the target threshold & Ed25519 signature must be verified

# Vault Architecture

```mermaid
flowchart TD
    %% Style Definitions - GitHub Monochrome Colors
    classDef darkBackground fill:#24292e,stroke:#1b1f23,stroke-width:2,color:#ffffff,font-size:24px
    classDef boxStyle fill:#2f363d,stroke:#1b1f23,stroke-width:2,color:#ffffff,font-size:22px
    classDef subBoxStyle fill:#444d56,stroke:#1b1f23,stroke-width:2,color:#ffffff,font-size:20px
    classDef lighterBoxStyle fill:#586069,stroke:#1b1f23,stroke-width:2,color:#ffffff,font-size:20px

    %% User Entry Points
    subgraph UserActions["User Actions"]
        direction TB
        class UserActions darkBackground
        Deposit["Deposit SOL + Ed25519 Sig"]
        Withdraw["Withdraw Request + Ed25519 Sig + Feed ID"]
    end

    %% Program Logic
    subgraph ProgramFlow["Escrow Program"]
        class ProgramFlow boxStyle

        %% Signature Verification
        subgraph SigVerification["Ed25519 Signature Verification"]
            class SigVerification subBoxStyle
            GetPrevIx["Get Previous Instruction"]
            VerifyProgram["Verify Ed25519 Program ID"]

            subgraph OffsetValidation["Signature Offset Validation"]
                class OffsetValidation lighterBoxStyle
                ValidatePK["Validate Public Key Offset"]
                ValidateSig["Validate Signature Offset"]
                ValidateMsg["Validate Message Data"]
                VerifyIndices["Verify Instruction Indices Match"]
            end
        end

        %% Main Operations
        subgraph Operations["Program Operations"]
            class Operations subBoxStyle

            subgraph DepositFlow["Deposit Handler"]
                class DepositFlow lighterBoxStyle
                UpdateState["Update Escrow State:
                - Set unlock_price
                - Set escrow_amount"]
                TransferToEscrow["Transfer SOL to Escrow Account"]
            end

            subgraph WithdrawFlow["Withdraw Handler"]
                class WithdrawFlow lighterBoxStyle
                GetPrice["Get Price from Pyth"]
                PriceCheck["Check if price > unlock_price"]
                TransferToUser["Transfer SOL to User"]
            end
        end
    end

    %% Flow Connections
    Deposit --> GetPrevIx
    Withdraw --> GetPrevIx
    GetPrevIx --> VerifyProgram
    VerifyProgram --> OffsetValidation
    ValidatePK & ValidateSig & ValidateMsg --> VerifyIndices

    VerifyIndices -->|"Signature Valid"| Operations
    VerifyIndices -->|"Invalid"| Error["Return Signature Error"]

    Operations --> DepositFlow
    Operations --> WithdrawFlow

    GetPrice --> PriceCheck
    PriceCheck -->|"Price > Unlock Price"| TransferToUser
    PriceCheck -->|"Price <= Unlock Price"| WithdrawError["Return Invalid Withdrawal Error"]

    %% Apply Styles
    class Deposit,Withdraw boxStyle
    class GetPrevIx,VerifyProgram,Error,WithdrawError subBoxStyle
    class UpdateState,TransferToEscrow,GetPrice,CheckAge,PriceCheck,TransferToUser lighterBoxStyle
```
