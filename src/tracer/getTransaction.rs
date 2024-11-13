use primitive_types::{H160, U256, H256};
use ethers::prelude::{Http, Provider, TxHash};
use ethers::providers::{Middleware, ProviderError};
use ethers::types::{Bytes};


#[derive(Debug, Clone)]
pub struct TransactionEnv {
    /// The transaction's hash
    pub tx_hash: H256,

    /// The transaction's nonce - 实际上就是当前交易sender的nonce
    pub nonce: usize,

    /// Block hash. None when pending.
    pub block_hash: H256,

    pub block_number: usize,

    pub coinbase: H160,

    pub timestamp: usize,

    pub from: H160,

    pub to: H160,

    /// Transferred value
    pub value: U256,

    pub gas_price: Option<U256>,

    /// Gas amount
    pub gas: U256,

    /// Input data
    pub calldata: Bytes,

    pub basefee: Option<U256>,

    pub difficulty: U256,

    pub prevrandao: Option<U256>,

    pub chain_id: Option<U256>,
}

#[derive(Debug, Clone)]
pub enum StateTracerType {
    None,
    TurnOffDiff,
    TurnOnDiffPre,
    TurnOnDiffPost,
    TXAfterState
}


pub async fn get_transaction_content(
    provider: Provider<Http>,
    tx_hash: TxHash,
) -> Result<TransactionEnv, ProviderError> {
    let transaction = provider
        .get_transaction(tx_hash)
        .await
        .expect("get transaction hash error");

    let calldata = transaction.clone().unwrap().input.to_vec();

    let block_number = transaction.clone().unwrap().block_number.unwrap();
    let block_info = provider
        .get_block(block_number)
        .await
        .expect("get block error");

    // get transaction nonce
    let mut nonce = transaction.clone().unwrap().nonce;


    // get transaction to [u8; 32]
    let to = if let Some(to) = transaction.clone().unwrap().to {
        to.0
    } else {
        [0u8; 20]
    };

    // get transaction gas_price
    let mut gas_price = [0u8; 32];
    transaction
        .clone()
        .unwrap()
        .gas_price
        .unwrap()
        .to_big_endian(&mut gas_price);

    // get transaction gas
    let mut gas = [0u8; 32];
    transaction.clone().unwrap().gas.to_big_endian(&mut gas);

    // get transaction basefee
    let mut basefee = [0u8; 32];
    transaction
        .clone()
        .unwrap()
        .max_fee_per_gas
        .unwrap()
        .to_big_endian(&mut basefee);

    // get transaction difficulty
    let mut difficulty = [0u8; 32];
    block_info
        .clone()
        .unwrap()
        .difficulty
        .to_big_endian(&mut difficulty);

    // get transaction timestamp
    let mut timestamp = [0u8; 32];
    block_info
        .clone()
        .unwrap()
        .timestamp
        .to_big_endian(&mut timestamp);

    Ok(TransactionEnv {
        tx_hash: transaction.clone().unwrap().hash,
        nonce: nonce.as_usize(),
        block_hash: transaction.clone().unwrap().block_hash.unwrap(),
        block_number: block_number.as_usize(),
        coinbase: block_info.clone().unwrap().author.unwrap(),
        timestamp: block_info.clone().unwrap().timestamp.as_usize(),
        from: transaction.clone().unwrap().from,
        to: H160::from(to),
        value: transaction.clone().unwrap().value,
        gas_price: transaction.clone().unwrap().gas_price,
        gas: transaction.clone().unwrap().gas,
        calldata: Bytes::from(calldata),
        basefee: transaction.clone().unwrap().max_fee_per_gas,
        difficulty: block_info.clone().unwrap().difficulty,
        prevrandao: None,
        chain_id: transaction.clone().unwrap().chain_id,
    })
}
