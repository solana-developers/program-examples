import { Idl, Program, utils } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { assert } from 'chai';
import { ProgramTestContext, startAnchor } from 'solana-bankrun';
import pdaIDL from '../target/idl/program_derived_addresses.json';
import { ProgramDerivedAddresses as PDA } from '../target/types/program_derived_addresses';

describe('program_derived_addresses', () => {
  let context: ProgramTestContext;
  let provider: BankrunProvider;
  let pdaProgram: Program<PDA>;
  let pdaProgramPDA: PublicKey;
  let bump: number;

  before(async () => {
    context = await startAnchor('../program_derived_addresses', [], []);
    provider = new BankrunProvider(context);
    pdaProgram = new Program(pdaIDL as Idl, provider) as unknown as Program<PDA>;

    [pdaProgramPDA, bump] = PublicKey.findProgramAddressSync(
      [utils.bytes.utf8.encode('page_visits'), provider.wallet.publicKey.toBuffer()],
      pdaProgram.programId,
    );
  });

  it('create page visits', async () => {
    await pdaProgram.methods
      .createPageVisit()
      .accounts({
        payer: provider.wallet.publicKey,
      })
      .rpc();

    const pageVisit = await pdaProgram.account.pageVisit.fetch(pdaProgramPDA);
    assert.equal(pageVisit.pageVisits, 0);
    assert.equal(pageVisit.bump, bump);
  });

  it('increment page visits', async () => {
    await pdaProgram.methods
      .increment()
      .accounts({
        user: provider.wallet.publicKey,
      })
      .rpc();

    const pageVisit = await pdaProgram.account.pageVisit.fetch(pdaProgramPDA);
    assert.equal(pageVisit.pageVisits, 1);
  });

  it('increment page visits', async () => {
    await pdaProgram.methods
      .increment()
      .accounts({
        user: provider.wallet.publicKey,
      })
      .rpc();

    const pageVisit = await pdaProgram.account.pageVisit.fetch(pdaProgramPDA);
    assert.equal(pageVisit.pageVisits, 2);
  });
});
