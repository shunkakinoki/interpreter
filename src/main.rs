use anyhow::{Ok, Result};
#[allow(unused_imports)]
use bytes::Bytes;
#[allow(unused_imports)]
use ethers::{
    abi::parse_abi,
    prelude::BaseContract,
    providers::{Http, Provider},
};
#[allow(unused_imports)]
use revm::{
    db::{CacheDB, EmptyDB, EthersDB},
    Database, TransactOut, TransactTo, B160, EVM, U256 as rU256,
};
use std::{str::FromStr, sync::Arc};

fn main() -> Result<()> {
    println!("Hello, world!");

    let client = Provider::<Http>::try_from("https://mainnet.infura.io/v3/")?;
    let client = Arc::new(client);

    Ok(())
}
