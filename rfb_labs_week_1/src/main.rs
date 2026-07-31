use rfb_labs_week_1::labs::lab01_network::inspect_network;
use rfb_labs_week_1::rpc::ProcessRpc;
use std::env;

fn main() {
    dotenvy::dotenv().ok(); // loads .env into the environment; .ok() ignores a missing file

    let container = env::var("RPC_DOCKER_CONTAINER")
        .unwrap_or_else(|_| "polar-n2-backend1".to_string());
    let user = env::var("RPC_USER").expect("RPC_USER must be set (check your .env file)");
    let password = env::var("RPC_PASSWORD").expect("RPC_PASSWORD must be set (check your .env file)");

    let rpc = ProcessRpc::new("docker").with_base_args([
        "exec".to_string(),
        container,
        "bitcoin-cli".to_string(),
        "-regtest".to_string(),
        format!("-rpcuser={user}"),
        format!("-rpcpassword={password}"),
    ]);

    match inspect_network(&rpc) {
        Ok(snapshot) => println!("{snapshot:#?}"),
        Err(error) => eprintln!("error: {error}"),
    }
}