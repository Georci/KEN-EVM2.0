use std::collections::{BTreeMap, HashMap};
use std::{fmt, process};
use std::hash::Hash;
use std::string::FromUtf8Error;
use ethers::utils::keccak256 as ethers_keccak256;
use revm_primitives::Address;
use ethers::types::{Selector, Bytes, Transaction};
use primitive_types::{U256, H160, H256};
use crate::AccountState;
use crate::error::exit::*;
use crate::machine::Stack::Stack;
use crate::machine::Memory::Memory;
use crate::globalState::{WorldState, Block, CallType};
use crate::globalState::Call;
use crate::opcode::{flow::*, account::*, arithmatic::*, bitewise::*, comparison::*, enviroment::*, flow::*, structure::*};
use crate::opcode::opcode::Opcode;
use crate::utils::{address_to_h160, increment_nonce, map_op, u256_to_h160, vec_to_string, vec_to_u256};

#[derive(Debug, Clone)]
pub struct EVM {
    ///数据结构
    pub stack: Stack,
    pub memory: Memory,
    pub transient_storage: HashMap<H160, BTreeMap<H256, H256>>,

    ///出现外部调用时进行压栈
    pub call_stack: Vec<Call>,
    pub memory_stack: Vec<Memory>,
    pub evm_stack: Vec<Stack>,
    pub function_stack: Vec<(H160, Selector)>,

    ///全局信息
    pub call_depth: usize,
    pub pc: usize,
    pub is_constructor: bool,
    pub is_revert: bool,
    pub world_state: WorldState,

    ///交易发起者
    pub origin: H160,

    // 只要出现call，则下面的信息不断更新，这些都代表着一笔内部交易
    pub bytecode: Option<Bytes>,        // bytecode一定是to地址的code
    pub before_world_state: WorldState, // 万一revert
    pub return_data: Option<Vec<u8>>, // returndata实际上和memory很像，就类似于一个很长的u8数组
    pub sub_return_data: Option<Vec<u8>>,

    // 交易复现使用
    pub block: Option<Block>,
    pub transaction: Option<Transaction>,
}

impl EVM {
    pub fn new(world_state: WorldState) -> Self {
        Self {
            sub_return_data: None,
            origin: H160::zero(),
            transient_storage: HashMap::new(),
            is_revert: false,
            stack: Stack::new(1024),
            memory: Memory::new(1024),
            call_stack: Vec::new(),
            memory_stack: Vec::new(),
            evm_stack: Vec::new(),
            call_depth: 0,
            function_stack: Vec::<(H160, Selector)>::new(),
            pc: 0,
            bytecode: None,
            world_state,
            block: None,
            transaction: None,
            before_world_state: WorldState::default(),
            return_data: None,
            // 是否是部署合约交易
            is_constructor: false,
        }
    }


    //todo!: 这里缺一个判断
    pub fn interepter(&mut self) -> Result<(), Box<dyn ExitError>> {
        match self.bytecode.clone() {
            None => {}
            Some(code) => {
                while self.pc < code.len() {
                    // 如果当前字节码存在于opcode表，即从对应的opcode表获取其对应的操作码，否则默认为INVALID
                    let op = map_op(code[self.pc]).unwrap_or_else(|| Opcode::INVALID);
                    println!("current op is {}: {}",self.pc, op);
                    println!("stack is {}", self.stack);
                    println!("memory is {}", self.memory);
                    match self.interepter_op_code(op) {
                        Ok(_) => {}
                        Err(_) => {
                            self.pc += 1;
                        }
                    };
                }
                self.pc = 0;
            }
        }
        Ok(())

    }

    /// 描述：该函数用户创建合约
    /// 注意：调用该函数前需要构建好Call
    /// 该函数能以主动调用的形式触发，同时也由 is_constructor == true && to.is_none() 条件被动触发
    pub fn deploy_contract(&mut self, caller: H160, value: U256) -> Result<H160, Box<dyn ExitError>> {
        self.is_constructor = false;

        let creation_code:Bytes = match &self.bytecode {
            Some(ref bytecode) => bytecode.clone(),
            None => {
                return Err(Box::new(EVMError::DeployContractFailed))
            }
        };
        // 创建地址
        let nonce = match self.world_state.get_nonce(caller) {
            Ok(nonce) => nonce,
            Err(e) => return Err(Box::new(EVMError::DeployContractFailed)),
        };
        let create_address = Address::from_slice(caller.as_ref()).create(nonce as u64);
        let contract_address = address_to_h160(create_address);

        let account_state = AccountState::new_contract(0, value, H256::default(), Default::default(),  creation_code);
        self.world_state.new_account(contract_address, account_state.clone());

        let deploy_call = Call{
            from: Default::default(),
            to: None,
            caller,
            address: None,
            value,
            call_data: Default::default(),
            call_type: Default::default(),
            call_depth: 0,
            pc: 0,
            world_state: self.world_state.clone()
        };
        self.call_stack.push(deploy_call);

        let call_result = self.interepter();
        if call_result.is_err() {
            return Err(Box::new(EVMError::DeployContractFailed));
        };

        let runtime_code = Bytes::from(self.return_data.clone().unwrap());
        let code_hash:H256 = H256::from(ethers_keccak256(&runtime_code));
        self.world_state.insert_code(contract_address, runtime_code);
        println!("accountState is {:?}", self.world_state.state.get(&contract_address));
        self.world_state.insert_codehash(contract_address, code_hash);

        self.call_stack.pop();
        Ok(contract_address)
    }

    pub fn interepter_op_code(&mut self, op: Opcode) -> Result<(), Box<dyn ExitError>> {
        match op{
            Opcode::STOP => {stop(self)},
            Opcode::ADD => {add(self)},
            Opcode::MUL => {mul(self)},
            Opcode::SUB => {sub(self)},
            Opcode::DIV => {div(self)},
            Opcode::SDIV => {sdiv(self)},
            Opcode::MOD => {_mod(self)}
            Opcode::SMOD => {smod(self)}
            Opcode::ADDMOD => {addmod(self)}
            Opcode::MULMOD => {mulmod(self)}
            Opcode::EXP => {exp(self)}
            Opcode::SIGNEXTEND => {signextend(self)}
            Opcode::LT => {lt(self)}
            Opcode::GT => {gt(self)}
            Opcode::SLT => {slt(self)}
            Opcode::SGT => {sgt(self)}
            Opcode::EQ => {eq(self)}
            Opcode::ISZERO => {iszero(self)}
            Opcode::AND => {and(self)}
            Opcode::OR => {or(self)}
            Opcode::XOR => {xor(self)}
            Opcode::NOT => {not(self)}
            Opcode::BYTE => {byte(self)}
            Opcode::SHL => {shl(self)}
            Opcode::SHR => {shr(self)}
            Opcode::SAR => {sar(self)}
            Opcode::KECCAK256 => {keccak256(self)}
            Opcode::ADDRESS => {address(self)}
            Opcode::BALANCE => {balance(self)}
            Opcode::ORIGIN => {origin(self)}
            Opcode::CALLER => {caller(self)}
            Opcode::CALLVALUE => {callvalue(self)}
            Opcode::CALLDATALOAD => {calldataload(self)}
            Opcode::CALLDATASIZE => {calldatasize(self)}
            Opcode::CALLDATACOPY => {calldatacopy(self)}
            Opcode::CODESIZE => {codesize(self)}
            Opcode::CODECOPY => {codecopy(self)}
            Opcode::GASPRICE => {gasprice(self)}
            Opcode::EXTCODESIZE => {extcodesize(self)}
            Opcode::EXTCODECOPY => {extcodecopy(self)}
            Opcode::RETURNDATASIZE => {returndatasize(self)}
            Opcode::RETURNDATACOPY => {returndatacopy(self)}
            Opcode::EXTCODEHASH => {extcodehash(self)}
            Opcode::BLOCKHASH => {blockhash(self)}
            Opcode::COINBASE => {coinbase(self)}
            Opcode::TIMESTAMP => {timestamp(self)}
            Opcode::NUMBER => {number(self)}
            Opcode::DIFFICULTY => {prevrandao(self)}
            Opcode::GASLIMIT => {gaslimit(self)}
            Opcode::CHAINID => {chainid(self)}
            Opcode::SELFBALANCE => {selfbalance(self)}
            Opcode::BASEFEE => {basefee(self)}
            Opcode::POP => {pop(self)}
            Opcode::MLOAD => {mload(self)}
            Opcode::MSTORE => {msotre(self)}
            Opcode::MSTORE8 => {msotre8(self)}
            Opcode::SLOAD => {sload(self)}
            Opcode::SSTORE => {sstore(self)}
            Opcode::JUMP => {jump(self)}
            Opcode::JUMPI => {jumpi(self)}
            Opcode::PC => {pc(self)}
            Opcode::MSIZE => {msize(self)}
            Opcode::GAS => {gas(self)}
            Opcode::JUMPDEST => {jumpdest(self)}
            Opcode::MCOPY => {mcopy(self)}
            Opcode::PUSH0 => {push0(self)}
            Opcode::PUSH1 => {push1(self)}
            Opcode::PUSH2 => {push2(self)}
            Opcode::PUSH3 => {push3(self)}
            Opcode::PUSH4 => {push4(self)}
            Opcode::PUSH5 => {push5(self)}
            Opcode::PUSH6 => {push6(self)}
            Opcode::PUSH7 => {push7(self)}
            Opcode::PUSH8 => {push8(self)}
            Opcode::PUSH9 => {push9(self)}
            Opcode::PUSH10 => {push10(self)}
            Opcode::PUSH11 => {push11(self)}
            Opcode::PUSH12 => {push12(self)}
            Opcode::PUSH13 => {push13(self)}
            Opcode::PUSH14 => {push14(self)}
            Opcode::PUSH15 => {push15(self)}
            Opcode::PUSH16 => {push16(self)}
            Opcode::PUSH17 => {push17(self)}
            Opcode::PUSH18 => {push18(self)}
            Opcode::PUSH19 => {push19(self)}
            Opcode::PUSH20 => {push20(self)}
            Opcode::PUSH21 => {push21(self)}
            Opcode::PUSH22 => {push22(self)}
            Opcode::PUSH23 => {push23(self)}
            Opcode::PUSH24 => {push24(self)}
            Opcode::PUSH25 => {push25(self)}
            Opcode::PUSH26 => {push26(self)}
            Opcode::PUSH27 => {push27(self)}
            Opcode::PUSH28 => {push28(self)}
            Opcode::PUSH29 => {push29(self)}
            Opcode::PUSH30 => {push30(self)}
            Opcode::PUSH31 => {push31(self)}
            Opcode::PUSH32 => {push32(self)}
            Opcode::DUP1 => {dup1(self)}
            Opcode::DUP2 => {dup2(self)}
            Opcode::DUP3 => {dup3(self)}
            Opcode::DUP4 => {dup4(self)}
            Opcode::DUP5 => {dup5(self)}
            Opcode::DUP6 => {dup6(self)}
            Opcode::DUP7 => {dup7(self)}
            Opcode::DUP8 => {dup8(self)}
            Opcode::DUP9 => {dup9(self)}
            Opcode::DUP10 => {dup10(self)}
            Opcode::DUP11 => {dup11(self)}
            Opcode::DUP12 => {dup12(self)}
            Opcode::DUP13 => {dup13(self)}
            Opcode::DUP14 => {dup14(self)}
            Opcode::DUP15 => {dup15(self)}
            Opcode::DUP16 => {dup16(self)}
            Opcode::SWAP1 => {swap1(self)}
            Opcode::SWAP2 => {swap2(self)}
            Opcode::SWAP3 => {swap3(self)}
            Opcode::SWAP4 => {swap4(self)}
            Opcode::SWAP5 => {swap5(self)}
            Opcode::SWAP6 => {swap6(self)}
            Opcode::SWAP7 => {swap7(self)}
            Opcode::SWAP8 => {swap8(self)}
            Opcode::SWAP9 => {swap9(self)}
            Opcode::SWAP10 => {swap10(self)}
            Opcode::SWAP11 => {swap11(self)}
            Opcode::SWAP12 => {swap12(self)}
            Opcode::SWAP13 => {swap13(self)}
            Opcode::SWAP14 => {swap14(self)}
            Opcode::SWAP15 => {swap15(self)}
            Opcode::SWAP16 => {swap16(self)}
            Opcode::LOG0 => {log0(self)}
            Opcode::LOG1 => {log1(self)}
            Opcode::LOG2 => {log2(self)}
            Opcode::LOG3 => {log3(self)}
            Opcode::LOG4 => {log4(self)}
            Opcode::CREATE => {create(self)}
            Opcode::CALL => {call(self)}
            Opcode::CALLCODE => {callcode(self)}
            Opcode::RETURN => {_return(self)}
            Opcode::DELEGATECALL => {delegatecall(self)}
            Opcode::CREATE2 => {create2(self)}
            Opcode::STATICCALL => {staticcall(self)}
            Opcode::REVERT => {revert(self)}
            Opcode::INVALID => {invalid(self)}
            Opcode::SELFDESTRUCT => {selfdestruct(self)}
        }
    }

    // 该调用操作，区别于call类型字节码实现的操作 主要用于create操作码在创造合约之后调用该合约
    pub fn call(
        &mut self,
        to: H160,
        value: U256,
        gas:usize,
        call_type: CallType
    ) -> Result<(), Box<dyn ExitError>> {

        let before_code = self.bytecode.clone();
        //将当前call组成Call压入栈中
        let now_call = self.call_stack.last().unwrap();
        // TODO!:这里需要注意内层和外层创建合约的区别，如果是在当前调用的合约内通过create类字节码创建则需要一个新的Call，如果是在最外层直接create合约，应该是不需要Call
        let _call = Call {
            from: now_call.to.unwrap(),
            to: Some(to),
            value,
            // create执行调用时并不会携带calldata去执行合约函数
            call_data: Bytes::new(),
            caller: if call_type == CallType::DelegateCall{
                now_call.caller
            } else {
                now_call.address.unwrap()
            },
            // 这里的is_err操作难道是如果目标地址没有code，则重新回到当前地址执行？
            address: if call_type == CallType::DelegateCall{
                now_call.address
            } else {
                Some(to)
            },
            call_type,
            call_depth: self.call_depth,
            pc: self.pc,
            world_state: self.world_state.clone(),
        };
        self.evm_stack.push(self.stack.clone());
        self.memory_stack.push(self.memory.clone());
        self.call_stack.push(_call.clone());
        self.call_depth += 1;

        // 执行call操作
        match self.world_state.get_code(to) {
            Ok(code) => {
                self.bytecode = Some(code);
                let interpret_result = self.interepter();
                if interpret_result.is_err() {
                    return Err(interpret_result.err().unwrap());
                }
            },
            Err(e) => return Err(e)
        };

        self.bytecode = before_code;
        self.pc = _call.pc + 1;
        self.call_depth -= 1;
        self.stack = self.evm_stack.pop().unwrap();
        self.memory = self.memory_stack.pop().unwrap();
        self.call_stack.pop();

        // create进入创建合约的子调用之后会将创建合约的runtime_code作为return的数据存放在return_data中
        self.sub_return_data = match self.return_data.clone(){
            None => {None}
            Some(ret_data) => {
                Some(ret_data)
            }
        };
        let runtime_code:Bytes = match &self.sub_return_data{
            None => { Bytes::new() }
            Some(data) => {  Bytes::from(data.clone()) }
        };
        // 更新对应地址的code
        self.world_state.insert_code(to, runtime_code);

        self.return_data = None;
        if _call.call_type.eq(&CallType::StaticCall) {
            // 恢复上下文
            self.world_state = _call.world_state;
        }
        if self.is_revert {
            self.stack.push(U256::zero())?;
        } else {
            self.stack.push(U256::one())?;
        };
        increment_nonce(self, self.call_stack.last().unwrap().address.unwrap())?;
        Ok(())
    }
}

impl fmt::Display for EVM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stack = self.stack.clone();
        let memory = self.memory.clone();

        write!(f, "current EVM's stack is {}\n, memory is {}\n", stack, memory)?;
        write!(f, "call is {}\n", self.call_stack.last().unwrap())?;
        write!(f, "current EVM's world state is :{}\n", self.world_state)?;

        let return_data= self.return_data.clone();
        write!(f, "excution returndata is:{:?}\n", return_data)
    }
}