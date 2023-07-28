# create_token

This project was created by Seahorse 0.2.7.

To get started, just add your code to **programs_py/create_token.py** and run `seahorse build`.


## Prerequisites
We need to install some command line tools for this project to build. We need [Solana](https://docs.solana.com/cli/install-solana-cli-tools), [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html#install-rust), [NodeJS](https://nodejs.org/en/) and [Seahorse](https://seahorse-lang.org/docs/installation). The links provided contain the step-by-step guide on installing these tools and the dependencies required for them like Rust.

You can check if package got installed correctly by running command like :

`solana -V`
`anchor -V`
`seahorse -V`

For this project, the version used are :
* anchor 0.28.0

* seahorse v0.2.7

* node 18.17.0

## Getting started with Seahorse
We initialize Seahorse Project using command `seahorse init token_program`. This will create a project directory with multiple files which is mostly similar to anchor projects, just will write our seahorse program in `token_program/programs_py/token_program.py`

## Involved Accounts
Under the hood, the Toke Program in Solana Program Library (SPL) is used. It contains instructions for creating and interacting with SPL-Tokens. These tokens represent all non-native (i.e. not SOL) tokens on the Solana network. 

The 3 parties involved in this token programms are :
* The **Token Program** contains instructions for creating and interacting with SPL-Tokens
* **Token Mints** are accounts which hold data about a specific Token, but do not hold Tokens
* **Token Accounts** are used to hold Tokens of a specific Token Mint

<p align="center">
  <img src="https://github.com/akshaydhayal/Seahorse-Token-Program/blob/master/assets/tokenProgram0.png" alt="Alt text" title="Optional title" height="420" width="550">
</p>


## Program Instructions
We have 3 functions/instructions in this Program. Let's understand all the different Program instructions one by one.

### 1. initTokenMint
When someone wants to create a new token, we have to use something create-token function to initialize a new Mint Account. `initTokenMint` is the function where we create our TokenMint Accounts. TokenMint account contains the following informations: 
* `mint-authority` which is a public-key (pubkey) authorized to mint this token
*  the number of `decimals` of the token etc. 
This account stores general information about the token and who has permissions over it.
<p align="center">
  <img src="https://github.com/akshaydhayal/Seahorse-Token-Program/blob/master/assets/figg1.png" alt="Alt text" title="Optional title" height="180" width="550">
</p>


### 2. initTokenAccount
A Token Account holds tokens of a specific "mint" and has a specified "owner" of the account. The token account itself is owned by the Token program and it is the Token program who controls access to these tokens using the owner, and delegate fields within the account. The owner is the pubkey who can spend/transfer the tokens, and the owner can give rights to a delegate pubkey to spend up to a delegatedAmount of tokens. 

The TokenAccount has number of fields like:
* mint - this is the mint whose tokens this this account will hold
* authority - the account that has authority over this account

We have created below 2 TokenAccounts containing 0 token now.
<p align="center">
  <img src="https://github.com/akshaydhayal/Seahorse-Token-Program/blob/master/assets/figg2.png" alt="Alt text" title="Optional title" height="150" width="740">
</p>

### 3. useTokenMint
Minting tokens is the process of issuing new tokens into circulation. When you mint tokens, you increase the supply of the token mint and deposit the newly minted tokens into a token account. Only the mint authority of a token mint is allowed to mint new tokens.

The user's token balance has been updated to 3000 now.
<p align="center">
  <img src="https://github.com/akshaydhayal/Seahorse-Token-Program/blob/master/assets/figg5.png" alt="Alt text" title="Optional title" height="140" width="850">
</p>
  
