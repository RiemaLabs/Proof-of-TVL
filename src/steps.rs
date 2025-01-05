use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufWriter, Write},
};

use bitcoin::Transaction;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    analyze::deduplicate,
    query::{query_all_tx_by_address, query_all_used_address, AddressResponse},
};

const API_URL: &str = "https://mainnet.prod.lombard.finance/api/v1/addresses";
const BATCH_SIZE: usize = 1000;

// Check from Babylon official dashboard: https://btcstaking.babylonlabs.io/
pub const BTC_PK: &str = "609b4b8e27e214fd830e69a83a8270a03f7af356f64dde433a7e4b81b2399806";

#[derive(Deserialize)]
struct ApiResponse {
    addresses: Vec<ApiAddress>,
    has_more: Option<bool>,
}

#[derive(Deserialize)]
struct ApiAddress {
    btc_address: String,
}

pub const LOMBARD_FILE0: &str = "0_lombard_addresses.txt";

pub async fn cache_lombard_address() -> Vec<String> {
    let mut addresses = query_deposit_address().await.unwrap();
    addresses.sort();

    let file = File::create(LOMBARD_FILE0).unwrap();
    let mut writer = BufWriter::new(file);

    for address in &addresses {
        writeln!(writer, "{}", address).unwrap();
    }

    addresses
}

pub fn load_lombard_address() -> Vec<String> {
    let path = std::path::Path::new(LOMBARD_FILE0);
    if !path.exists() {
        return vec![];
    }
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let addresses: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    addresses
}

pub const LOMBARD_FILE1: &str = "1_lombard_deposit_addresses.txt";

pub async fn query_deposit_address() -> Result<Vec<String>, reqwest::Error> {
    let client = Client::new();
    let mut all_addresses = Vec::new();
    let mut offset = 0;
    let mut batch_number = 1;
    let mut has_more = true;

    while has_more {
        let url = format!("{}?limit={}&offset={}", API_URL, BATCH_SIZE, offset);
        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let ApiResponse {
                addresses,
                has_more: new_has_more,
            } = response.json::<ApiResponse>().await?;

            let new_addresses: Vec<String> = addresses.into_iter().map(|a| a.btc_address).collect();
            all_addresses.extend(new_addresses.clone());

            println!(
                "Batch {} completed: {} addresses",
                batch_number,
                new_addresses.len()
            );

            has_more = match new_has_more {
                Some(more) => more,
                None => false,
            };
            offset += BATCH_SIZE;
            batch_number += 1;
        } else {
            eprintln!("Error: Received status {} from API", response.status());
            break;
        }
    }

    Ok(deduplicate(all_addresses))
}

pub async fn cache_deposit_address(addresses: Vec<String>) -> Vec<AddressResponse> {
    dotenv::dotenv().ok();
    let client = Client::new();
    let url = std::env::var("QUICKNODE_BB_RPC").expect("API_URL must be set");
    let exist_addresses = load_deposit_address();
    let exist_address_set: HashSet<String> = exist_addresses
        .iter()
        .map(|address| address.address.clone())
        .collect();
    let update_addresses: Vec<String> = addresses
        .into_iter()
        .filter(|address| !exist_address_set.contains(address))
        .collect();
    let update_addresses = query_all_used_address(&client, &url, update_addresses).await;
    let mut used_addresses = exist_addresses
        .into_iter()
        .chain(update_addresses)
        .collect::<Vec<_>>();
    used_addresses.sort_by(|a, b| a.address.cmp(&b.address));

    let file = File::create(LOMBARD_FILE1).unwrap();
    let mut writer = BufWriter::new(file);

    for address in &used_addresses {
        writeln!(writer, "{}", serde_json::to_string(&address).unwrap()).unwrap();
    }

    used_addresses
}

pub fn load_deposit_address() -> Vec<AddressResponse> {
    let path = std::path::Path::new(LOMBARD_FILE1);
    if !path.exists() {
        return vec![];
    }
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let addresses = reader
        .lines()
        .filter_map(|line| serde_json::from_str(&line.unwrap()).ok())
        .collect();
    addresses
}

pub const LOMBARD_FILE4: &str = "4_lombard_staking_txs.json";

pub async fn cache_staking_txs(addresses: Vec<String>) -> HashMap<String, Vec<Transaction>> {
    dotenv::dotenv().ok();
    let client = Client::new();
    let url = std::env::var("QUICKNODE_BB_RPC").expect("API_URL must be set");
    let mapping = query_all_tx_by_address(&client, &url, addresses).await;
    let file = File::create(LOMBARD_FILE4).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &mapping).unwrap();

    mapping
}

pub fn load_staking_txs() -> HashMap<String, Vec<bitcoin::Transaction>> {
    let path = std::path::Path::new(LOMBARD_FILE4);
    if !path.exists() {
        return HashMap::new();
    }
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mapping: HashMap<String, Vec<bitcoin::Transaction>> =
        serde_json::from_reader(reader).unwrap();
    mapping
}

pub async fn query_lbtc() -> Option<f64> {
    let url = "https://api.coingecko.com/api/v3/coins/lombard-staked-btc";

    let response = reqwest::get(url).await;
    if response.is_err() {
        println!("Failed to fetch data: {:?}", response.err());
        return None;
    }
    let response = response.unwrap();
    if response.status().is_success() {
        let json: Result<serde_json::Value, reqwest::Error> = response.json().await;
        if json.is_err() {
            return None;
        }
        let json = json.unwrap();
        json["market_data"]["total_supply"].as_f64()
    } else {
        println!("Failed to fetch data. HTTP status: {}", response.status());
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::steps::load_deposit_address;

    use super::*;

    #[tokio::test]
    async fn test_cache_lombard_address() {
        cache_lombard_address().await;
    }

    #[tokio::test]
    async fn test_cache_deposit_address() {
        let addresses = load_lombard_address();
        cache_deposit_address(addresses).await;
    }

    #[test]
    fn test_load_deposit_address() {
        let addresses = load_deposit_address();
        let addresses = addresses
            .into_iter()
            .map(|address| address.address)
            .collect::<Vec<String>>();
        assert!(!addresses.is_empty());
    }

    #[tokio::test]
    async fn test_cache_staking_txs() {
        let addresses = load_deposit_address();
        let addresses = addresses
            .into_iter()
            .map(|address| address.address)
            .collect::<Vec<String>>();
        cache_staking_txs(addresses).await;
    }

    #[test]
    fn test_load_staking_txs() {
        let mapping = load_staking_txs();
        assert!(!mapping.is_empty());
    }

    #[tokio::test]
    async fn test_query_lbtc() {
        let result = query_lbtc().await.unwrap();
        println!("Total LBTC supply: {}", result);
    }
}
