## Transfer SOL with Seahorse

Sadly, you can't send SOL(lamports) to another SystemAccount (Public Key) with Seahorse. That's why this Seahorse example looks a little different than the Anchor and Native ones. Here, we initialize a Mock PDA account where we send SOL to using transferLamports, which invokes a CPI with the System Program.
