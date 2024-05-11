use dotenv::dotenv;
use ethers::providers::{Http, Middleware, Provider, ProviderError};
use ethers::types::U64;
use std::fmt::Error;
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
    println!("Hello, world!");
    dotenv::dotenv().ok();
    let env = Env::new();
    let alchemy_url = env.alchemy_url;
    let local_node_url = env.local_node_url;

    let alchemy_w3 =
        Provider::<Http>::try_from(alchemy_url).expect("Couldn't instantiate alchemy w3");
    let local_node_w3 =
        Provider::<Http>::try_from(local_node_url).expect("Couldn't instantiate local node w3");

    let next_block = alchemy_w3.get_block_number().await?;
    println!("next block {:?}", next_block.clone());

    let now = Instant::now();
    benchmark_req(alchemy_w3, next_block.clone()).await?;
    println!("Elapsed time Achemy: {:.2?}", now.elapsed());

    let now = Instant::now();
    benchmark_req(local_node_w3, next_block.clone()).await?;
    println!("Elapsed time Local Node: {:.2?}", now.elapsed());

    Ok(())
}

async fn benchmark_req(prov: Provider<Http>, block_number: U64) -> Result<(), ProviderError> {
    for i in 1..100 {
        let block_to_get = block_number.clone() - i.clone();
        prov.get_block(block_to_get).await?;
    }
    Ok(())
}
