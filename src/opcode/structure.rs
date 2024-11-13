use std::fs::metadata;
use ethers::utils::__serde_json::to_vec;
use primitive_types::{H256, U256};
use crate::error::exit::*;
use crate::evm::EVM;
use crate::utils::{h256_to_u256, u256_to_h256, vec_to_u256, u256_to_vec};

// push dup swap sstore sload mstore mstore8 mload mcopy pop
pub fn _push(evm: &mut EVM, value_len: usize) -> Result<(), Box<dyn ExitError>> {
    let put_value = if value_len == 0 {
        U256::zero()
    } else {
        let start = evm.pc + 1;
        let end = start + value_len;
        U256::from(&evm.bytecode.as_ref().unwrap()[start..end])
    };

    match evm.stack.push(put_value) {
        Ok(_) => {
            evm.pc += value_len + 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}


pub fn _dup(evm: &mut EVM, dum_loc: usize) -> Result<(), Box<dyn ExitError>> {
    let loc = dum_loc - 1;
    let dup_value = match evm.stack.peek(loc) {
        Ok(value) => {
            value
        }
        Err(e) => { return Err(e); }
    };
    match evm.stack.push(dup_value){
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => { Err(e) }
    }
}

pub fn _swap(evm: &mut EVM, swp_loc: usize) -> Result<(), Box<dyn ExitError>> {
    match evm.stack.swap(swp_loc) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn sstore(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let key = evm.stack.pop()?;
    let value = evm.stack.pop()?;
    let address = evm.call_stack.last().unwrap().address.unwrap();

    match evm.world_state.insert_storage_value(address, u256_to_h256(key), u256_to_h256(value)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn sload(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let key = evm.stack.pop()?;
    let value = evm.world_state.get_storage_value(evm.call_stack.last().unwrap().address.unwrap(), u256_to_h256(key)).unwrap_or_else(|_| { H256::zero() });

    match evm.stack.push(h256_to_u256(value)){
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => { Err(e) }
    }
}

pub fn msotre(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let value = evm.stack.pop()?;
    match evm.memory.mstore(offset, &*u256_to_vec(value)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn msotre8(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let value = evm.stack.pop()?;
    match evm.memory.mstore8(offset, &*u256_to_vec(value)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn mload(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let offset = evm.stack.pop()?;
    let value = match evm.memory.mload(offset) {
        Ok(value) => value,
        Err(e) => return Err(e)
    };
    match evm.stack.push(vec_to_u256(value)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn mcopy(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let destOffset = evm.stack.pop()?;
    let offset = evm.stack.pop()?;
    let size = evm.stack.pop()?;

    let data_cpoy = match evm.memory.read(offset, size){
        Ok(data) => {
            data
        }
        Err(e) => return Err(e)
    };

    evm.memory.write(destOffset, &*data_cpoy);
    evm.pc += 1;
    Ok(())
}

pub fn pop(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let _ = evm.stack.pop();
    evm.pc += 1;
    Ok(())
}

pub fn push0(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(evm, 0)
}

pub fn push1(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(evm, 1)
}
pub fn push2(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(evm, 2)
}

pub fn push3(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(evm, 3)
}

pub fn push4(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(evm, 4)
}

pub fn push5(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(evm, 5)
}

pub fn push6(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 6)
}

pub fn push7(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 7)
}

pub fn push8(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 8)
}

pub fn push9(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 9)
}

pub fn push10(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 10)
}

pub fn push11(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 11)
}

pub fn push12(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 12)
}

pub fn push13(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 13)
}

pub fn push14(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 14)
}



pub fn push15(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 15)
}

pub fn push16(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 16)
}

pub fn push17(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 17)
}

pub fn push18(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 18)
}

pub fn push19(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 19)
}

pub fn push20(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 20)
}

pub fn push21(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 21)
}

pub fn push22(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 22)
}

pub fn push23(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 23)
}

pub fn push24(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 24)
}

pub fn push25(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 25)
}

pub fn push26(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 26)
}

pub fn push27(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 27)
}

pub fn push28(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 28)
}

pub fn push29(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 29)
}

pub fn push30(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 30)
}

pub fn push31(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 31)
}

pub fn push32(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _push(_evm, 32)
}


pub fn dup1(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 1)
}

pub fn dup2(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 2)
}

pub fn dup3(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 3)
}

pub fn dup4(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 4)
}

pub fn dup5(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 5)
}

pub fn dup6(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 6)
}

pub fn dup7(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 7)
}

pub fn dup8(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 8)
}

pub fn dup9(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 9)
}

pub fn dup10(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 10)
}

pub fn dup11(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 11)
}

pub fn dup12(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 12)
}

pub fn dup13(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 13)
}

pub fn dup14(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 14)
}

pub fn dup15(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 15)
}

pub fn dup16(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _dup(_evm, 16)
}
pub fn swap1(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 1)
}

pub fn swap2(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 2)
}

pub fn swap3(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 3)
}

pub fn swap4(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 4)
}

pub fn swap5(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 5)
}

pub fn swap6(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 6)
}

pub fn swap7(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 7)
}

pub fn swap8(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 8)
}

pub fn swap9(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 9)
}

pub fn swap10(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 10)
}

pub fn swap11(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 11)
}

pub fn swap12(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 12)
}

pub fn swap13(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 13)
}

pub fn swap14(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 14)
}

pub fn swap15(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 15)
}

pub fn swap16(_evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    _swap(_evm, 16)
}
