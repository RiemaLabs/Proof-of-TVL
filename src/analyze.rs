use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use bitcoin::{Address, Network, Transaction};

pub fn deduplicate(vec: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    vec.into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}

pub fn analyze_funding_flows(address_txs: HashMap<String, Vec<Transaction>>) -> Vec<String> {
    let mut stake_addresses = vec![];
    for (_, txs) in address_txs {
        for tx in txs {
            for output in tx.output {
                let script = output.script_pubkey;
                let address = Address::from_script(&script, bitcoin::Network::Bitcoin);
                if address.is_err() {
                    continue;
                }
                let address = address.unwrap();
                stake_addresses.push(address.to_string());
            }
        }
    }
    let result = deduplicate(stake_addresses);
    println!("Found {} staking addresses", result.len());
    result
}

pub fn analyze_address_babylon_staking(
    btc_pk: &str,
    address_txs: HashMap<String, Vec<Transaction>>,
) -> HashMap<String, u64> {
    let mut mappings = HashMap::new();
    let mut all_tx_ids: HashSet<String> = HashSet::new();
    for (_, txs) in address_txs {
        for tx in txs {
            let txid = tx.compute_ntxid().to_string();
            if all_tx_ids.contains(&txid) {
                continue;
            }
            let (output_address, sub_amount) = get_babylon_staking_amount(btc_pk, &tx);
            if !mappings.contains_key(&output_address) {
                mappings.insert(output_address, sub_amount);
            } else {
                let mut amount = mappings.get(&output_address).unwrap().clone();
                amount += sub_amount;
                mappings.insert(output_address, amount);
            }
            all_tx_ids.insert(tx.compute_ntxid().to_string());
        }
    }
    mappings
}

/// Use this impl spec to identify babylon staking tx:
/// https://github.com/babylonlabs-io/babylon/blob/main/docs/transaction-impl-spec.md
pub fn get_babylon_staking_amount(btc_pk: &str, tx: &Transaction) -> (String, u64) {
    let default_result = (String::from(""), 0);
    if tx.output.len() == 1 {
        return default_result;
    }
    let op_return = &tx.output[1];
    let script = &op_return.script_pubkey;
    let data = script.to_bytes();
    if data.len() != 1 + 1 + 71 {
        return default_result;
    }
    if data[0] != 0x6a {
        return default_result;
    }
    if data[1] != 0x47 {
        return default_result;
    }
    let encoded_btc_pk = hex::encode(&data[39..39 + 32]);
    if encoded_btc_pk != btc_pk {
        return default_result;
    }
    let output_address = Address::from_script(&tx.output[0].script_pubkey, Network::Bitcoin)
        .unwrap()
        .to_string();
    return (output_address, tx.output[0].value.to_sat());
}
