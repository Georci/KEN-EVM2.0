use ethers::prelude::Bytes;
use primitive_types::{H256, H160, U256};
use ethers::utils::keccak256 as ethers_keccak256;
use crate::error::exit::*;
use crate::evm::EVM;
use crate::utils::{h160_to_u256, h256_to_u256, u256_to_h256, pad_right, pad_left, u256_to_h160};


pub fn pc(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let pc = evm.pc;
    match evm.stack.push(U256::from(pc)){
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn msize(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let length = evm.memory.len();
    match evm.stack.push(U256::from(length)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }

}

pub fn chainid(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let chainid = match &evm.block {
        Some(b) => b.chainid.clone(),
        None => 1
    };
    match evm.stack.push(U256::from(chainid)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}


pub fn prevrandao(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let prevrandao = match &evm.block {
        Some(b) => b.prevrandao.clone(),
        None => H256::random()
    };
    match evm.stack.push(h256_to_u256(prevrandao)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn number(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let number = match &evm.block {
        Some(b) => b.number.clone(),
        None => 1
    };
    match evm.stack.push(U256::from(number)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn basefee(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let basefee = match &evm.block {
        Some(b) => b.basefee.clone(),
        None => 1
    };
    match evm.stack.push(U256::from(basefee)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn blockhash(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let blockhash = match &evm.block {
        Some(b) => b.blockhash.clone(),
        None => H256::random()
    };
    match evm.stack.push(h256_to_u256(blockhash)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn gaslimit(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let gaslimit = match &evm.block {
        Some(b) => b.gas_limit.clone(),
        None => U256::MAX
    };
    match evm.stack.push(U256::from(gaslimit)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn timestamp(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let timestamp = match &evm.block {
        Some(b) => b.timestamp.clone(),
        None => 0
    };
    match evm.stack.push(U256::from(timestamp)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn coinbase(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let coinbase = match &evm.block {
        Some(b) => b.coinbase.clone(),
        None => H160::random()
    };
    match evm.stack.push(h160_to_u256(coinbase)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn gasprice(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let gasprice = evm.block.as_ref().unwrap().basefee;
    match evm.stack.push(U256::from(gasprice)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

///将当前evm执行的code从offset开始，长度为size的字节码复制到memory从destOffset开始的空间
pub fn codecopy(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let destOffset = evm.stack.pop()?;
    let offset = evm.stack.pop().unwrap().as_usize();
    let size = evm.stack.pop().unwrap().as_usize();

    let bytecode_slice = &evm.bytecode.as_ref().unwrap()[offset..offset + size].to_vec();
    evm.memory.write(destOffset, &bytecode_slice);

    evm.pc += 1;
    Ok(())
}

///获取当前正在执行合约的bytecode长度
pub fn codesize(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {

    let code = match evm.world_state.get_code(evm.call_stack.last().unwrap().address.unwrap()) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };
    match evm.stack.push(U256::from(code.len())) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn address(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let address = evm.call_stack.last().unwrap().address;
    match evm.stack.push(h160_to_u256(address.unwrap())) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn origin(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let origin = &evm.origin;
    match evm.stack.push(h160_to_u256(*origin)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn caller(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let caller = &evm.call_stack.last().unwrap().caller;
    match evm.stack.push(h160_to_u256(*caller)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn callvalue(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let callvalue = &evm.call_stack.last().unwrap().value;
    match evm.stack.push(U256::from(callvalue)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// 加载指定位置的32字节inputdata到栈中
pub fn calldataload(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let byte_offset = evm.stack.pop().unwrap().as_usize();
    let call_data = evm.call_stack.last().unwrap().call_data.clone();
    let mut call_data_slice:[u8; 32] = if call_data.len() > byte_offset + 32 {
        let mut slice = [0u8; 32];
        slice.copy_from_slice(&call_data[byte_offset..byte_offset + 32]);
        slice
    } else {
        pad_right(&call_data[byte_offset..])
    };
    match evm.stack.push(U256::from_big_endian(&call_data_slice)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// 返回当前calldata数据长度
pub fn calldatasize(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let size = evm.call_stack.last().unwrap().call_data.len();
    match evm.stack.push(U256::from(size)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// 复制calldata中指定位置指定长度的数据到 memory 中
pub fn calldatacopy(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let destOffset = evm.stack.pop().unwrap().as_usize();
    let offset = evm.stack.pop().unwrap().as_usize();
    let size = evm.stack.pop().unwrap().as_usize();
    let copy_data_slice = &evm.call_stack.last().unwrap().call_data.clone()[offset..offset+size].to_vec();

    evm.memory.write(U256::from(destOffset), copy_data_slice);
    evm.pc += 1;
    Ok(())
}




pub fn balance(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let address = evm.stack.pop()?;
    let value = evm.world_state.get_balance(u256_to_h160(address)).unwrap();
    match evm.stack.push(value) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// 获取一段memory中的数据并对他进行哈希，将结果推入栈中
pub fn keccak256(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;
    match evm.memory.read(offset, size){
     Ok(data) => {
         let keccak_data = ethers_keccak256(data);
         match evm.stack.push(U256::from(keccak_data)) {
             Ok(_) => {
                 evm.pc += 1;
                 Ok(())
             }
             Err(e) => Err(e),
         }
     }
        Err(e) => Err(e)
    }
}


#[derive(Clone, Debug)]
pub struct Log{
    address: H160,
    topics: Vec<U256>,
    data: Vec<u8>,
}

/// 暂时不实现吧
/// log0将memory中的某段数据作为data，函数参数中有index标记的参数会被写入topics中，其余的写入data中
pub fn log0(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;
    let data:Vec<u8> = if size == U256::zero() {
        Vec::new()
    } else {
        evm.memory.read(offset, size)?
    };
    let address = evm.call_stack.last().unwrap().caller.clone();
    evm.pc += 1;
    Ok(())

}
pub fn log1(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;
    let data:Vec<u8> = if size == U256::zero() {
        Vec::new()
    } else {
        evm.memory.read(offset, size)?
    };
    let address = evm.call_stack.last().unwrap().caller.clone();
    let mut topics = Vec::new();
    let value = evm.stack.pop()?;
    topics.push(value);
    evm.pc += 1;
    Ok(())
}

pub fn log2(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;
    let data:Vec<u8> = if size == U256::zero() {
        Vec::new()
    } else {
        evm.memory.read(offset, size)?
    };
    let address = evm.call_stack.last().unwrap().caller.clone();
    let mut topics = Vec::new();
    let value1 = evm.stack.pop()?;
    let value2 = evm.stack.pop()?;
    topics.push(value1);
    evm.pc += 1;
    Ok(())
}
pub fn log3(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;
    let data:Vec<u8> = if size == U256::zero() {
        Vec::new()
    } else {
        evm.memory.read(offset, size)?
    };
    let address = evm.call_stack.last().unwrap().caller.clone();
    let mut topics = Vec::new();
    let value = evm.stack.pop()?;
    let value2 = evm.stack.pop()?;
    let value3 = evm.stack.pop()?;
    topics.push(value);
    evm.pc += 1;
    Ok(())
}

pub fn log4(evm : &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;
    let data:Vec<u8> = if size == U256::zero() {
        Vec::new()
    } else {
        evm.memory.read(offset, size)?
    };
    let address = evm.call_stack.last().unwrap().caller.clone();
    let mut topics = Vec::new();
    let value = evm.stack.pop()?;
    let value2 = evm.stack.pop()?;
    let value3 = evm.stack.pop()?;
    let value4 = evm.stack.pop()?;
    topics.push(value);
    evm.pc += 1;
    Ok(())
}

pub fn gas(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let _ = _evm.stack.push(U256::MAX);
    _evm.pc += 1;
    Ok(())
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_calldataload() {

    }
}



