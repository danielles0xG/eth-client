/*
#[derive(Clone, Debug)]
pub struct Provider<P> {
    inner: P,
    ens: Option<Address>,
    interval: Option<Duration>,
    from: Option<Address>,
    node_client: Arc<Mutex<Option<NodeClient>>>,
}
*/
use ethers::{
    prelude::*,
    providers::{Http, Provider},
    types::Address,
};
use std::sync::Arc;
use std::str::FromStr;
use std::env;

// const RPC_URL: &str = env::var("HTTP_ETHEREUM").unwrap();

abigen!(
    IUniswapV2Pair,
    r#"[function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)]"#
);

pub async fn get_uni_reserves() -> eyre::Result<()>{
    let rpc_url = "https://eth.llamarpc.com";
    let llama_provider = Arc::new(Provider::try_from(rpc_url)?);

    // Initialize a new instance of the Weth/Dai Uniswap V2 pair contract
    let pair_address: Address = "0xA478c2975Ab1Ea89e8196811F51A7B7Ade33eB11".parse()?;
    let uniswap_v2_pair = IUniswapV2Pair::new(pair_address, llama_provider);

    // Use the get_reserves() function to fetch the pool reserves
    let (reserve_0, reserve_1, block_timestamp_last) = uniswap_v2_pair.get_reserves().call().await?;

    println!(" pair_address,reserve_0,reserve_1 {:?} {:?} {:?} {:?}",
            pair_address,
            reserve_0,
            reserve_1,
            block_timestamp_last
        );
    Ok(())
}

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
