# Destroy an Account 
> **_NOTE_:_** If you haven't installed poseidon yet, follow the installation steps [here](https://turbin3.github.io/poseidon/installation.html)
- We're writing our TypeScript program code in [/ts-programs](./ts-programs/)
- Once TypeScript program is completed, generate a program id and replace the `PROGRAM_ID` with the actual one. To generate a program id, run:
```
anchor keys list
# You'll get something similar, but it will definitely be different
close-account: At2EEHZ4zq2roeR5Cba6dryYEsmsHz7MKt9tjUCpCng1
```
- To convert your TypeScript Solana program to Anchor program, run
```
poseidon -i ./ts-programs/closeAccount.ts -o programs/close-account/src/lib.rs
```