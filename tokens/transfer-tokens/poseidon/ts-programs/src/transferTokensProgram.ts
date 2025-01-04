import {
  AssociatedTokenAccount,
  Mint,
  Pubkey,
  Result,
  Signer,
  SystemAccount,
  TokenProgram,
  UncheckedAccount,
  u8,
  u64,
} from '@solanaturbine/poseidon';

export default class TransferTokensProgram {
  static PROGRAM_ID = new Pubkey('CSqtsYXnt2UfXttszwG6rGFFY7EedJ5kmn4xEyas4LeE');

  createToken(payer: Signer, mintAccount: Mint, decimals: u8, freezeAuthority?: Pubkey): Result {
    // Initialize the mint account with specified decimals
    TokenProgram.initializeMint(
      mintAccount,
      payer, // authority
      decimals,
      freezeAuthority, // freeze authority
    );
  }

  mint(mintAuthority: Signer, mintAccount: Mint, recipient: SystemAccount, associatedTokenAccount: AssociatedTokenAccount, amount: u64): Result {
    associatedTokenAccount.derive(mintAccount, recipient.key).initIfNeeded(mintAuthority);
    TokenProgram.mintTo(mintAccount, associatedTokenAccount, mintAuthority, amount);
  }

  transferTokens(
    sender: Signer,
    senderTokenAccount: AssociatedTokenAccount,
    recipient: SystemAccount,
    recipientTokenAccount: AssociatedTokenAccount,
    mintAccount: Mint,
    amount: u64,
  ) {
    senderTokenAccount.derive(mintAccount, sender.key).initIfNeeded(sender);
    recipientTokenAccount.derive(mintAccount, recipient.key).initIfNeeded(sender);

    TokenProgram.transfer(senderTokenAccount, recipientTokenAccount, sender, amount);
  }
}
