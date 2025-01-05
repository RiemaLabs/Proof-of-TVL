# Proof of TVL: Regaining Trust in BTCFi
*Authors: [Nubit](https://x.com/nubit_org) & [Nebra](https://x.com/nebrazkp)*

*Special thanks to [Bitcoin Layers](https://x.com/BitcoinLayers) and [Alpen Labs](https://x.com/AlpenLabs) for reviewing and providing valuable feedback.*


## The Evolution of BTCFi and Liquid Staking Tokens (LSTs)

Bitcoin has long been the backbone of the digital asset ecosystem. Yet for years, its utility was largely confined to being a store of value and medium of exchange. In 2023, protocols like Babylon introduced the concept of Bitcoin staking, allowing users to lock their BTC in a self-custodial manner and participate in Proof-of-Stake consensus mechanisms to earn rewards.

This innovation unlocked a new era for Bitcoin, now often referred to as **BTCFi**, a movement that enables Bitcoin to do more than ever before. With BTCFi, Bitcoin is no longer limited to passive holding but can actively participate in DeFi ecosystems.

To enhance the usability and liquidity of staked Bitcoin, a wave of Bitcoin Liquid Staking Tokens (LSTs) emerged. These protocols act as custodians, enabling users to stake their BTC and receive tokenized representations in return. These LSTs can be freely deployed in DeFi applications and used for borrowing, lending, trading, or yield farming. This model lets Bitcoin stakers "have their cake and eat it too" by earning staking rewards while still participating in the countless DeFi opportunities.

These LSTs have quickly gained adoption, with protocols reporting billions of dollars in Total Value Locked (TVL), a metric often viewed as the gold standard for gauging user activity and protocol success.![LST](https://hackmd.io/_uploads/B1R79tP8kx.png)


The fundamental question we want to raise for the industry is this: **How valid are the TVL metrics being reported by Bitcoin LST protocols?** More specifically, should BTC that the protocol cannot actually control or slash still be counted toward TVL? If these metrics are overstated, it can create a false sense of security and mislead both users and investors. An inflated TVL may mask the true liquidity and risk profile of a protocol, leading to misguided decisions and potential losses for everyone involved.

## Why Bitcoin Liquid Staking Protocols TVL is hard to track?

In the context of Bitcoin staking, the unique UTXO model introduces complexities that make TVL figures difficult to interpret. This undermines trust in Bitcoin LST protocols and raises concerns about the overall sustainability of the BTCFi ecosystem.

Let’s take a closer look at why.

Bitcoin operates on the UTXO model, where each transaction creates discrete "chunks" of Bitcoin with specific spending conditions. For example:

- A UTXO might require a private key signature to spend.
- More complex UTXOs might include multisig requirements or timelocks.

Unlike Ethereum’s account-based model, Bitcoin’s UTXO approach doesn’t aggregate balances, making it more complicated—though not impossible—to track and lock funds. As a result, the TVL figures of LST protocols end up being self-reported in practice. To verify these claims, we start with a simple question:

> How should we count Bitcoin LST TVL?

The goal of Bitcoin staking protocol is to provide economic security for application layer protocols such as Rollups, DAs and many others. In that angle, the economic security is only valid if the staked BTC is under the staking protocols custodian or slashable. So there is one thing crystal clear:

> The BTC that is not under the staking protocol nor slashable cannot be counted in the TVL.

## How does the Bitcoin Restaking TVL get unreal today?

Many Bitcoin LST protocols are pursuing high TVL at all costs, striking deals with large holders to artificially boost their numbers.

Here’s how it works:

1. Large Bitcoin holders (whales) are incentivized to “stake” their BTC by transferring it to an address jointly controlled by the whale and the protocol.
2. The whale retains ultimate control over the UTXO. The protocol has no power to enforce redemptions or penalties (including slashing), meaning the funds are never truly at risk.
3. The protocol counts the UTXO as part of its TVL, even though the funds are not locked in any meaningful way and could be withdrawn or reused by the whale at any time.

In reality:

- The user retains full control of the UTXO and can spend it or pledge it elsewhere at any time.
- This "staking" process has no enforceable slashing conditions, making it functionally meaningless.


At its core, staking is meant to secure a network by incentivizing good behavior and punishing malicious actions through mechanisms like slashing. Slashing ensures that participants risk their funds if they act dishonestly or against the protocol’s rules. Without this, staking becomes a meaningless exercise: a "stake-for-the-sake-of-stake" charade.

Ask yourself: What’s the purpose of staking? It’s not about inflating TVL metrics or signaling intent. Instead, it’s about enabling slashing and creating a security guarantee for the protocol.

Consider the cautionary tale of FTX, where a reported figures (receipt tokens) and real reserves (redeemable assets) led to a catastrophic collapse in user trust. If a protocol is overstating its TVL, can you really trust it not to misuse your reserves behind the scenes? **A protocol that distorts something as fundamental as reserves is unlikely to honor the trustless principles Bitcoin represents.**

This inflated TVL figure prompts a broader question: Are the Bitcoin reported as “staked” genuinely locked, or is it simply a misrepresented metric intended to boost numbers and draw attention?


## The Risks of Unreal TVL

In theory, Liquid Staking Tokens (LSTs) are intended to represent Bitcoin staked into protocols like Babylon, enabling holders to earn rewards while still retaining liquidity. The premise is that each LST is fully backed on a 1:1 basis by actual Bitcoin reserves. However, certain staking arrangements that emphasize large TVL figures can call these assurances into question. If a portion of the staked BTC remains entirely under the original owner’s control while the protocol simultaneously reports it as fully locked, the foundational assumptions behind LSTs may begin to erode. The actual locked collateral could be less than stated, the staking model may not deliver the intended security guarantees, and the reported TVL might diverge from the genuine amount of BTC truly at stake. Ultimately, these practices challenge the notion of LSTs being completely backed by verifiable reserves, raising doubts about the real level of economic security these tokens provide, such as:

1. **(No Assurance of 1:1 Backing)** Since protocols count Bitcoin as "staked" without it being genuinely locked or staked, there is no guarantee that the assets backing the issued LSTs actually exist or are under the protocol’s control. Users holding these tokens are left to rely solely on the protocol’s claims. Furthermore, **if the backing isn’t there, users face real risks of financial losses during redemption** of the underlying assets.
2. **(Unverifiable Staking Rewards)** Staking rewards are supposed to come from legitimate contributions to network security or PoS consensus. But when the underlying Bitcoin is not genuinely staked, where are these rewards coming from? Are they sustainable?

This is a systemic risks to BTCFi. As confidence wanes, liquidity drains from the system, destabilizing not just one protocol but the entire BTCFi ecosystem built on Bitcoin staking.

**What happens when Bitcoin staking protocols become indistinguishable from centralized entities, where users cannot audit reserves and must trust the operators?**

The current situation is an existential threat to BTCFi’s credibility. To avoid repeating the mistakes of centralized systems and fake TVL, we must address the root cause: the lack of a trustless, verifiable mechanism for proving reserves and staking activity.

This is where **Proof of TVL (PoTVL)** becomes critical. A scientific, transparent, and cryptographic standard for reserve validation is the only way forward to restore trust in Bitcoin LSTs and ensure the ecosystem’s long-term sustainability.


## A Vanilla Solution: Calculating TVL with Transparency

In the context of Bitcoin staking, Taproot addresses play a key role in implementing staking lock scripts, such as Babylon. These lock scripts define clear rules for how BTC can be staked, tracked, and eventually withdrawn. Babylon is a great example, as it ties staking actions directly to verifiable, protocol-level rules on Bitcoin’s UTXO model.



When stakers participate in a staking protocol, they construct special transactions that send BTC to a protocol-designated Taproot address. These transactions typically include:

- The staking output: A UTXO that sends the BTC to the Taproot address for staking.
- The ownership verification output: A second UTXO that includes public keys for the staker and the protocol. These public keys prove ownership of the staked BTC.

An example is  [Babylon staking protocol's Spec](https://github.com/babylonlabs-io/babylon/blob/main/docs/transaction-impl-spec.md), where:

The specication requires stakers (or LST protocols) to construct the transaction as:
- The first UTXO sends BTC to a Taproot address tied to Babylon’s staking lock script.
- The second UTXO includes public keys for both the staker and Babylon, ensuring ownership verification.

This design ensures that staking can be tracked entirely on-chain with clear proof of ownership and transparent rules.


## Case Study: Lombard Finance 

To demonstrate how this methodology works in practice, we applied it to Lombard Finance using our open-source tool, [Proof of TVL](https://github.com/RiemaLabs/Proof-of-TVL). 

Here’s the entire process:


1. **(Identify User Deposit Wallets)** Start with user wallets depositing BTC into Lombard. These wallets represent the initial flow of funds into the system.
2. **(Trace Transactions to Staking Wallets)** Follow the flow of BTC from deposit wallets to staking wallets controlled by Lombard. Identify all staking transactions based on Babylon’s staking specification.
3. **(Verify Ownership)** Use Babylon's protocol rules to confirm that the staking transactions include the required public keys for ownership verification. Ensure that the transactions conform to the staking lock script.
4. **(Calculate Real TVL)** Sum up the BTC outputs in verified staking transactions to compute the onchain collateral. Compare the collateral to the total supply of LBTC to calculate the collateralization ratio.

Using the steps above, we calculated Lombard’s LST TVL as follows:

```
BTC On-Chain Collateral: 16,580.9220 (15,028.3565 BTC / 90.64% Staked)
LBTC Total Supply: 16,386.4157 (101.19% Collateralized)
Latest Verification Date: Jan 4, 2025, 7:30 PM PST
Status: SAFE (101.19% Collateralized)
```

1. **(90.64% Staked)** Out of 16,580.9220 BTC on-chain collateral, 15,028.3565 BTC is actively staked into Babylon.
2. **(101.19% Collateralized)** The total supply of LBTC is 16,386.41, while the on-chain collateral is 16,599 BTC.
3. **(Full On-Chain Transparency)** Every staking transaction can be traced directly to Lombard’s protocol deposit addresses, and ownership verification matches the staking rules.

The verification was performed on **Jan 4, 2025, at 7:30 PM PST** when writing this article. This data is fully reproducible with no manual intervention. Using our open-source [Proof of TVL](https://github.com/RiemaLabs/Proof-of-TVL) tool, anyone can independently verify LBTC's TVL data in real-time.

This is **transparency**.


While this solution provides transparency, it has a critical flaw: it relies on trusting the protocol to accurately compute and report TVL using the proposed procedure. Is there a way to eliminate this reliance and allow anyone to independently verify the results with confidence? Zero-Knowledge Proofs (ZKPs) offer a path forward.


## Proof of TVL using zero-knowledge proofs


One advantage of zero-knowledge proof is the cryptographic trust with very low verification cost so that everyone can verify the zero-knowledge proofs on her client devices such as mobile phones or browsers. This further lowest the friction and the trust assumption of Proof of TVL. Now you don't even need to trust the 3rd party who is running the Proof of TVL protocol.


The concrete statement of the zero-knowledge proof that can attest the LST TVL is:

$$ \textsf{BTC}\text{ on Babylon from LST} + \text{Proof of Reserve of LST Wallets} \geq \text{LST total supply}$$

- $\textsf{BTC}\text{ on Babylon from LST}$: From Babylon transaction spec: For the transaction to be considered a valid staking transaction, it must have Taproot output which has the key spending path disabled and commits to a script tree composed of three scripts:: timelock script, unbonding script, slashing script. This output is henceforth known as the staking_output and the value in this output is known as staking_amount. Additionally, it must have `OP_RETURN` output which contains: `global_parameters.tag`, `version`, `staker_pk`, `finality_provider_pk`, `staking_time`. To verify the LST's BTC on Babylon, we need to first check the validity of the staking transactions, for example, the Taproot output and the `OP_RETURN` have the same public key.

- Proof of Reserve of LST Wallets: We can adopt the standard proof of reserve protocol here, for example, the one proposed by Vitalik Buterin: https://vitalik.eth.limo/general/2022/11/19/proof_of_solvency.html. There is a slightly improved version proposed by Shumo et. al. 

    > Further read: SNARKed Merkle Sum tree: https://ethresear.ch/t/snarked-merkle-sum-tree-a-practical-proof-of-solvency-protocol-based-on-vitaliks-proposal/14405

    The only technicality here is the we need to replace the signature algorithm used by Ethereum to the one used by Bitcoin. For example, despite both Bitcoin and Etheruem uses ECDSA, Bitcoin chose SHA instead of Keccak as the secure Hash algorithm.
    
- LST Total Supply: This can be a public input from the users. 

Proof of TVL using zero-knowledge proofs can effectively minimize the counter party risk and lower the barrier for any users to verify the result. 

## Conclusion: The Road Ahead for BTCFi

Bitcoin has always stood for trust, decentralization, and transparency. Yet the rise of inflated TVL metrics in Bitcoin staking threatens to erode these principles.

The solution is clear: Proof of TVL, enabled by Zero-Knowledge Proofs, offers a path to true accountability.

By eliminating reliance on trust and making reserves verifiable by anyone, we can rebuild confidence in Bitcoin LSTs and ensure BTCFi is built on a foundation of truth.


---

## Join the Conversation  

Let’s work together to restore transparency to BTCFi:  
- **Contribute**: Submit PRs, suggest improvements, or help refine industry standards.  
- **Join the discussion**: [Telegram](https://t.me/+HnKQI9iz4ic3NmQ1)
- **Request Proof of TVL**: Let us know which protocols you’d like us to analyze next!  

---

## Continuing Engagement  

We believe in collective progress. Here’s how you can help drive this forward:  
1. **Provide More Proof of TVL Analyses**: Help expand the scope by contributing analyses for other BTCFi protocols. Transparency is an ecosystem-wide effort.
2. **Contribute PRs**: Improve the tool or propose new features (e.g., zk-proofs implementation).
3. **Build Standards**: Collaborate with us to create open, verifiable standards for BTCFi transparency. 
4. **Spread the Word**: Share this article to raise awareness of the need for trustless TVL verification.


