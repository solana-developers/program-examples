import { Account, Pubkey, Result, u8, Signer } from "@solanaturbine/poseidon";

export default class closeAccountProgram {
	static PROGRAM_ID = new Pubkey(
		"7U4SZvsUMjGYCwnzqGGE9enJactKhVPF2EaE7VHtBLTd",
	);

	// create user account

	create_user(user_account: closeAccountState, user: Signer): Result {
		user_account.derive(["user", user.key]).init();

		// Set the initial value to the `user_account` fields
		user_account.user = user.key;
		user_account.bump = user_account.getBump();
	}

	// close user account

	close_user(user_account: closeAccountState, user: Signer): Result {
		user_account.close(user);
	}
}

// setup the state

export interface closeAccountState extends Account {
	user: Pubkey; // This field store the user pub key
	bump: u8; // bump is for PDA (program derieved account, a special type of account which controlled by program on Solana)
}
