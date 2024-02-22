use ethers::{
    contract::Contract,
    providers::{Http},
    types::{Address, U256},
    abi::Abi,
    prelude::{Provider},
};
use std::fs;
use std::{
    sync::{Arc},
    fs::{File},
    io::{Read},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://rpc-mainnet.maticvigil.com".to_string(); // Example Polygon node URL
    let client = Provider::<Http>::try_from(url).unwrap();
    let token_address = "0xc2132D05D31c914a87C6611C10748AEb04B58e8F".parse::<Address>()?;
    
    // Load ABI from JSON file
    let mut file = File::open("./src/erc20abi.json")?;
    let mut abi_json = String::new();
    file.read_to_string(&mut abi_json)?;
    let token_abi: Abi = serde_json::from_str(&abi_json)?;
    
    let contract = Contract::new(token_address, token_abi, Arc::new(client));
    let account_address = "0x3bF39a306D35b79dDF82AF9060158f58eE5B5959".parse::<Address>()?;
    println!("hello");
    let method = contract.method::<_, U256>("balanceOf", account_address)?;
    let balance: U256 = method.call().await?;

    println!("The balance of account is {} Wei", balance);
    Ok(())
}
