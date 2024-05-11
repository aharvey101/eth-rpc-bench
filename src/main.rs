use dotenv::dotenv;
use ethers::abi::{Abi, Address};
use ethers::prelude::Contract;
use ethers::providers::{Http, Middleware, Provider, ProviderError};
use ethers::types::{H160, U64};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;

fn get_env(key: &str) -> String {
    std::env::var(key).unwrap_or(String::from(""))
}
struct Env {
    alchemy_url: String,
    local_node_url: String,
}

impl Env {
    fn new() -> Self {
        Env {
            alchemy_url: get_env("ALCHEMY_URL"),
            local_node_url: get_env("LOCAL_NODE_URL"),
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), ProviderError> {
    let uniswap_v3_pool_abi: Abi = serde_json::from_str(
        r#"
        [
                {
                    "inputs": [],
                    "name": "factory",
                    "outputs": [{"internalType": "address", "name": "", "type": "address"}],
                    "stateMutability": "view",
                    "type": "function"
                }
            ]
        "#,
    )
    .unwrap();
    let uniswap_v3_pool_address =
        Address::try_from(H160::from_str("0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640").unwrap())
            .unwrap();
    println!("Hello, world!");
    dotenv().ok();
    let env = Env::new();
    let alchemy_url = env.alchemy_url;
    let local_node_url = env.local_node_url;

    let alchemy_w3 =
        Provider::<Http>::try_from(alchemy_url).expect("Couldn't instantiate alchemy w3");
    let local_node_w3 =
        Provider::<Http>::try_from(local_node_url).expect("Couldn't instantiate local node w3");

    let next_block = alchemy_w3.get_block_number().await?;
    println!("next block {:?}", &next_block);

    let now = Instant::now();
    benchmark_req(&alchemy_w3, &next_block).await?;
    println!("Elapsed time Achemy: {:.2?}", now.elapsed());

    let now = Instant::now();
    benchmark_req(&local_node_w3, &next_block).await?;
    println!("Elapsed time Local Node: {:.2?}", now.elapsed());

    let now = Instant::now();
    benchmark_contract_call(
        alchemy_w3.clone(),
        uniswap_v3_pool_address.clone(),
        uniswap_v3_pool_abi.clone(),
    )
    .await?;
    println!("Elapsed time Alchemy: {:?}", now.elapsed());

    let now = Instant::now();
    benchmark_contract_call(
        local_node_w3.clone(),
        uniswap_v3_pool_address.clone(),
        uniswap_v3_pool_abi.clone(),
    )
    .await
    .unwrap();
    println!("Elapsed Time contract call local node: {:?}", now.elapsed());

    Ok(())
}

async fn benchmark_req(prov: &Provider<Http>, block_number: &U64) -> Result<(), ProviderError> {
    for i in 1..100 {
        let block_to_get = block_number.clone() - i.clone();
        prov.get_block(block_to_get).await?;
    }
    Ok(())
}

async fn benchmark_contract_call(
    prov: Provider<Http>,
    swap_address: Address,
    abi: Abi,
) -> Result<(), ProviderError> {
    let arked_prov = Arc::new(prov);
    let pool = Contract::new(swap_address, abi, arked_prov.clone());
    let factory_address = pool.connect(arked_prov.clone()).address();
    println!("factory address: {:?}", factory_address);
    Ok(())
}
