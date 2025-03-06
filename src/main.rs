use aleo_agent::account::Account;
use aleo_agent::agent::{Agent, TransferArgs, TransferType};
fn main() {

    // private key format: APrivateKey1zkp...
    let alice_key = "APrivateKey1zkp7sFhPXR94xWjWbDqMyNs6KsUHfwudMzDfqGvoygcZA25";

    // 手动处理错误，而不是使用 ?
    let alice_account = match Account::from_private_key(alice_key) {
        Ok(account) => account,
        Err(err) => {
            println!("创建 Alice 账户失败: {}", err);
            return;
        }
    };

    let alice_agent = Agent::builder().with_account(alice_account).build();

    let bob_key = "APrivateKey1zkp7WjP4mgYpkak2MDa9oPXuvfm1xU45fuEfFyTSvMMN6xK";
    let bob_account = match Account::from_private_key(bob_key) {
        Ok(account) => account,
        Err(err) => {
            println!("创建 Bob 账户失败: {}", err);
            return;
        }
    };

    let bob_address = bob_account.address();

    let transfer_args = TransferArgs::from(
        1_000_000,
        bob_address.to_owned(),
        0,
        None,
        TransferType::Public,
    );

    const CREDITS_PROGRAM_ID: &str = "credits.aleo";

    // 手动处理最后的错误
    match alice_agent.transfer(CREDITS_PROGRAM_ID, transfer_args) {
        Ok(tx_hash) => println!("execution: {tx_hash}"),
        Err(err) => println!("转账失败: {}", err),
    }
}