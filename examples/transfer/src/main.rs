use aleo_agent::account::Account;
use aleo_agent::agent::{Agent, TransferArgs, TransferType};
use aleo_agent::{Address, CiphertextRecord, Identifier, Network, PlaintextRecord, PrivateKey, ProgramID, Value, MICROCREDITS};
use anyhow::Result;
use std::str::FromStr;


#[test]
fn test_pvp()-> Result<()>{
    let bob_key = "APrivateKey1zkp7sFhPXR94xWjWbDqMyNs6KsUHfwudMzDfqGvoygcZA25";
    let bob_account = Account::from_private_key(bob_key)?;
    let bob_address = bob_account.address();
    println!("{:?}", bob_account);
    println!("{:?}", bob_address);
    Ok(())
}
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

#[test]
fn test_public_balance(){
    // private key format: APrivateKey1zkp...
    let alice_private_key = "APrivateKey1zkp7WjP4mgYpkak2MDa9oPXuvfm1xU45fuEfFyTSvMMN6xK";
    // build an account using the private key
    let alice_account = Account::from_private_key(alice_private_key).unwrap();
    // build an agent using the account
    let alice_agent = Agent::builder().with_account(alice_account).build();

    let alice_public_balance = alice_agent.get_public_balance().unwrap();
    println!("alice Public Balance : {}", alice_public_balance);

    // private key format: APrivateKey1zkp...
    let bob_private_key = "APrivateKey1zkp7sFhPXR94xWjWbDqMyNs6KsUHfwudMzDfqGvoygcZA25";
    // build an account using the private key
    let bob_account = Account::from_private_key(bob_private_key).unwrap();
    // build an agent using the account
    let bob_agent = Agent::builder().with_account(bob_account).build();

    let bob_public_balance = bob_agent.get_public_balance().unwrap();
    println!("bob Public Balance : {}", bob_public_balance);



   /* let recipient_address = Address::from_str(recipient_address).expect("Invalid recipient address");

    let transfer_args = TransferArgs::from(
        MICROCREDITS, // transfer 1 credit
        recipient_address,
        1, // priority fee
        None, // no record, using public balance
        TransferType::Public, // transfer 1 credit using public balance
    );*/


}
fn main()  {

  /*  // private key format: APrivateKey1zkp...
    let alice_key = "Alice PRIVATE KEY";
    let alice_account = Account::from_private_key(alice_key)?;
    let alice_agent = Agent::builder().with_account(alice_account).build();

    let bob_key = "Bob PRIVATE KEY";
    let bob_account = Account::from_private_key(bob_key)?;
    let bob_address = bob_account.address();

    // get alice public balance
    let public_balance = alice_agent.get_public_balance()?;
    println!("Alice Public Balance : {}", public_balance);

    // public transfer to public account
    let transfer_args = TransferArgs::from(
        MICROCREDITS, // 1 credit
        bob_address.to_owned(),
        1,
        None,
        TransferType::Public,
    );
    let tx_hash = alice_agent.transfer(transfer_args)?;
    println!("tx_hash: {tx_hash}");

    // public transfer to private
    let transfer_args = TransferArgs::from(
        MICROCREDITS, // 1 credit
        bob_address.to_owned(),
        1,
        None,
        TransferType::PublicToPrivate,
    );
    let tx_hash = alice_agent.transfer(transfer_args)?;
    println!("tx_hash: {tx_hash}");

    // private transfer to public
    // plaintext record format : "{owner: aleo1xxxxx.private,microcredits: 1u64.private,_nonce: xxxxxgroup.public}"
    let record = PlaintextRecord::from_str("PLAINTEXT RECORD")?;

    //  decrypt plaintext record from ciphertext record
    //  format: record1xxxxxxxxxx
    let ciphertext_record = CiphertextRecord::from_str("CIPHERTEXT RECORD")?;
    let fee_record = alice_agent.decrypt_ciphertext_record(&ciphertext_record)?;

    let transfer_args = TransferArgs::from(
        MICROCREDITS, // 1 credit
        bob_address.to_owned(),
        10,
        Some(fee_record),
        TransferType::PrivateToPublic(record),
    );
    let tx_hash = alice_agent.transfer(transfer_args)?;
    println!("tx_hash: {tx_hash}");

    // private transfer to private
    let encrypted_fee_record = CiphertextRecord::from_str("record1xxxxxxx")?;
    let record = alice_agent.decrypt_ciphertext_record(&encrypted_fee_record)?;

    let transfer_args = TransferArgs::from(
        MICROCREDITS,
        bob_address.to_owned(),
        1,
        None,
        TransferType::Private(record),
    );
    let tx_hash = alice_agent.transfer(transfer_args)?;
    println!("tx_hash: {tx_hash}");*/

}

