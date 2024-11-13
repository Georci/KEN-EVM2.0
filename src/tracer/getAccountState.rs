use primitive_types::{H160, H256, U256};
use std::sync::Arc;
use ethers::providers::Middleware;
use std::collections::{BTreeMap, HashMap};
use std::process;
use std::str::FromStr;
use ethers::types::{Address, Bytes};
use crate::tracer::getTransaction::StateTracerType;
use ethers::prelude::{AccountState, GethDebugBuiltInTracerConfig, GethDebugTracerConfig, GethDebugTracerType, GethDebugTracingOptions, Http, PreStateConfig, PreStateFrame, Provider, ProviderExt};
use ethers::prelude::GethDebugBuiltInTracerType::PreStateTracer;
use revm_primitives::{keccak256, B256};

#[derive(Debug, Clone)]
pub struct AccountStateEx {
    /// The account's nonce, which is incremented each time a transaction is sent from the account.
    pub nonce: usize,

    /// The account's balance, represented as a 32-byte array.
    pub balance: U256,

    /// The account's storage, represented as a hashmap where the keys and values are both 32-byte arrays.
    pub storage: Option<BTreeMap<H256, H256>>,

    /// The hash of the account's code, represented as a 32-byte array.
    pub code_hash: Option<B256>,
    pub code: Option<Bytes>,
    pub state_tracer_type: StateTracerType,
}
#[derive(Debug, Clone)]
pub struct ISDiff {
    /// is diff
    pub is_diff: bool,
    /// turn on diff and judge is_pre
    pub is_pre: Option<bool>
}

impl ISDiff {
    pub fn default() -> Self {
        Self{
            is_diff: false,
            is_pre: None,
        }
    }

    pub fn new(is_diff: bool, is_pre: Option<bool>) -> Self {
        Self {
            is_diff,
            is_pre
        }
    }
}

pub async fn get_accounts_state_tx(
    provider: Arc<Provider<Http>>,
    tx_hash: H256,
    is_diff: ISDiff,
) -> BTreeMap<H160, AccountStateEx> {
    let tracer_config = GethDebugTracerConfig::BuiltInTracer(
        GethDebugBuiltInTracerConfig::PreStateTracer(PreStateConfig {
            diff_mode: Some(is_diff.is_diff),
        }),
    );

    let mut options = GethDebugTracingOptions::default();
    options.tracer = Some(GethDebugTracerType::BuiltInTracer(PreStateTracer));
    options.tracer_config = Some(tracer_config);

    let tracer_info = provider
        .debug_trace_transaction(tx_hash, options)
        .await
        .unwrap_or_else(|err| {
            eprintln!("transaction reverted with err: {}", err);
            process::exit(1);
        });
    // println!("PreStatetracer difference：{:?}",tracer_info);
    let mut tx_account_state_ex: BTreeMap<Address, AccountStateEx> = BTreeMap::new();

    match tracer_info {
        ethers::types::GethTrace::Known(ref geth_tracer_frame) => match geth_tracer_frame {
            ethers::types::GethTraceFrame::PreStateTracer(pre_state_frame ) => match pre_state_frame {
                PreStateFrame::Default(default_mode ) => {
                    let true_off_pre_state = &default_mode.0;
                    tx_account_state_ex = insert_tx_account_state_ex(tx_account_state_ex, true_off_pre_state, is_diff);
                }
                PreStateFrame::Diff(diff_on) => {
                    if is_diff.is_pre.unwrap() == true {
                        let turn_on_diff_pre_state = &diff_on.pre;
                        println!("This is diff_on.pre:{:?}",turn_on_diff_pre_state);
                        tx_account_state_ex = insert_tx_account_state_ex(tx_account_state_ex, turn_on_diff_pre_state, is_diff);
                    } else {
                        let turn_on_diff_pre_state = &diff_on.post;
                        println!("This is diff_on.post:{:?}",turn_on_diff_pre_state);
                        tx_account_state_ex = insert_tx_account_state_ex(tx_account_state_ex, turn_on_diff_pre_state, is_diff);
                    }
                }
            },
            _ => todo!(),
        },
        _ => todo!(),
    };
    tx_account_state_ex
}


// 将完成的新的账户状态进行插入
pub fn insert_tx_account_state_ex(
    mut tx_account_state_ex: BTreeMap<Address, AccountStateEx>,
    new_tx_account_state_ex: &BTreeMap<Address, AccountState>,
    isdiff: ISDiff,
) -> BTreeMap<Address, AccountStateEx> {
    new_tx_account_state_ex
        .iter()
        .for_each(|(_addr, _account_state)| {
            let balance = _account_state.balance.unwrap();

            let code: Option<String> = _account_state.clone().code;
            println!("_account_state.code is {:?}", _account_state.clone().code);
            // 使用 `map` 将 `Option<String>` 转换为 `Option<Bytes>`
            let code_bytes: Option<Bytes> = if code.is_some(){
                Some(Bytes::from_str(code.unwrap().as_str()).unwrap())
            } else { None };

            let code_hash = if let Some(inner_code) = code_bytes.clone() {
                Some(keccak256(&inner_code))
            } else {
                None
            };
            let storage = _account_state.storage.clone();
            let mut my_storage: BTreeMap<H256, H256> = BTreeMap::new();
            if let Some(inner_storage) = storage.clone() {
                inner_storage.iter().for_each(|(slot, value)| {
                    my_storage.insert(*slot, *value);
                });
            };
            let nonce = if let Some(inner_nonce) = _account_state.nonce {
                inner_nonce.as_usize()
            } else {
                0
            };

            let state_tracer_type = if isdiff.is_diff == false {
                StateTracerType::TurnOffDiff
            } else {
                let state_tracer_type = if isdiff.is_pre.unwrap() == true {
                    StateTracerType::TurnOnDiffPre
                } else {
                    StateTracerType::TurnOnDiffPost
                };
                state_tracer_type
            };

            let mut account_state = AccountStateEx {
                nonce,
                balance,
                storage: Some(my_storage),
                code_hash,
                code: code_bytes,
                state_tracer_type,
            };
            tx_account_state_ex.insert(*_addr, account_state.clone());
        });
    tx_account_state_ex
}




#[tokio::test]
 pub async fn test_get_accounts_state_tx() {
    let provider_http_url = String::from("https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b");
    let provider = Provider::try_connect(provider_http_url.as_str()).await.expect("could not connect");

    let attack_hash = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";

    let account_state =
        get_accounts_state_tx(Arc::from(provider), H256::from_str(attack_hash).unwrap(), ISDiff::default()).await;
    println!("{:?}", account_state);
}
