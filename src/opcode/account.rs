use ethers::prelude::Bytes;
use primitive_types::U256;
use crate::error::exit::*;
use crate::evm::EVM;
use crate::globalState::AccountState;
use crate::utils::{h256_to_u256, u256_to_h160};

///_extcodecopy _extcodehash _extcodesize _selfbalance
/// 将某个地址的code复制到memory中
pub fn extcodecopy(evm: &mut EVM) -> Result<(), Box<dyn ExitError>>{
    let address = evm.stack.pop()?;
    let destOffset = evm.stack.pop()?;
    let offset = evm.stack.pop()?.as_usize();
    let size = evm.stack.pop()?.as_usize();
    // 获取某个地址的code，eoa账户没有code
    match evm.world_state.get_code(u256_to_h160(address)) {
        Ok(bytecode) => {
            let bytecode_to_copy = &bytecode[offset..offset+size].to_vec();
            evm.memory.write(destOffset, &bytecode_to_copy);
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn extcodehash(evm: &mut EVM) -> Result<(), Box<dyn ExitError>>{
    let address = evm.stack.pop()?;
    match evm.world_state.get_code_hash(u256_to_h160(address)) {
        Ok(code_hash) => {
            match evm.stack.push(h256_to_u256(code_hash)) {
                Ok(_) => {
                    evm.pc += 1;
                    Ok(())
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}

/// 获取某个地址的code长度
pub fn extcodesize(evm: &mut EVM) -> Result<(), Box<dyn ExitError>>{
    let address = evm.stack.pop()?;
    match evm.world_state.get_code(u256_to_h160(address)) {
        Ok(bytecode) => {
            let length = U256::from(bytecode.len());
            match evm.stack.push(length) {
                Ok(_) => {
                    evm.pc += 1;
                    Ok(())
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}

/// 获取当前正在执行账户余额
pub fn selfbalance(evm: &mut EVM) -> Result<(), Box<dyn ExitError>>{
    let address = evm.call_stack.last().unwrap().clone().address;
    match evm.world_state.get_balance(address.unwrap()) {
        Ok(balance) => {
            match evm.stack.push(balance) {
                Ok(_) => {
                    evm.pc += 1;
                    Ok(())
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}