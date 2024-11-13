use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use ethers::types::{Bytes};
use primitive_types::{U256, H160, H256};
use serde::{Deserialize, Serialize};
use crate::error::exit::*;

#[derive(Debug, Clone)]
pub struct Block {
    pub blockhash: H256,
    pub coinbase: H160,
    pub timestamp: usize,
    pub number: usize,
    pub prevrandao: H256,
    pub gas_limit: U256,
    pub chainid: usize,
    pub basefee: usize,
}

#[derive(Debug, Clone)]
pub struct Call {
    // 当前call操作直接发起地址
    pub from: H160,
    // call操作的接收者
    pub to: Option<H160>,
    // call操作的实际发起地址，如果call操作不是delegatecall且不是在delegatecall执行上下文中的call操作，则caller与from相同
    pub caller: H160,
    // call操作执行环境的地址，如果call操作不是delegatecall，则address为to地址
    pub address: Option<H160>,
    pub value: U256,
    pub call_data: Bytes,
    pub call_type: CallType,
    pub call_depth: usize,
    pub pc: usize,
    pub world_state: WorldState,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let from = self.from;
        let to = self.to;
        let caller = self.caller;
        let address = self.address;
        write!(f, "\nThe current call operation originates from 'from':{}\n, targets the address 'to':{:?}\n, is initiated by 'caller':{}\n, and is executed in the 'address':{:?}\n", from, to, caller, address)?;

        let call_value = self.value;
        let selector= &self.call_data.clone().to_vec()[0..4];
        let hex_string:Vec<String> = selector.chunks(4).map(|chunk|{
            let hex_chunk: String = chunk.iter().map(|byte| format!("{:02x}", byte)).collect();
            format!("0x{}", hex_chunk)
        }).collect();
        write!(f, "this call operation call function:{:?}, and callvalue is {}", hex_string, call_value)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum CallType {
    /// None
    #[default]
    #[serde(rename = "none")]
    None,
    /// Call
    #[serde(rename = "call")]
    Call,
    /// Call code
    #[serde(rename = "callcode")]
    CallCode,
    /// Delegate call
    #[serde(rename = "delegatecall")]
    DelegateCall,
    /// Static call
    #[serde(rename = "staticcall")]
    StaticCall,
    /// Create
    #[serde(rename = "create")]
    Create,
    /// Create2
    #[serde(rename = "create2")]
    Create2,
}

#[derive(Debug, Clone)]
pub struct AccountState {
    pub nonce: usize,
    pub balance: U256,
    pub code_hash: Option<H256>,
    pub storage: Option<BTreeMap<H256,H256>>,
    pub(crate) code: Option<Bytes>
}

impl AccountState {
    pub fn new_contract(nonce: usize, balance: U256, code_hash: H256, storage: BTreeMap<H256,H256>, code: Bytes) -> Self {
        Self{
            nonce,
            balance,
            code_hash: Some(code_hash),
            storage: Some(storage),
            code: Some(code),
        }
    }

    pub fn new_eoa(nonce: usize, balance: U256) -> Self {
        Self{
            nonce,
            balance,
            code_hash: None,
            storage: None,
            code: None,
        }
    }

    pub fn default() -> Self {
        Self {
            nonce: 0,
            balance: U256::zero(),
            code_hash: Some(H256::default()),
            storage: Some(Default::default()),
            code: Some(Bytes::new())
        }
    }
}
impl fmt::Display for AccountState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ \n  \"nonce\": {},\n  \"balance\": {},\n  \"code_hash\": {},\n  \"storage\":",
            self.nonce, self.balance, self.code_hash.clone().unwrap_or_else(|| H256::default())
        )?;
        let account_storage = self.storage.clone().unwrap_or_else(|| BTreeMap::new());
        for (k,v) in account_storage {
            write!(f, "{}: {}", k, v)?;
        }
        let account_code = self.code.clone().unwrap_or_else(|| Bytes::new());
        write!(
            f,
            "\n  \"code\": {:?} \n}}", account_code
        )
    }
}

#[derive(Debug, Clone)]
pub struct WorldState {
    pub state: HashMap<H160, AccountState>,
}

impl WorldState {
    pub fn new(state: HashMap<H160,AccountState>) -> Self {
        Self{
            state
        }
    }

    pub fn default() -> Self {
        Self{
            state: Default::default(),
        }
    }

    // balance、code_hash、nonce、storage
    pub fn get_nonce(&self, address: H160) -> Result<usize, Box<dyn ExitError>> {
            match self.state.get(&address) {
                None => {
                    Err(Box::new(EVMError::AddressNotFound(address)))
                }
                Some(accountState) => {
                    let nonce = accountState.nonce;
                    Ok(nonce)
                }
            }
    }
    pub fn get_balance(&self, address: H160) -> Result<U256, Box<dyn ExitError>> {
        match self.state.get(&address){
            None => { Err(Box::new(EVMError::AddressNotFound(address))) }
            Some(accountState) => {
                let balance = accountState.balance;
                Ok(balance)
            }
        }
    }
    pub fn get_code_hash(&self, address: H160) -> Result<H256, Box<dyn ExitError>> {
        match self.state.get(&address){
            None => { Err(Box::new(EVMError::AddressNotFound(address))) }
            Some(accountState) => {
                if accountState.code_hash.is_some(){
                    Ok(accountState.code_hash.clone().unwrap())
                } else {
                    Err(Box::new(EVMError::NoContract(address)))
                }
            }
        }
    }

    pub fn get_code(&self, address: H160) -> Result<Bytes, Box<dyn ExitError>> {
        match self.state.get(&address) {
            None => { Err(Box::new(EVMError::AddressNotFound(address))) }
            Some(accountState) => {
                match &accountState.code {
                    None => {
                        Err(Box::new(EVMError::NoContract(address)))
                    }
                    Some(code) => {
                        Ok(code.clone())
                    }
                }
            }
        }
    }
    pub fn get_storage(&self, address: H160) -> Result<BTreeMap<H256, H256>, Box<dyn ExitError>> {
        match self.state.get(&address){
            None => { Err(Box::new(EVMError::AddressNotFound(address))) }
            Some(accountState) => {
                if accountState.storage.is_some(){
                    let storage = accountState.storage.clone().unwrap();
                    Ok(storage)
                } else {
                    Err(Box::new(EVMError::NoContract(address)))
                }
            }
        }
    }

    /// 全局状态中可以修改账户状态的操作
    pub fn insert_storage_value(&mut self, address: H160, key:H256, value: H256) -> Result<(), Box<dyn ExitError>> {
        let state = self.state.get_mut(&address);
        match state {
            // 如果账户状态存在
            Some(accountState) => match accountState.storage.as_mut() {
                // 如果该账户的storage存在
                Some(storage) => {
                    storage.insert(key, value);
                    Ok(())
                }
                // storage不存在意味着不是合约
                None => { Err(Box::new(EVMError::NoContract(address))) }
            }
            None => {
                // 没有该地址则创建新的账户状态
                let _ = &self.state.insert(
                    address,
                    AccountState {
                        balance: U256::zero(),
                        code_hash: Some(H256::default()),
                        nonce: 0,
                        storage: Some(BTreeMap::new()),
                        code: Some(Bytes::new()),
                    },
                );
                self.state
                    .get_mut(&address)
                    .unwrap()
                    .storage
                    .as_mut()
                    .unwrap()
                    .insert(key, value);
                Ok(())
            }
        }
    }

    pub fn get_storage_value(&self, address: H160, key: H256) -> Result<H256, Box<dyn ExitError>> {
        let state = self.state.get(&address);
        match state {
            Some(accountState) => match accountState.storage.as_ref() {
                Some(storage) => {
                    let storage_value = storage.get(&key).unwrap();
                    Ok(*storage_value)
                }
                // storage不存在意味着不是合约
                None => { Err(Box::new(EVMError::NoContract(address))) }
            }
            // 获取一个state不存在地址上的storage_value
            None => { Err(Box::new(EVMError::AddressNotFound(address))) }
        }
    }

    pub fn set_balance(&mut self, address: H160, value: U256) -> Result<(), Box<dyn ExitError>> {
        let state = self.state.get_mut(&address);
        match state {
            Some(accountState) => {
                accountState.balance = value;
                Ok(())
            }
            None => { Err(Box::new(EVMError::AddressNotFound(address))) }
        }
    }

    pub fn add_balance(&mut self, address: H160, value: U256) {
        let state = self.state.get_mut(&address);
        match state {
            Some(accountState) => {
                accountState.balance += value;
            }
            None => {}
        }
    }

    pub fn sub_balance(&mut self, address: H160, value: U256) {
        let state = self.state.get_mut(&address);
        match state {
            Some(accountState) => {
                accountState.balance -= value;
            }
            None => {}
        }
    }

    pub fn account_is_exsit(&self, address: H160) -> bool {
        self.state.get(&address).is_some()
    }

    pub fn default_sender(&mut self) -> H160 {
        let addr = H160::random();
        self.state.insert(
            addr,
            AccountState {
                balance: U256::zero(),
                code_hash: Some(H256::default()),
                nonce: 0,
                storage: Some(BTreeMap::new()),
                code: Some(Bytes::new()),
            },
        );
        addr
    }
    pub fn insert_code(&mut self, address: H160, code: Bytes) {
        let account = self.state.get_mut(&address).unwrap();
        account.code = Some(code);
    }

    pub fn insert_codehash(&mut self, address: H160, code_hash: H256) {
        let account = self.state.get_mut(&address).unwrap();
        account.code_hash = Some(code_hash);
    }
    pub fn new_account(&mut self, address: H160, account: AccountState) {
        self.state.insert(address, account);
    }
    pub fn remove_account(&mut self, address: H160) {
        self.state.remove(&address);
    }
}

// 实现 Display trait for WorldState
impl fmt::Display for WorldState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n  \"state\": {{\n")?;
        for (address, account_state) in &self.state {
            writeln!(
                f,
                "{}: {},",
                address,
                account_state
            )?;
        }
        write!(f, "}}\n}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_account_state() {
        let account = AccountState::default();
        println!("{}", account);
    }

    #[test]
    fn test_world_state() {
        let account = AccountState::default();
        let mut world_state = WorldState::default();
        let user = H160::random();
        world_state.new_account(user, account);
        println!("{}", world_state);
    }
}