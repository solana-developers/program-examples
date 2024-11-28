import { Pubkey, Signer, SystemAccount, u64, SystemProgram } from "@solanaturbine/poseidon";

export default class poseidon {
    static PROGRAM_ID = new Pubkey("7ogRvKFEYpbceu1CorL3V9jFdwPKkWdpUgUqNdsVswbG");

    transferSol(payer: Signer, recipient: SystemAccount, amount: u64) {
        SystemProgram.transfer(
            payer,
            recipient,
            amount
        );
    }
}
