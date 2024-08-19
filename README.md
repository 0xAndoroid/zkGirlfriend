# ZK Girlfriend
*A project built over a weekend at the Paradigm Frontiers confrence* 

ZK Girlfriend allows you to prove you have a girlfriend at another school without showing your messages to prove you have a gf. It uses [SP1 by Succinct](https://github.com/succinctlabs/sp1) to make a ZK proof of the *GF score* (patent pending) output by our algorithm written in rust. 

## User flow:

- Forward your messages to the ZK Girlfirend telegram bot
- The bot sends your messages to our backend which generates a ZK proof of the GF score
- Our backend mints an NFT to your wallet if your GF score is above a certain threshold proving you have a GF
