
use ethers::utils::{hex, keccak256};
use ethers::types::Bytes;
use revm_primitives::Address;
use primitive_types::{H160, H256, U256};
use crate::error::exit::*;
use crate::evm::EVM;
use crate::globalState::{Call, CallType};
use crate::machine::Memory::Memory;
use crate::machine::Stack::Stack;
use crate::utils::{u256_to_h160, vec_to_string, vec_to_u256};
use crate::globalState::AccountState;
use crate::opcode::arithmatic::add;

/// create create2 return revert jump jumpi stop jumpdest returndatacopy returndatasize invalid selfdestruct
/// call type: call staticcall delegatecall

pub fn create(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let value = evm.stack.pop()?;
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;
    let init_code = match evm.memory.read(offset, size) {
        Ok(data) => Bytes::from(data),
        Err(e) => return Err(e)
    };
    let nonce = match evm.world_state.get_nonce(evm.call_stack.last().unwrap().caller) {
        Ok(nonce) => nonce,
        Err(e) => return Err(Box::new(OpcodeExecutionError::MaxNonce))
    };
    let address = evm.call_stack.last().unwrap().caller.as_ref();
    let create_address = Address::from_slice(address).create(nonce as u64);
    let account_state = AccountState::new_contract(0, Default::default(), H256::default(), Default::default(), init_code);
    evm.world_state.new_account(u256_to_h160(vec_to_u256(create_address.to_vec())), account_state);
    // 创建完新账户之后，带着value去call这个地址
    let call_result = evm.call(u256_to_h160(vec_to_u256(create_address.to_vec())), value, usize::MAX, CallType::Create);
    if call_result.is_err() {
        evm.stack.push(U256::zero());
    } else {
        evm.stack.push(U256::from(vec_to_u256(create_address.to_vec())));
    }

    evm.pc += 1;
    Ok(())
}

pub fn create2(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let value = evm.stack.pop()?;
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;
    let salt:[u8;32] = evm.stack.pop()?.try_into().unwrap();

    let init_code = match evm.memory.read(offset, size) {
        Ok(data) => Bytes::from(data),
        Err(e) => return Err(e)
    };
    let code_hash = keccak256(init_code.clone());

    let address = evm.call_stack.last().unwrap().caller.as_ref();
    let create_address = Address::from_slice(address).create2(salt, code_hash);

    let account_state = AccountState::new_contract(0, value, H256::default(), Default::default(),  init_code);
    evm.world_state.new_account(u256_to_h160(vec_to_u256(create_address.to_vec())), account_state);
    // 创建完新账户之后，带着value去call这个地址
    let call_result = evm.call(u256_to_h160(vec_to_u256(create_address.to_vec())), value, usize::MAX, CallType::Create2);
    if call_result.is_err() {
        evm.stack.push(U256::zero());
    } else {
        evm.stack.push(U256::from(vec_to_u256(create_address.to_vec())));
    }

    evm.pc += 1;
    Ok(())
}

///# 从memory中copy一段数据放到当前evm的returndata中，并结束当前evm的运行
pub fn _return(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;

    match evm.memory.read(offset, size) {
        Ok(copy_data) => {
            evm.return_data = Some(copy_data.clone());
            println!("call returndata:{:?}", evm.return_data);
            evm.pc = usize::MAX;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

/// 将子调用的returndata
pub fn revert(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;

    evm.return_data = match evm.memory.read(offset,size) {
        Ok(data) => {
            Some(data)
        }
        Err(e) => return Err(e)
    };
    evm.is_revert = true;
    // _evm.evm_stack.push(U256::ZERO);
    evm.pc = usize::MAX;
    Ok(())
}

/// jump 需要对目标pc的opcode进行判断吧，判断是否是jumpdest？
pub fn jump(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let counter = evm.stack.pop()?;

    evm.pc = counter.as_usize();
    Ok(())
}

pub fn jumpi(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let counter = evm.stack.pop()?;
    let b = evm.stack.pop()?;
    if b != U256::zero() {
        evm.pc = counter.as_usize();
    } else {
        evm.pc += 1;
    }
    Ok(())
}

pub fn stop(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    evm.pc = usize::MAX;
    Ok(())
}

pub fn jumpdest(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    evm.pc += 1;
    Ok(())
}

/// 将上一层子调用的returndata copy到memory中
pub fn returndatacopy(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let destOffset = evm.stack.pop()?;
    let offset = evm.stack.pop()?.as_usize();
    let size = evm.stack.pop()?.as_usize();
    // 如果没有returndata则传入一段空数组
    let copy_data:Vec<u8> = match &evm.return_data {
        None => { Vec::new() }
        Some(ret_data) => {
            ret_data[offset..offset + size].to_vec()
        }
    };

    evm.memory.write(destOffset, &*copy_data);
    evm.pc += 1;
    Ok(())
}


pub fn returndatasize(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let ret_data_size = match &evm.sub_return_data {
        None => 0,
        Some(data) => {
            data.len()
        }
    };
    println!("ret_data_size is :{:?}", ret_data_size);

    match evm.stack.push(U256::from(ret_data_size)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn invalid(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    evm.pc += 1;
    Ok(())
}

/// 将当前执行code地址上的全部ether发送到指定address
pub fn selfdestruct(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let address = evm.stack.pop()?;
    let value = match evm.world_state.get_balance(u256_to_h160(address)) {
        Ok(balance) => {
            balance
        }
        Err(e) => { return Err(e); }
    };
    match evm.world_state.set_balance(evm.call_stack.last().unwrap().address.unwrap(), U256::zero()){
        Ok(_) => {}
        Err(e) => { return Err(e); }
    };
    match evm.world_state.set_balance(u256_to_h160(address), value){
        Ok(_) => {}
        Err(e) => { return Err(e); }
    };
    evm.world_state.remove_account(evm.call_stack.last().unwrap().address.unwrap());
    evm.pc = usize::MAX;
    Ok(())
}

pub fn call(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let (gas, address, value, argsOffset, argsSize, retOffset, retSize) =
        call_pop(evm, CallType::Call);
    match call_core(
        evm,
        gas,
        address,
        value.unwrap(),
        argsOffset,
        argsSize,
        retOffset,
        retSize,
        CallType::Call,
    ) {
        Ok(_) => { Ok(()) }
        Err(e) => Err(e)
    }
}

/// todo!:目前来说staticcall还没有对状态更改进行检查
pub fn staticcall(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let (gas, address, value, argsOffset, argsSize, retOffset, retSize) =
        call_pop(evm, CallType::StaticCall);
    println!("static call to:{}, callvalue:{}", address, value.unwrap_or(U256::zero()));
    match call_core(
        evm,
        gas,
        address,
        value.unwrap_or(U256::zero()),
        argsOffset,
        argsSize,
        retOffset,
        retSize,
        CallType::Call,
    ) {
        Ok(_) => { Ok(()) }
        Err(e) => Err(e)
    }
}

pub fn delegatecall(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let (gas, address, value, argsOffset, argsSize, retOffset, retSize) =
        call_pop(evm, CallType::DelegateCall);
    match call_core(
        evm,
        gas,
        address,
        value.unwrap(),
        argsOffset,
        argsSize,
        retOffset,
        retSize,
        CallType::Call,
    ) {
        Ok(_) => { Ok(()) }
        Err(e) => Err(e)
    }
}

pub fn callcode(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    Err(Box::new(OpcodeExecutionError::NotImplemented(0xF2)))
}

pub fn call_pop(
    _evm: &mut EVM,
    call_type: CallType,
) -> (U256, U256, Option<U256>, U256, U256, U256, U256) {
    match call_type {
        CallType::Call => {
            let gas = _evm.stack.pop().unwrap();
            let address = _evm.stack.pop().unwrap();
            let value = _evm.stack.pop().unwrap();
            let argsOffset = _evm.stack.pop().unwrap();
            let argsSize = _evm.stack.pop().unwrap();
            let retOffset = _evm.stack.pop().unwrap();
            let retSize = _evm.stack.pop().unwrap();
            (
                gas,
                address,
                Some(value),
                argsOffset,
                argsSize,
                retOffset,
                retSize,
            )
        }
        CallType::StaticCall | CallType::DelegateCall => {
            let gas = _evm.stack.pop().unwrap();
            let address = _evm.stack.pop().unwrap();
            let argsOffset = _evm.stack.pop().unwrap();
            let argsSize = _evm.stack.pop().unwrap();
            let retOffset = _evm.stack.pop().unwrap();
            let retSize = _evm.stack.pop().unwrap();
            (gas, address, None, argsOffset, argsSize, retOffset, retSize)
        }
        _ => {
            panic!("error pop for call");
        }
    }
}

pub fn call_core(
    evm: &mut EVM,
    gas:U256,
    address:U256,
    value:U256,
    argsOffset:U256,
    argsSize:U256,
    retOffset:U256,
    retSize:U256,
    call_type: CallType
    ) -> Result<(), Box<dyn ExitError>> {
    let calldata = match evm.memory.read(argsOffset, argsSize){
        Ok(data) => data,
        Err(e) => return Err(e)
    };
    let calldata = Bytes::from(calldata);
    // call深度限制
    if evm.call_depth > 1024 {
        return Err(Box::new(OpcodeExecutionError::CallTooDeep));
    }

    //保存上下文
    let before_code = evm.bytecode.clone();
    //将当前call组成Call压入栈中
    let now_call = evm.call_stack.last().unwrap();
    // 如果是最外层的call操作应该是不需要在创建一个Call的，就比如说userA -> contractB, 这个call应该是直接就在外层调用函数的时候由用户构建
    // 但是还是有一个问题，就是如果是由用户直接参与的外部调用，应该不会遇到call类型操作码吧 (√)
    println!("now execution address: {}", address);
    let _call = Call {
        from: now_call.to.unwrap(),
        to: Some(u256_to_h160(address)),
        value,
        call_data: calldata,
        caller: if call_type == CallType::DelegateCall{
            now_call.caller
        } else {
            now_call.address.unwrap()
        },
        // 这里的is_err操作难道是如果目标地址没有code，则重新回到当前地址执行？
        address: if evm.world_state.get_code(u256_to_h160(address)).is_err(){
            now_call.address
        } else if call_type.eq(&CallType::DelegateCall){
            now_call.to
        } else {
            Some(u256_to_h160(address))
        },
        call_type,
        call_depth: evm.call_depth,
        pc: evm.pc,
        world_state: evm.world_state.clone(),
    };
    evm.evm_stack.push(evm.stack.clone());
    evm.memory_stack.push(evm.memory.clone());
    evm.call_stack.push(_call.clone());
    evm.call_depth += 1;
    if evm.world_state.get_code(u256_to_h160(address)).is_err() {
        // 如果要call调用的地址不是合约地址，则提前关闭调用
        evm.pc += 1;
        evm.call_depth -= 1;
        evm.evm_stack.pop();
        evm.call_stack.pop();
        evm.memory_stack.pop();
        match evm.stack.push(U256::one()) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }
        return Ok(());
    } else {
        evm.bytecode = Some(evm.world_state.get_code(u256_to_h160(address)).unwrap());
    }
    evm.pc = 0;
    evm.stack = Stack::new(1024);
    evm.memory = Memory::new(1024);
    if value != U256::zero() {
        evm.world_state.sub_balance(_call.from, value);
        evm.world_state
            .add_balance(u256_to_h160(address), value);
    }
    // =========================执行字节码==========================
    let _ = evm.interepter();
    // =========================恢复上下文==========================
    evm.bytecode = before_code;
    evm.pc = _call.pc + 1;
    evm.call_depth -= 1;
    evm.stack = evm.evm_stack.pop().unwrap();
    evm.memory = evm.memory_stack.pop().unwrap();
    evm.call_stack.pop();
    // 在每次call结束后，sub_returndata就是当前的returndata，这里我可以理解sub_return_data是存放子调用的return数据，return_data存放的是当前调用的return数据，但是interepter()中好像是没有处理return_data的逻辑
    evm.sub_return_data = match evm.return_data.clone(){
        None => {None}
        Some(ret_data) => {
            evm.memory.write(retOffset, &*ret_data);
            Some(ret_data)
        }
    };
    // 下一个call的returndata还是空
    evm.return_data = None;
    if _call.call_type.eq(&CallType::StaticCall) {
        // 恢复上下文
        evm.world_state = _call.world_state;
    }
    if evm.is_revert {
        evm.stack.push(U256::zero())?;
    } else {
        evm.stack.push(U256::one())?;
    };
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec() {
        let a:Vec<u8> = Vec::new();
        println!("{:?}", a);
    }
}