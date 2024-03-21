// Rust code for token swapping (native)
fn swap_tokens(sender: Account, recipient: Account, amount: u64) {
    // Exchange one token for another
    sender.token_balance -= amount;
    recipient.token_balance += amount;
}
