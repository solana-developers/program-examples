// import * as anchor from "@coral-xyz/anchor";
// import type { Program } from "@coral-xyz/anchor";
// import { expect } from "chai";
// import type { SwapExample } from "../target/types/token_swap";
// import { type TestValues, createValues, mintingTokens } from "./utils";
//import { startAnchor } from "solana-bankrun";
// import { BankrunProvider } from "anchor-bankrun";

// const IDL = require("../target/idl/token_swap.json");
// const PROGRAM_ID = new PublicKey(IDL.address);

// describe("Withdraw liquidity", async () => {
// / const context = await startAnchor(
//    "",
//    [{ name: "token_swap", programId: PROGRAM_ID }],
//    []
//  );

//  const provider = new BankrunProvider(context);

//  const payer = provider.wallet as anchor.Wallet;

//  const connection = provider.connection;

//  const program = new anchor.Program<TokenSwap>(IDL, provider);

//   let values: TestValues;

//   beforeEach(async () => {
//     values = createValues();

//     await program.methods
//       .createAmm(values.id, values.fee)
//       .accounts({ amm: values.ammKey, admin: values.admin.publicKey })
//       .rpc();

//     await mintingTokens({
//       connection,
//       creator: values.admin,
//       mintAKeypair: values.mintAKeypair,
//       mintBKeypair: values.mintBKeypair,
//     });

//     await program.methods
//       .createPool()
//       .accounts({
//         amm: values.ammKey,
//         pool: values.poolKey,
//         poolAuthority: values.poolAuthority,
//         mintLiquidity: values.mintLiquidity,
//         mintA: values.mintAKeypair.publicKey,
//         mintB: values.mintBKeypair.publicKey,
//         poolAccountA: values.poolAccountA,
//         poolAccountB: values.poolAccountB,
//       })
//       .rpc();

//     await program.methods
//       .depositLiquidity(values.depositAmountA, values.depositAmountA)
//       .accounts({
//         pool: values.poolKey,
//         poolAuthority: values.poolAuthority,
//         depositor: values.admin.publicKey,
//         mintLiquidity: values.mintLiquidity,
//         mintA: values.mintAKeypair.publicKey,
//         mintB: values.mintBKeypair.publicKey,
//         poolAccountA: values.poolAccountA,
//         poolAccountB: values.poolAccountB,
//         depositorAccountLiquidity: values.liquidityAccount,
//         depositorAccountA: values.holderAccountA,
//         depositorAccountB: values.holderAccountB,
//       })
//       .signers([values.admin])
//       .rpc({ skipPreflight: true });
//   });

//   it("Withdraw everything", async () => {
//     await program.methods
//       .withdrawLiquidity(values.depositAmountA.sub(values.minimumLiquidity))
//       .accounts({
//         amm: values.ammKey,
//         pool: values.poolKey,
//         poolAuthority: values.poolAuthority,
//         depositor: values.admin.publicKey,
//         mintLiquidity: values.mintLiquidity,
//         mintA: values.mintAKeypair.publicKey,
//         mintB: values.mintBKeypair.publicKey,
//         poolAccountA: values.poolAccountA,
//         poolAccountB: values.poolAccountB,
//         depositorAccountLiquidity: values.liquidityAccount,
//         depositorAccountA: values.holderAccountA,
//         depositorAccountB: values.holderAccountB,
//       })
//       .signers([values.admin])
//       .rpc({ skipPreflight: true });

//     const liquidityTokenAccount = await connection.getTokenAccountBalance(
//       values.liquidityAccount
//     );
//     const depositTokenAccountA = await connection.getTokenAccountBalance(
//       values.holderAccountA
//     );
//     const depositTokenAccountB = await connection.getTokenAccountBalance(
//       values.holderAccountB
//     );
//     expect(liquidityTokenAccount.value.amount).to.equal("0");
//     expect(Number(depositTokenAccountA.value.amount)).to.be.lessThan(
//       values.defaultSupply.toNumber()
//     );
//     expect(Number(depositTokenAccountA.value.amount)).to.be.greaterThan(
//       values.defaultSupply.sub(values.depositAmountA).toNumber()
//     );
//     expect(Number(depositTokenAccountB.value.amount)).to.be.lessThan(
//       values.defaultSupply.toNumber()
//     );
//     expect(Number(depositTokenAccountB.value.amount)).to.be.greaterThan(
//       values.defaultSupply.sub(values.depositAmountA).toNumber()
//     );
//   });
// });
