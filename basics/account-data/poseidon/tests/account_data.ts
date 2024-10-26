import { Program, Wallet } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { startAnchor } from 'solana-bankrun';
import { AddressInfoProgram } from '../target/types/address_info_program';

describe('Address Info Program', () => {
  // Constants
  const IDL = require('../target/idl/address_info_program.json');
  const PROGRAM_ID = new PublicKey(IDL.address);
  const ADDRESS_SEED = 'address_info';

  // Test setup
  let addressProgram: Program<AddressInfoProgram>;
  let provider: BankrunProvider;
  let owner: Wallet;
  let addressInfoPda: PublicKey;
  let addressInfoBump: number;

  before(async () => {
    // Start bankrun with Anchor workspace
    const context = await startAnchor(
      '.', // Path to Anchor.toml
      [], // No extra programs needed
      [], // No extra accounts needed
    );

    // Set up provider and program
    provider = new BankrunProvider(context);
    addressProgram = new Program(IDL, provider);

    owner = provider.wallet as Wallet;

    // Find PDA for address info account
    [addressInfoPda, addressInfoBump] = PublicKey.findProgramAddressSync([Buffer.from(ADDRESS_SEED), owner.publicKey.toBuffer()], PROGRAM_ID);
  });

  describe('initialize', () => {
    it('creates new address info', async () => {
      const addressData = {
        houseNumber: 42,
        streetNumber: 1000,
        zipCode: 12345,
        countryCode: 1,
      };

      // Initialize address info account
      await addressProgram.methods
        .initialize(addressData.houseNumber, addressData.streetNumber, addressData.zipCode, addressData.countryCode)
        .accounts({
          owner: owner.publicKey,
          addressInfo: addressInfoPda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      // Fetch and verify account data
      const account = await addressProgram.account.addressInfoState.fetch(addressInfoPda);

      expect(account.owner.equals(owner.publicKey)).to.be.true;
      expect(account.houseNumber).to.equal(addressData.houseNumber);
      expect(account.streetNumber).to.equal(addressData.streetNumber);
      expect(account.zipCode).to.equal(addressData.zipCode);
      expect(account.countryCode).to.equal(addressData.countryCode);
      expect(account.bump).to.equal(addressInfoBump);
    });

    it('fails to initialize existing account', async () => {
      const addressData = {
        houseNumber: 43,
        streetNumber: 1001,
        zipCode: 12346,
        countryCode: 2,
      };

      try {
        await addressProgram.methods
          .initialize(addressData.houseNumber, addressData.streetNumber, addressData.zipCode, addressData.countryCode)
          .accounts({
            owner: owner.publicKey,
            addressInfo: addressInfoPda,
            systemProgram: SystemProgram.programId,
          })
          .rpc();

        expect.fail('Should have failed');
      } catch (err) {
        expect(err).to.exist;
      }
    });
  });

  describe('edit', () => {
    it('edits existing address info', async () => {
      const newData = {
        houseNumber: 44,
        streetNumber: 1002,
        zipCode: 12347,
        countryCode: 3,
      };

      await addressProgram.methods
        .edit(newData.houseNumber, newData.streetNumber, newData.zipCode, newData.countryCode)
        .accounts({
          owner: owner.publicKey,
          addressInfo: addressInfoPda,
        })
        .rpc();

      const account = await addressProgram.account.addressInfoState.fetch(addressInfoPda);

      expect(account.houseNumber).to.equal(newData.houseNumber);
      expect(account.streetNumber).to.equal(newData.streetNumber);
      expect(account.zipCode).to.equal(newData.zipCode);
      expect(account.countryCode).to.equal(newData.countryCode);
    });
  });
});
