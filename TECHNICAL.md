# Proof of TVL: Restoring Trust in Bitcoin LSTs

## Get started

First, copy the example env file.

```bash
cp .env.example .env
```

Then, please specify the RPC endpoint in the `.env` file as `QUICKNODE_BB_RPC` because the proof uses Bitcoin RPC with Blockbook.
Here is the documentation: [RPC Documentation](https://www.quicknode.com/docs/bitcoin/bb_getaddress)

## Snapshot at 2025-01-05 05:04:08 UTC

```bash
Address bc1phz9f27wshtset37f96xl266w9zaq0wdmls749qad2rj3zz4zc8psmgts3w stakes 1000.0116 BTC in Babylon
Address bc1pt0kh88fl2uadsfkp76wg9e5zdfcsqpuzv8cwy4c3h0djcj3sndfqwetd0s stakes 499.8500 BTC in Babylon
Address bc1pt0d6nnr7563lmfrzwpmegpdtefu4q3858tt3jq0d08jy5ryd47csuamllz stakes 99.4979 BTC in Babylon
Address bc1p3nmr2yzmgaml07lx7n3ehe8a2uvu2nrqfrqx8leqt562t4dnm8tsqxmpyt stakes 13129.0000 BTC in Babylon
Address bc1prvdw5sjll3rpvuxfneq6mvpc9rvffkfqg5dfk94skqt2knsr8lksagvns4 stakes 299.9970 BTC in Babylon

BTC On-Chain Collateral: 16580.9220 BTC (15028.3565 BTC / 90.64% Staked)
LBTC Total Supply: 16386.4157 LBTC (101.19% Collateralized)
Status: SAFE (101.19% Collateralized)
Latest Verification Time: 2025-01-05 05:04:08 UTC
```

## Fetch the Latest Data

Please remove the cached files to fetch the latest data.

```bash
rm 0_lombard_addresses.txt 1_lombard_deposit_addresses.txt 4_lombard_staking_txs.json
cargo run
```
