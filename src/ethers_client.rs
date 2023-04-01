// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.
use ethers::prelude::*;
use std::str::FromStr;
use std::env;

// const RPC_URL: &str = env::var("HTTP_ETHEREUM").unwrap();

pub async  fn ethers_client() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let provider = Provider::<Http>::try_from(&env::var("HTTP_ETHEREUM").unwrap())?;
    let block_number: U64 = provider.get_block_number().await?;
    println!(
        "Block number {:?}",
        block_number
    );
    Ok(())
}
