Solana Testing
==============
Uses Kurtosis to run holistic integration tests on Solana, referencing Solana's benchmark local cluster documentation: https://docs.solana.com/cluster/bench-tps . To execute the testsuite, run `scripts/build-and-run.sh all`.

### Genesis and Faucet Configuration Files
Genesis and Faucet files, found in `config-v1.3.23`, were created using Solana's multinode-demo scripts at tag v1.3.23.
These are preserved in this repository. The docker images used by Kurtosis for Solana testnets come with these configurations pre-loaded, allowing faucet and bootstrap nodes to start the networks.

### Solana Testnets
A Solana testnet consists of a faucet node, a bootstrap node, and then the rest of the validators.
The faucet spins up first, the bootstrap spins up referencing the faucet.
