use std::str::FromStr;
mod web3_client;
mod ethers_client;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // web3_client::web3_client().await;
    ethers_client::get_uni_reserves().await;
    ethers_client::ethers_client().await;
    Ok(())
}
