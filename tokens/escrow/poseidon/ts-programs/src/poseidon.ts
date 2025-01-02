import { Account, AssociatedTokenAccount, Mint, Pubkey, Seeds, Signer, SystemAccount, TokenAccount, TokenProgram, UncheckedAccount, u64, u8 } from "@solanaturbine/poseidon";

export default class EscrowProgram {
    static PROGRAM_ID = new Pubkey("9gHvfnsqwQ6y5j9NrAQVKmeWGaKUrnv9ZcGrRPn41sRh");

    make(
        maker: Signer,
        escrow: EscrowState,
        makerAta: AssociatedTokenAccount,
        makerMint: Mint,
        takerMint: Mint,
        auth: UncheckedAccount,
        vault: TokenAccount,
        depositAmount: u64,
        offerAmount: u64,
        seed: u64
    ) {
        makerAta.derive(makerMint, maker.key)

        auth.derive(["auth"])

        vault.derive(["vault", escrow.key], makerMint, auth.key).init()

        escrow.derive(["escrow", maker.key, seed.toBytes()])
            .init()

        escrow.authBump = auth.getBump()
        escrow.vaultBump = vault.getBump()
        escrow.escrowBump = escrow.getBump()

        escrow.maker = maker.key;
        escrow.amount = offerAmount;
        escrow.seed = seed;
        escrow.makerMint = makerMint.key;
        escrow.takerMint = takerMint.key;

        TokenProgram.transfer(
            makerAta,
            vault,
            maker,
            depositAmount,
        )
    }

    refund(
        maker: Signer,
        makerAta: AssociatedTokenAccount,
        makerMint: Mint,
        auth: UncheckedAccount,
        vault: TokenAccount,
        escrow: EscrowState
    ) {
        makerAta.derive(makerMint, maker.key);
        escrow.derive(["escrow", maker.key, escrow.seed.toBytes()])
            .has([maker])
            .close(maker)

        auth.derive(["auth"])

        vault.derive(["vault", escrow.key], makerMint, auth.key)

        TokenProgram.transfer(
            vault,
            makerAta,
            auth,
            escrow.amount,
            ["auth", escrow.authBump.toBytes()]
        )
    }

    take(
        taker: Signer,
        maker: SystemAccount,
        makerAta: AssociatedTokenAccount,
        takerAta: AssociatedTokenAccount,
        takerReceiveAta: AssociatedTokenAccount,
        makerMint: Mint,
        takerMint: Mint,
        auth: UncheckedAccount,
        vault: TokenAccount,
        escrow: EscrowState
    ) {
        takerAta
            .derive(makerMint, taker.key)
            .initIfNeeded();

        takerReceiveAta
            .derive(makerMint, taker.key)
            .initIfNeeded()

        makerAta.derive(makerMint, maker.key)

        escrow.derive(["escrow", maker.key, escrow.seed.toBytes()])
            .has([maker, makerMint, takerMint])
            .close(maker)

        auth.derive(["auth"])

        vault.derive(["vault", escrow.key], makerMint, auth.key)

        TokenProgram.transfer(
            takerAta,
            makerAta,
            taker,
            escrow.amount,
        )

        let seeds: Seeds = ["auth", escrow.authBump.toBytes()];

        TokenProgram.transfer(
            vault,
            takerReceiveAta,
            auth,
            escrow.amount,
            seeds
        )
    }
}

export interface EscrowState extends Account {
    maker: Pubkey
    makerMint: Pubkey
    takerMint: Pubkey
    amount: u64
    seed: u64
    authBump: u8
    escrowBump: u8
    vaultBump: u8
}