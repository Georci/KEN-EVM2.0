pub mod machine;
pub mod error;
pub mod opcode;
pub mod globalState;
pub mod evm;
pub mod utils;

use std::collections::HashMap;
use std::process;
pub use error::exit::*;
pub use machine::Stack::Stack;
pub use machine::Memory::Memory;
pub use globalState::*;
use crate::evm::EVM;
use ethers::types::{Selector, Bytes, Transaction};
use primitive_types::{H160, U256};

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