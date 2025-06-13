use std::{
    sync::Arc,
    env,
};
use dotenv::dotenv;
use ethers::{
    contract::abigen, middleware::SignerMiddleware, providers::{
        Http, Middleware, Provider
    }, signers::{LocalWallet, Signer}, types::{
        Address, 
        U256
    }
};

// Generate the contract bindings for SimpleStorage
// abigen! is a macro that generates Rust bindings for the contract ABI
// Ensure you have the ABI file in the correct location
// .\abi\SimpleStorage.json
// The SimpleStorage contract is expected to have the following functions:
// - set(uint256) to set a value
// - get() to retrieve the stored value
// The event_derives attribute allows the generated events
// to be deserialized and serialized using serde
abigen!(
    SimpleStorage,
    ".\\abi\\SimpleStorage.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
// Box<dyn std::error::Error> is used to allow for any error type to be returned
// This is useful for handling different types of errors that may occur
// in the asynchronous main function
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // load from .env 
    dotenv().ok();

    // Set up the provider and wallet
    let rpc_url = env::var("ALCHEMY_RPC_URL").expect("ALCHEMY_RPC_URL not set");
    let provider = Provider::<Http>::try_from(rpc_url)?;
    // Get the chain ID from the provider
    // This is useful for signing transactions correctly
    // and ensuring the wallet is configured for the correct network
    let chain_id = provider.get_chainid().await?.as_u64();

    // Load the private key from environment variable
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");

    // Create a LocalWallet from the private key
    // and wrap it in a SignerMiddleware to allow signing transactions
    // The LocalWallet is used to sign transactions and messages
    // The SignerMiddleware is used to add signing capabilities to the provider
    // The wallet is then configured with the chain ID to ensure it signs transactions correctly
    // The provider is used to interact with the Ethereum network
    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // The contract address is the address of the deployed contract on the Ethereum network
    let contract_address: Address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set"). parse()?;
    println!("Interacting with contract at: {:?}", contract_address);   

    // Create an instance of the SimpleStorage contract
    // The contract instance allows us to call functions on the contract
    let contract = SimpleStorage::new(contract_address, client);


    // // call set() function
    // // This function sets a value in the contract
    // // The value to set is passed as a U256
    // // The send() method sends the transaction to the Ethereum network
    // // sending 42 as an example value
    // // The transaction is awaited to get the transaction hash
    // let send_value = contract.set(U256::from(42));
    // let tx = send_value.send().await?;
    // println!("Transaction hash: {:?}", tx);


    // comment out set() function code before running get()
    // Call get() function
    // This function retrieves the stored value from the contract
    // The call() method is used to call a view function that does not require a transaction
    // The result is awaited to get the value stored in the contract
    // The value is returned as a U256
    // This is a read-only operation and does not require gas
    let value = contract.get().call().await?;
    println!("Stored value: {}", value);

    // The program has completed successfully
    Ok(())
}