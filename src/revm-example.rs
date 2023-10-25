use revm::{
    db::{CacheDB, EmprtyDB, InMemory},
    EVM,
};

pub fn create_evm_isntance() -> EVM<InMemoryDB> {
    let db = CacheDB::new(EmptyDB::default()); 
    let mut evm = EVM::new();
    evm.database(db); 
    evm
}

// add this 👇
pub fn evm_env_setup(evm: &mut EVM<InMemoryDB>) {
    // overriding some default env values to make it more efficient for testing
    evm.env.cfg.limit_contract_code_size = Some(0x100000);
    evm.env.cfg.disable_block_gas_limit = true;
    evm.env.cfg.disable_base_fee = true;
}