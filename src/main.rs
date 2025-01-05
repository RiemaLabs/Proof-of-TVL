use analyze::analyze_address_babylon_staking;

mod analyze;
mod query;
mod steps;

#[tokio::main]
async fn main() {
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

    println!(
        "Holding BTC amount: {}",
        cached_deposit_address
            .iter()
            .map(|address| address.total_received() - address.total_sent())
            .sum::<u64>()
    );

    // Ignore addresss with less than 10 BTC to speed up.
    let cached_staking_addresses = cached_deposit_address
        .into_iter()
        .filter(|address| address.total_received() - address.total_sent() > 10_00_000_000)
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
        println!("Staked BTC in address {}: {}", address, sub_amount);
        amount += sub_amount;
    }
    println!("Staking BTC amount: {}", amount);
}
