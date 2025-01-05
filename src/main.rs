use analyze::analyze_address_babylon_staking;
use chrono::Utc;

mod analyze;
mod query;
mod steps;

const SATS: u64 = 100_000_000;

#[tokio::main]
async fn main() {
    // -------- Notice cached data are used here. Remove the cache and fetch them again to get the latest result. --------

    // -------- Step 0: Initialize addresses --------
    let mut cached_address = steps::load_lombard_address();
    if cached_address.is_empty() {
        cached_address = steps::cache_lombard_address().await;
    }

    // -------- Step 1: Query Lombard API for all deposit addresses --------
    let mut cached_deposit_address = steps::load_deposit_address();
    if cached_deposit_address.is_empty() {
        cached_deposit_address = steps::cache_deposit_address(cached_address).await;
    }

    let btc_on_chain = cached_deposit_address
        .iter()
        .map(|address| address.balance() as f64)
        .sum::<f64>()
        / SATS as f64;

    // Ignore addresss with less than 10 BTC to speed up.
    let cached_staking_addresses = cached_deposit_address
        .into_iter()
        .filter(|address| address.balance() > 10 * SATS)
        .map(|address| address.address)
        .collect::<Vec<String>>();

    // -------- Step 2: Query txs in all staking addresses to analyze staking flows --------

    let mut cached_staking_txs = steps::load_staking_txs();
    if cached_staking_txs.is_empty() {
        cached_staking_txs = steps::cache_staking_txs(cached_staking_addresses).await;
    }

    let mut amount = 0;
    let result = analyze_address_babylon_staking(steps::BTC_PK, cached_staking_txs);
    for (address, sub_amount) in result {
        if sub_amount == 0 {
            continue;
        }
        println!(
            "Address {} stakes {:.4} BTC in Babylon",
            address,
            sub_amount as f64 / SATS as f64
        );
        amount += sub_amount;
    }
    println!("");
    let staked_btc = amount as f64 / SATS as f64;

    println!(
        "BTC On-Chain Collateral: {:.4} BTC ({:.4} BTC / {:.2}% Staked)",
        btc_on_chain,
        staked_btc,
        staked_btc / btc_on_chain * 100.0
    );

    // -------- Step 3: Query LBTC Amount --------
    let lbtc_total_supply = steps::query_lbtc().await.unwrap();

    println!(
        "LBTC Total Supply: {:.4} LBTC ({:.2}% Collateralized)",
        lbtc_total_supply,
        btc_on_chain / lbtc_total_supply * 100.0
    );
    if btc_on_chain / lbtc_total_supply > 1.0 {
        println!(
            "Status: SAFE ({:.2}% Collateralized)",
            btc_on_chain / lbtc_total_supply * 100.0
        );
    } else {
        println!(
            "Status: UNSAFE ({:.2}% Collateralized)",
            btc_on_chain / lbtc_total_supply * 100.0
        );
    }
    println!(
        "Latest Verification Time: {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );
}
