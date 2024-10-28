


use anyhow::Result;
use aleo_agent::account::Account;
use aleo_agent::agent::{Agent, TransferArgs, TransferType};

#[test]

fn test_transfer() -> Result<()> {

    // private key format: APrivateKey1zkp...
    let alice_key = "APrivateKey1zkp7WjP4mgYpkak2MDa9oPXuvfm1xU45fuEfFyTSvMMN6xK";
    let alice_account = Account::from_private_key(alice_key)?;
    let alice_agent = Agent::builder().with_account(alice_account).build();

    let bob_key = "APrivateKey1zkp7sFhPXR94xWjWbDqMyNs6KsUHfwudMzDfqGvoygcZA25";
    let bob_account = Account::from_private_key(bob_key)?;
    let bob_address = bob_account.address();

    //alice_account
    //Account{privateKey:"APrivateKey1zkp7WjP4mgYpkak2MDa9oPXuvfm1xU45fuEfFyTSvMMN6xK",
    //	viewKey:"AViewKey1kAUqjVfLhqzvyumzgtadUgCXjBmkfk3cukHJoS7h3pndE",
    //	address:"aleo1qs5qdk6le32z3wv73f6zl5cswjwew3qjpxkwrqxnedxxckgfeuxsd0whp8"}
    //          ·

    //bob_account
    //Account{privateKey:"APrivateKey1zkp7sFhPXR94xWjWbDqMyNs6KsUHfwudMzDfqGvoygcZA25",
    //	viewKey:"AViewKey1nKwqw3B7DDkGdu4PsoN9tjy1Enuo8GahZta8FfkqMipS",
    //	address:"aleo1tgys9nhnytn2femfggn7638fj6e8rera6e0xxdcvp36crzv28y8sncnzaf"}


    // get alice public balance
    /*  let public_balance = alice_agent.get_public_balance()?;
      println!("Alice Public Balance : {}", public_balance);*/

    // public transfer to public account
    let transfer_args = TransferArgs::from(
        /* amount: u64,
         recipient_address: Address,
         priority_fee: u64,
         fee_record: Option<PlaintextRecord>,
         transfer_type: TransferType,*/
        10_000_000,
        //MICROCREDITS, // 1 credit
        bob_address.to_owned(),
        600_000,
        None,
        TransferType::Public,

    );
    pub const CREDITS_PROGRAM_ID: &str = "credits.aleo";

    let tx_hash = alice_agent.transfer(CREDITS_PROGRAM_ID, transfer_args)?;
    println!("execution: {tx_hash}");
    Ok(())
}