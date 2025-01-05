use futures::future;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, vec};

use reqwest::Client;

use bitcoin::{consensus::Decodable, Transaction};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressResponse {
    pub page: u32,
    pub total_pages: u32,
    pub items_on_page: u32,
    pub address: String,
    pub balance: String,
    pub total_received: String,
    pub total_sent: String,
    pub unconfirmed_balance: String,
    pub unconfirmed_txs: u32,
    pub txs: u32,
    #[serde(default)]
    pub txids: Option<Vec<String>>,
}

impl AddressResponse {
    pub fn total_received(&self) -> u64 {
        self.total_received.parse().unwrap()
    }

    pub fn total_sent(&self) -> u64 {
        self.total_sent.parse().unwrap()
    }

    pub fn balance(&self) -> u64 {
        self.balance.parse().unwrap()
    }
}

pub async fn query_tx_by_txid(client: &Client, url: &str, address: &str) -> Option<Transaction> {
    let payload = json!({
        "method": "getrawtransaction",
        "params": [
            address, 0
        ]
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            let response_json: serde_json::Value = serde_json::from_str(&response_text).unwrap();
            let tx_hex = response_json["result"].as_str();
            if tx_hex.is_none() {
                return None;
            } else {
                let tx_hex = tx_hex.unwrap();
                let tx_bytes = hex::decode(tx_hex);
                if tx_bytes.is_err() {
                    return None;
                } else {
                    let tx_bytes = tx_bytes.unwrap();
                    let mut reader = &tx_bytes[..];
                    let tx = Transaction::consensus_decode(&mut reader);
                    if tx.is_err() {
                        return None;
                    } else {
                        Some(tx.unwrap())
                    }
                }
            }
        }
        Err(_) => None,
    }
}

pub async fn query_address(client: &Client, url: &str, address: &str) -> Option<AddressResponse> {
    let payload = json!({
        "method": "bb_getAddress",
        "params": [
            address,
            {
                "page": 1,
                "size": 1000,
                "fromHeight": 0,
                "details": "txids"
            }
        ]
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            let response_json: serde_json::Value = serde_json::from_str(&response_text).unwrap();
            let result = response_json.get("result");
            if result.is_none() {
                return None;
            }
            let result = result.unwrap();
            let address = serde_json::from_value::<AddressResponse>(result.clone());
            if address.is_err() {
                println!("Error address: {:?}", address);
                return None;
            }
            let address = address.unwrap();
            Some(address)
        }
        Err(_) => None,
    }
}

pub async fn query_all_used_address(
    client: &Client,
    url: &str,
    addresses: Vec<String>,
) -> Vec<AddressResponse> {
    let mut results = Vec::new();
    let batch_size = 100;
    let chunked_addresses: Vec<Vec<String>> =
        addresses.chunks(batch_size).map(|c| c.to_vec()).collect();

    for c in chunked_addresses {
        let address_futures = c.iter().map(|address| query_address(client, url, address));
        let query_result = future::join_all(address_futures).await;
        results.extend(query_result.into_iter().filter_map(|r| r));
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
    }
    results.sort_by(|a, b| a.address.cmp(&b.address));
    results
}

pub async fn query_tx_by_address(client: &Client, url: &str, address: &str) -> Vec<Transaction> {
    let payload = json!({
        "method": "bb_getAddress",
        "params": [
            address,
            {
                "page": 1,
                "size": 1000,
                "fromHeight": 0,
                "details": "txids"
            }
        ]
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            let response_json: serde_json::Value = serde_json::from_str(&response_text).unwrap();
            let received = response_json["result"]["txids"].as_array();
            if received.is_none() {
                return vec![];
            } else {
                let received = received.unwrap();
                let txid = received
                    .iter()
                    .map(|tx| tx.as_str().unwrap())
                    .collect::<Vec<&str>>();
                let tx_futures = txid.iter().map(|txid| query_tx_by_txid(client, url, txid));
                let tx_results: Vec<Option<Transaction>> = future::join_all(tx_futures).await;
                tx_results.into_iter().filter_map(|tx| tx).collect()
            }
        }
        Err(_) => vec![],
    }
}

pub async fn query_all_tx_by_address(
    client: &Client,
    url: &str,
    addresses: Vec<String>,
) -> HashMap<String, Vec<Transaction>> {
    let mut tx_map = HashMap::new();

    for address in addresses {
        let txs = query_tx_by_address(client, url, &address).await;

        tx_map.insert(address, txs);
    }

    tx_map
}
