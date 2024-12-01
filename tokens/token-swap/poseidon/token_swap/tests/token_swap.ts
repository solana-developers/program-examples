// import * as anchor from "@coral-xyz/anchor";
// import type { Program } from "@coral-xyz/anchor";
// import { BN } from "bn.js";
// import { expect } from "chai";
// import type { SwapExample } from "../target/types/token_swap";
// import { type TestValues, createValues, mintingTokens } from "./utils";
// import { startAnchor } from "solana-bankrun";
// import { BankrunProvider } from "anchor-bankrun";

// const IDL = require("../target/idl/token_swap.json");
// const PROGRAM_ID = new PublicKey(IDL.address);

// describe("Swap", async () => {
//  const context = await startAnchor(
//    "",
//    [{ name: "token_swap", programId: PROGRAM_ID }],
//    []
//  );

//  const provider = new BankrunProvider(context);

//  const connection = provider.connection;

//  const payer = provider.wallet as anchor.Wallet;

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
//       .depositLiquidity(values.depositAmountA, values.depositAmountB)
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

//   it("Swap from A to B", async () => {
//     const input = new BN(10 ** 6);
//     await program.methods
//       .swapExactTokensForTokens(true, input, new BN(100))
//       .accounts({
//         amm: values.ammKey,
//         pool: values.poolKey,
//         poolAuthority: values.poolAuthority,
//         trader: values.admin.publicKey,
//         mintA: values.mintAKeypair.publicKey,
//         mintB: values.mintBKeypair.publicKey,
//         poolAccountA: values.poolAccountA,
//         poolAccountB: values.poolAccountB,
//         traderAccountA: values.holderAccountA,
//         traderAccountB: values.holderAccountB,
//       })
//       .signers([values.admin])
//       .rpc({ skipPreflight: true });

//     const traderTokenAccountA = await connection.getTokenAccountBalance(
//       values.holderAccountA
//     );
//     const traderTokenAccountB = await connection.getTokenAccountBalance(
//       values.holderAccountB
//     );
//     expect(traderTokenAccountA.value.amount).to.equal(
//       values.defaultSupply.sub(values.depositAmountA).sub(input).toString()
//     );
//     expect(Number(traderTokenAccountB.value.amount)).to.be.greaterThan(
//       values.defaultSupply.sub(values.depositAmountB).toNumber()
//     );
//     expect(Number(traderTokenAccountB.value.amount)).to.be.lessThan(
//       values.defaultSupply.sub(values.depositAmountB).add(input).toNumber()
//     );
//   });
// });
