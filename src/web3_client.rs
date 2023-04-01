use std::env;
use std::str::FromStr;

use web3::types::{H160, U256};


pub async fn web3_client() -> web3::Result<()> {
        dotenv::dotenv().ok();
    
        let websocket = web3::transports::WebSocket::new(&env::var("WS_ETHEREUM").unwrap()).await?;
        let web3s = web3::Web3::new(websocket);
    
        let mut accounts = web3s.eth().accounts().await?;
        accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
        println!("Accounts: {:?}", accounts);
    
        for account in accounts {
            let balance = web3s.eth().balance(account, None).await?;
            println!(
                "Eth balance of {:?}: {}",
                account,
                balance
            );
        }
        
        Ok(())
    }
