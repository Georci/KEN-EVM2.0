use std::collections::{BTreeMap, HashMap};
use primitive_types::{H160, U256};
use ken_evm::evm::EVM;
use ken_evm::{deploy, external_call, Memory, Stack};
use ken_evm::globalState::*;

fn main(){
    // 构建调用者状态
    let caller:H160 = "0xbCDF0E814b7c65B238E2815289aCc05D3B933624".parse().unwrap();
    let caller_state:AccountState = AccountState::new_eoa(
        0,
        U256::from(10),
    );
    let mut hash_map = HashMap::new();
    hash_map.insert(caller, caller_state);
    let world_state = WorldState::new(hash_map);

    // 构建EVM状态
    let mut handler = EVM::new(world_state.clone());
    let run_time = "0x6080604052348015600e575f80fd5b50607480601a5f395ff3fe6080604052348015600e575f80fd5b50600436106026575f3560e01c806311f37ceb14602a575b5f80fd5b600c60405190815260200160405180910390f3fea26469706673582212208f8107617c0706b60751cb6ed139c3edd4b43be3b02fcbc22b28192e202c027e64736f6c634300081a0033";
    let to = deploy(&mut handler, run_time.parse().unwrap(), caller, U256::zero());

    let return_value = handler.return_data.clone();
    println!("return_data is : {:?}", return_value);
    println!("sub_return_data is : {:?}", handler.sub_return_data);

    // 调用函数
    let call:Call = Call{
        from: caller,
        to:Some(to),
        caller,
        address: Some(to),
        value: Default::default(),
        call_data: "0x11f37ceb".parse().unwrap(),
        call_type: CallType::Call,
        call_depth: 0,
        pc: 0,
        world_state: world_state.clone(),
    };
    // 更新EVM信息
    let value = external_call(&mut handler, call).expect("external call failed!");

    println!("return_data is : {:?}", value);
}

// todo!:使用我们的evm执行真实的链上交易
