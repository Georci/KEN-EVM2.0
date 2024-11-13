pub mod machine;
pub mod error;
pub mod opcode;
pub mod globalState;
pub mod evm;
pub mod utils;
pub mod tracer;

use std::collections::HashMap;
use std::{env, process};
use std::str::FromStr;
use std::sync::Arc;
use dotenv::dotenv;
pub use error::exit::*;
pub use machine::Stack::Stack;
pub use machine::Memory::Memory;
pub use globalState::*;
use ethers::types::{Selector, Bytes, Transaction, TxHash};
use primitive_types::{H160, H256, U256};
use ethers::prelude::{Provider, ProviderExt};
use crate::evm::EVM;
use crate::tracer::getAccountState::{get_accounts_state_tx, ISDiff};
use crate::tracer::getTransaction::get_transaction_content;
use crate::utils::u256_to_h256;

pub fn deploy(evm: &mut EVM, bytecode: Bytes, caller: H160, value: U256) -> H160 {
    // 预备部署状态
    evm.bytecode = Some(bytecode);
    evm.is_constructor = true;
    let target_address = evm.deploy_contract(caller, value);
    let to = target_address.unwrap_or_else(|error|{
        println!("depoly error :{:?}", error);
        process::exit(1);
    });
    to
}

/// 该函数用于最外层的函数调用
pub fn external_call(evm: &mut EVM, call:Call) -> Result<Option<Vec<u8>>, Box<dyn ExitError>> {
    // 更新evm状态
    evm.origin = call.from;
    let bytecode = match evm.world_state.get_code(call.to.unwrap()){
        Ok(bytecode) => bytecode,
        Err(e) => {
            println!("encounter error: {:?}", e);
            process::exit(1);
        }
    };
    evm.pc = 0;
    evm.bytecode = Some(bytecode);
    evm.is_constructor = false;
    evm.sub_return_data = None;
    evm.transient_storage = HashMap::new();
    evm.stack = Stack::new(1024);
    evm.memory = Memory::new(1024);
    evm.call_stack.push(call);

    if evm.call_stack.len() != 1 {
        println!("call stack error");
        process::exit(1);
    }
    // 执行
    match evm.interepter(){
        Ok(_) => {
            if evm.return_data.is_some() {
                Ok(evm.return_data.clone())
            }
            else {
                Ok(None)
            }
        }
        Err(e) => {
            println!("external call error:{:?}", e);
            process::exit(1);
        }
    }
}


#[tokio::test]
async fn test_execute_on_chain_tx() {
    dotenv().ok();
    // 1. set provider
    let provider_http_url = env::var("ethereum").unwrap_or_else(|_| String::from("https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b"));
    let provider = Provider::try_connect(provider_http_url.as_str())
        .await
        .unwrap();

    let olympus_dao_tx = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";

    // 2. Obtain the pre_transaction_account_state
    let accounts_state_pre_tx = get_accounts_state_tx(
        Arc::new(provider.clone()),
        H256::from_str(olympus_dao_tx).unwrap(),
        ISDiff::default(),
    ).await;


    // 3. Obtain the transaction context
    let transaction = get_transaction_content(provider, TxHash::from_str(olympus_dao_tx).unwrap()).await;
    let transaction_content = if transaction.is_ok() {
        transaction.unwrap()
    } else {
        println!("encounter error:{:?}", transaction);
        process::exit(1);
    };

    // 4.create a evm
    let mut world_state = WorldState::default();
    accounts_state_pre_tx.iter().for_each(|(addr, accountStateEx)| {
        let accountState:AccountState = AccountState{
            nonce: accountStateEx.clone().nonce,
            balance: accountStateEx.clone().balance,
            code_hash: None,
            storage: accountStateEx.clone().storage,
            code: accountStateEx.code.clone(),
        };
        world_state.new_account(*addr, accountState)
    });
    println!("now world_state is :{:?}", world_state);
    // Call
    let from = transaction_content.from;
    let to =  Some(transaction_content.to);
    let caller = transaction_content.from;
    let address=  Some(transaction_content.to);
    let value=  transaction_content.value;
    let call_data = transaction_content.calldata.clone();
    let call_type = CallType::Call;
    let call_depth = 0;
    let pc =  0;
    println!("call's to address :{:?}", to);
    println!("calldata is :{:?}", call_data);
    let call:Call = Call{
        from,
        to,
        caller,
        address,
        value,
        call_data,
        call_type,
        call_depth,
        pc,
        world_state:world_state.clone(),
    };
    let bytecode = world_state.get_code(to.unwrap()).unwrap();
    println!("execute bytecode:{:?}", bytecode);

    // Block
    let blockhash = transaction_content.block_hash.clone();
    let coinbase = transaction_content.coinbase.clone();
    let timestamp = transaction_content.timestamp;
    let number = transaction_content.block_number;
    let prevrandao = H256::default();
    let gas_limit = transaction_content.gas.clone();
    let chainid = transaction_content.chain_id.clone().unwrap().as_usize();
    let basefee = transaction_content.basefee.clone().unwrap().as_usize();

    let block:Block = Block {
        blockhash,
        coinbase,
        timestamp,
        number,
        prevrandao,
        gas_limit,
        chainid,
        basefee,
    };

    let mut handler = EVM::new(world_state);
    handler.call_stack.push(call);
    handler.origin = transaction_content.from;
    handler.bytecode = Some(bytecode);
    handler.block = Some(block);

    // 5.execution
    match handler.interepter(){
        Ok(_) => {
            println!("execute successful, current evm is :{:?}", handler);
        }
        Err(e) => {
            println!("execute error: {:?}", e);
            process::exit(1);
        }
    };

}