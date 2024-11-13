use std::ops::{Add, Div, Mul, Sub};
use primitive_types::{U256};

use crate::error::exit::*;
use crate::evm::EVM;

/// add sub mul div mod addmod mulmod smod exp signextend sdiv
pub fn add(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    let res = a.add(b);
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn sub(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    let res = a.sub(b);
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn mul(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    let res = a.mul(b);
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn div(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    if b == U256::zero() { return Err(Box::new(OpcodeExecutionError::DivisionByZero)) };
    let res = a.div(b);
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn _mod(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;

    let res = a % b;
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn addmod(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;
    let N = evm.stack.pop()?;

    let res = a.add(b) % N;
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn mulmod(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let b = evm.stack.pop()?;
    let N = evm.stack.pop()?;

    let res = a.mul(b) % N;
    match evm.stack.push(res) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn smod(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a:isize = evm.stack.pop().unwrap().as_usize().try_into().unwrap();
    let b:isize = evm.stack.pop().unwrap().as_usize().try_into().unwrap();

    let res = a % b;
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn exp(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a = evm.stack.pop()?;
    let exponent = evm.stack.pop()?;

    let res = a.pow(exponent);
    match evm.stack.push(U256::from(res)) {
        Ok(_) => {
            evm.pc += 1;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn signextend(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let b_size = evm.stack.pop()?;
    let x = evm.stack.pop()?;
    Ok(())
}

pub fn sdiv(evm: &mut EVM) -> Result<(), Box<dyn ExitError>> {
    let a:isize = evm.stack.pop()?.as_usize().try_into().unwrap();
    let b:isize = evm.stack.pop()?.as_usize().try_into().unwrap();

    let res = a.div(b);
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
    fn test_add() {
        let a = U256::from(5);
        let b = U256::from(45);

        let res = a.add(b);
        println!("{:?}", res);
    }

    #[test]
    fn test_exp() {
        let a = U256::from(5);
        let exp = U256::from(3);

        let result = a.pow(exp);
        println!("{:?}", result);
    }
}