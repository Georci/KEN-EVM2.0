// use std::ops::{BitAnd, BitOr};
use primitive_types::U256;
use crate::error::exit::*;
use crate::evm::EVM;

/// and or xor not shl shr sar byte
pub fn and(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    let res = a & b;
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn or(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    let res = a | b;
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn xor(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    let res = a ^ b;
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}


pub fn not(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let a = evm.stack.pop()?;

    let res = !a;
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn shl(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let shift = evm.stack.pop()?;
    let value = evm.stack.pop()?;

    let res = value << shift;
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn shr(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let shift = evm.stack.pop()?;
    let value = evm.stack.pop()?;
    let res = value >> shift;
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

/// 有符号整数右移
pub fn sar(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let shift = evm.stack.pop()?;
    let value = evm.stack.pop()?;

    let res = value >> shift;
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

/// 获取32字节数据中单个字节的值
pub fn byte(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let i = evm.stack.pop()?.as_usize();
    let x = evm.stack.pop()?;

    let res = x.byte(i);
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_byte() {
        let i:usize = 1;
        let x = U256::from(12212);

        let res = x.byte(0);
        println!("{:?}", res);
    }
}