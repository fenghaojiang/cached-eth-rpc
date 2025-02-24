use serde_json::{json, Value};

pub async fn get_chain_id(rpc_url: &str) -> anyhow::Result<u64> {
    let request_payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_chainId",
        "params": [],
        "id": 1
    });

    let client = reqwest::Client::new();
    let response = client.post(rpc_url).json(&request_payload).send().await?;

    let json: Value = response.json().await?;
    match json["result"].as_str() {
        Some(chain_id) => Ok(u64::from_str_radix(&chain_id[2..], 16)?),
        None => Err(anyhow::anyhow!("fail to get chain id: {json}")),
    }
}
