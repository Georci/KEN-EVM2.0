use std::str::FromStr;
use primitive_types::{H160, H256, U256};
use revm_primitives::Address;
use crate::error::exit::*;
use crate::evm::EVM;
use crate::opcode::opcode::Opcode;
/// Convert [U256] into [H256].
#[must_use]
pub fn u256_to_h256(v: U256) -> H256 {
    let mut r = H256::default();
    v.to_big_endian(&mut r[..]);
    r
}

/// Convert [H256] to [U256].
#[must_use]
pub fn h256_to_u256(v: H256) -> U256 {
    U256::from_big_endian(&v[..])
}

/// Convert [H160] to [U256].
pub fn h160_to_u256(v: H160) -> U256 {
    U256::from_big_endian(&v[..])
}

/// Convert [U256] to [H160].
pub fn u256_to_h160(u: U256) -> H160 {
    // 将 U256 转换为 32 字节的大端数组
    let mut bytes = [0u8; 32];
    u.to_big_endian(&mut bytes);

    // 提取低 20 字节
    H160::from_slice(&bytes[12..32])
}

/// Convert [U256] to [usize].
pub fn u256_to_usize(v: U256) -> Result<usize, EVMError> {
    if v > U256::from(usize::MAX) {
        return Err(EVMError::InvalidRange);
    }
    Ok(v.as_usize())
}



pub fn pad_left(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    let len = bytes.len();
    if len <= 32 {
        padded[32 - len..].copy_from_slice(bytes);
    }
    padded
}

pub fn pad_right(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    let len = bytes.len();
    if len <= 32 {
        padded[..len].copy_from_slice(bytes);
    }
    padded
}

pub fn vec_to_string(vec: Vec<u8>) -> String {
    String::from_utf8_lossy(vec.as_ref()).parse().unwrap()
}

pub fn vec_to_u256(vec: Vec<u8>) -> U256 {
    U256::from_big_endian(vec.as_ref())
}

pub fn u256_to_vec(u: U256) -> Vec<u8> {
    let mut buf = [0u8; 32]; // 创建一个32字节的缓冲区
    u.to_big_endian(&mut buf); // 将 U256 转换为大端序字节数组
    buf.to_vec() // 将字节数组转换为 Vec<u8> 并返回
}

pub fn address_to_h160(address: Address) -> H160 {
    let address = H160::from_slice(address.as_ref());
    address
}

#[test]
fn test_address_to_h160() {
    let address_str = "0xbCDF0E814b7c65B238E2815289aCc05D3B933624";
    let address:Address = Address::from_str(address_str).unwrap();
    let contract = address_to_h160(address);
    println!("{:?}", contract);
}

pub fn get1() -> u8{
    1
}

fn get2() -> u8{
    2
}


pub fn map_op(op: u8) -> Option<Opcode> {
    match op {
        0x00 => Some(Opcode::STOP),
        0x01 => Some(Opcode::ADD),
        0x02 => Some(Opcode::MUL),
        0x03 => Some(Opcode::SUB),
        0x04 => Some(Opcode::DIV),
        0x05 => Some(Opcode::SDIV),
        0x06 => Some(Opcode::MOD),
        0x07 => Some(Opcode::SMOD),
        0x08 => Some(Opcode::ADDMOD),
        0x09 => Some(Opcode::MULMOD),
        0x0a => Some(Opcode::EXP),
        0x0b => Some(Opcode::SIGNEXTEND),
        0x10 => Some(Opcode::LT),
        0x11 => Some(Opcode::GT),
        0x12 => Some(Opcode::SLT),
        0x13 => Some(Opcode::SGT),
        0x14 => Some(Opcode::EQ),
        0x15 => Some(Opcode::ISZERO),
        0x16 => Some(Opcode::AND),
        0x17 => Some(Opcode::OR),
        0x18 => Some(Opcode::XOR),
        0x19 => Some(Opcode::NOT),
        0x1a => Some(Opcode::BYTE),
        0x1b => Some(Opcode::SHL),
        0x1c => Some(Opcode::SHR),
        0x1d => Some(Opcode::SAR),

        0x20 => Some(Opcode::KECCAK256),

        0x30 => Some(Opcode::ADDRESS),
        0x31 => Some(Opcode::BALANCE),
        0x32 => Some(Opcode::ORIGIN),
        0x33 => Some(Opcode::CALLER),
        0x34 => Some(Opcode::CALLVALUE),
        0x35 => Some(Opcode::CALLDATALOAD),
        0x36 => Some(Opcode::CALLDATASIZE),
        0x37 => Some(Opcode::CALLDATACOPY),
        0x38 => Some(Opcode::CODESIZE),
        0x39 => Some(Opcode::CODECOPY),
        0x3a => Some(Opcode::GASPRICE),
        0x3b => Some(Opcode::EXTCODESIZE),
        0x3c => Some(Opcode::EXTCODECOPY),
        0x3d => Some(Opcode::RETURNDATASIZE),
        0x3e => Some(Opcode::RETURNDATACOPY),
        0x3f => Some(Opcode::EXTCODEHASH),
        0x40 => Some(Opcode::BLOCKHASH),
        0x41 => Some(Opcode::COINBASE),
        0x42 => Some(Opcode::TIMESTAMP),
        0x43 => Some(Opcode::NUMBER),
        0x44 => Some(Opcode::DIFFICULTY),
        0x45 => Some(Opcode::GASLIMIT),
        0x46 => Some(Opcode::CHAINID),
        0x47 => Some(Opcode::SELFBALANCE),
        0x48 => Some(Opcode::BASEFEE),

        0x50 => Some(Opcode::POP),
        0x51 => Some(Opcode::MLOAD),
        0x52 => Some(Opcode::MSTORE),
        0x53 => Some(Opcode::MSTORE8),
        0x54 => Some(Opcode::SLOAD),
        0x55 => Some(Opcode::SSTORE),
        0x56 => Some(Opcode::JUMP),
        0x57 => Some(Opcode::JUMPI),
        0x58 => Some(Opcode::PC),
        0x59 => Some(Opcode::MSIZE),
        0x5a => Some(Opcode::GAS),
        0x5b => Some(Opcode::JUMPDEST),
        0x5e => Some(Opcode::MCOPY),
        0x5f => Some(Opcode::PUSH0),
        0x60 => Some(Opcode::PUSH1),
        0x61 => Some(Opcode::PUSH2),
        0x62 => Some(Opcode::PUSH3),
        0x63 => Some(Opcode::PUSH4),
        0x64 => Some(Opcode::PUSH5),
        0x65 => Some(Opcode::PUSH6),
        0x66 => Some(Opcode::PUSH7),
        0x67 => Some(Opcode::PUSH8),
        0x68 => Some(Opcode::PUSH9),
        0x69 => Some(Opcode::PUSH10),
        0x6a => Some(Opcode::PUSH11),
        0x6b => Some(Opcode::PUSH12),
        0x6c => Some(Opcode::PUSH13),
        0x6d => Some(Opcode::PUSH14),
        0x6e => Some(Opcode::PUSH15),
        0x6f => Some(Opcode::PUSH16),
        0x70 => Some(Opcode::PUSH17),
        0x71 => Some(Opcode::PUSH18),
        0x72 => Some(Opcode::PUSH19),
        0x73 => Some(Opcode::PUSH20),
        0x74 => Some(Opcode::PUSH21),
        0x75 => Some(Opcode::PUSH22),
        0x76 => Some(Opcode::PUSH23),
        0x77 => Some(Opcode::PUSH24),
        0x78 => Some(Opcode::PUSH25),
        0x79 => Some(Opcode::PUSH26),
        0x7a => Some(Opcode::PUSH27),
        0x7b => Some(Opcode::PUSH28),
        0x7c => Some(Opcode::PUSH29),
        0x7d => Some(Opcode::PUSH30),
        0x7e => Some(Opcode::PUSH31),
        0x7f => Some(Opcode::PUSH32),
        0x80 => Some(Opcode::DUP1),
        0x81 => Some(Opcode::DUP2),
        0x82 => Some(Opcode::DUP3),
        0x83 => Some(Opcode::DUP4),
        0x84 => Some(Opcode::DUP5),
        0x85 => Some(Opcode::DUP6),
        0x86 => Some(Opcode::DUP7),
        0x87 => Some(Opcode::DUP8),
        0x88 => Some(Opcode::DUP9),
        0x89 => Some(Opcode::DUP10),
        0x8a => Some(Opcode::DUP11),
        0x8b => Some(Opcode::DUP12),
        0x8c => Some(Opcode::DUP13),
        0x8d => Some(Opcode::DUP14),
        0x8e => Some(Opcode::DUP15),
        0x8f => Some(Opcode::DUP16),
        0x90 => Some(Opcode::SWAP1),
        0x91 => Some(Opcode::SWAP2),
        0x92 => Some(Opcode::SWAP3),
        0x93 => Some(Opcode::SWAP4),
        0x94 => Some(Opcode::SWAP5),
        0x95 => Some(Opcode::SWAP6),
        0x96 => Some(Opcode::SWAP7),
        0x97 => Some(Opcode::SWAP8),
        0x98 => Some(Opcode::SWAP9),
        0x99 => Some(Opcode::SWAP10),
        0x9a => Some(Opcode::SWAP11),
        0x9b => Some(Opcode::SWAP12),
        0x9c => Some(Opcode::SWAP13),
        0x9d => Some(Opcode::SWAP14),
        0x9e => Some(Opcode::SWAP15),
        0x9f => Some(Opcode::SWAP16),
        0xa0 => Some(Opcode::LOG0),
        0xa1 => Some(Opcode::LOG1),
        0xa2 => Some(Opcode::LOG2),
        0xa3 => Some(Opcode::LOG3),
        0xa4 => Some(Opcode::LOG4),

        0xf0 => Some(Opcode::CREATE),
        0xf1 => Some(Opcode::CALL),
        0xf2 => Some(Opcode::CALLCODE),
        0xf3 => Some(Opcode::RETURN),
        0xf4 => Some(Opcode::DELEGATECALL),
        0xf5 => Some(Opcode::CREATE2),

        0xfa => Some(Opcode::STATICCALL),

        0xfd => Some(Opcode::REVERT),
        0xfe => Some(Opcode::INVALID),
        0xff => Some(Opcode::SELFDESTRUCT),
        _ => None,
    }
}

pub fn increment_nonce(evm :&mut EVM, address: H160) -> Result<(), Box<dyn ExitError>> {
    let account_state = evm.world_state.state.get_mut(&address);
    let nonce = match account_state {
        Some(nonce) => nonce,
        None => return Err(Box::new(EVMError::AddressNotFound(address)))
    };
    nonce.nonce += 1;
    Ok(())
}