use ethers_contract::BaseContract;
use ethers_core::abi::parse_abi;
use ethers_providers::{Http, Provider};
use revm::{
    db::{CacheDB, EmptyDB, EthersDB, InMemoryDB},
    primitives::{address, ExecutionResult, Output, TransactTo, U256},
    Database, EVM,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/f153f7b0c19b4e60aa51ecfd9ca2df8b")?;
    let provider = Arc::new(provider);


    let mut ethersdb = EthersDB::new(Arc::clone(&provider), None).unwrap();


    

    Ok(())
}


pub fn create_evm_instance() -> EVM<InMemoryDB> {
    let db = CacheDB::new(EmptyDB::default());
    let mut evm = EVM::new();
    evm.database(db);
    evm
}