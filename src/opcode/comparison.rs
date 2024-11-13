use ethnum::AsI256;
use primitive_types::U256;
use crate::error::exit::*;
use crate::evm::EVM;

///lt gt slt sgt eq iszero

pub fn lt(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    let res = if a < b {
        U256::one()
    } else { U256::zero() };
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn gt(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    let res = if a > b {
        U256::one()
    } else { U256::zero() };
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn slt(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let a:isize = evm.stack.pop()?.as_usize().try_into().unwrap();
    let b:isize = evm.stack.pop()?.as_usize().try_into().unwrap();

    let res = if a < b {
        U256::one()
    } else { U256::zero() };
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn sgt(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let a:isize = evm.stack.pop()?.as_usize().try_into().unwrap();
    let b:isize = evm.stack.pop()?.as_usize().try_into().unwrap();

    let res = if a > b {
        U256::one()
    } else { U256::zero() };
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn eq(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b= evm.stack.pop()?;

    let res = if a == b {
        U256::one()
    } else { U256::zero() };
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn iszero(evm: &mut EVM) -> Result<(),Box<dyn ExitError>> {
    let a = evm.stack.pop()?;

    let res = if a == U256::zero() {
        U256::one()
    } else { U256::zero() };
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}