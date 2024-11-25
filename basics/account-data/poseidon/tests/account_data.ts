import { Program, Wallet } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { BankrunProvider } from 'anchor-bankrun';
import { expect } from 'chai';
import { startAnchor } from 'solana-bankrun';
import { AddressInfoProgram } from '../target/types/address_info_program';

describe('Address Info Program', () => {
  // Constants
  const PROGRAM_ID = new PublicKey('ChA1o71vBEwkYNs6FnkmG4cxyZWtWkbXSEJ6xP2zaJAq');

  // Test setup
  let program: Program<AddressInfoProgram>;
  let provider: BankrunProvider;
  let addressInfoPda: PublicKey;
  let addressInfoBump: number;
  let owner: Wallet;

  before(async () => {
    try {
      // Initialize program test environment
      const context = await startAnchor(
        '.', // Path to Anchor.toml
        [], // No extra programs needed
        [], // No
      );

      // Set up provider and program
      provider = new BankrunProvider(context);

      // Get program from workspace
      const idl = require('../target/idl/address_info_program.json');
      program = new Program(idl, provider);

      owner = provider.wallet as Wallet;

      // Find PDA
      [addressInfoPda, addressInfoBump] = PublicKey.findProgramAddressSync([Buffer.from('address_info'), owner.publicKey.toBuffer()], PROGRAM_ID);
    } catch (error) {
      console.error('Setup failed:', error);
      throw error;
    }
  });

  describe('initialize', () => {
    it('creates new address info', async () => {
      try {
        const addressData = {
          houseNumber: 42,
          streetNumber: 1000,
          zipCode: 12345,
          countryCode: 1,
        };

        // Initialize address info account
        await program.methods
          .initialize(addressData.houseNumber, addressData.streetNumber, addressData.zipCode, addressData.countryCode)
          .accounts({
            owner: owner.publicKey,
            state: addressInfoPda,
          })
          .rpc();

        // Verify account data
        const account = await program.account.addressInfoState.fetch(addressInfoPda);

        expect(account.owner.toString()).to.equal(provider.wallet.publicKey.toString());
        expect(account.houseNumber).to.equal(addressData.houseNumber);
        expect(account.streetNumber).to.equal(addressData.streetNumber);
        expect(account.zipCode).to.equal(addressData.zipCode);
        expect(account.countryCode).to.equal(addressData.countryCode);
        expect(account.bump).to.equal(addressInfoBump);

        console.log('Address info initialized successfully');
      } catch (error) {
        console.error('Initialize failed:', error);
        throw error;
      }
    });

    it('fails to initialize existing account', async () => {
      try {
        const addressData = {
          houseNumber: 43,
          streetNumber: 1001,
          zipCode: 12346,
          countryCode: 2,
        };

        await program.methods
          .initialize(addressData.houseNumber, addressData.streetNumber, addressData.zipCode, addressData.countryCode)
          .accounts({
            owner: provider.wallet.publicKey,
            state: addressInfoPda,
            systemProgram: SystemProgram.programId,
          })
          .rpc();

        expect.fail('Should have failed');
      } catch (error) {
        expect(error).to.exist;
        console.log('Failed to initialize existing account as expected');
      }
    });
  });
});
