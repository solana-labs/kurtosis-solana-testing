Solana Testing
==============
Uses Kurtosis to run holistic integration tests on Solana, referencing Solana's benchmark local cluster documentation: https://docs.solana.com/cluster/bench-tps . To execute the testsuite, run `scripts/build-and-run.sh all`.

### Genesis and Faucet Configuration Files
The genesis configuration was created using the following command:

```bash
./target/debug/solana-genesis \
    --cluster-type testnet \
    --enable-warmup-epochs \
    --faucet-pubkey $(cat ~/gdrive/project-support/solana-validating-testnet/base58-keypairs/pubkey1.txt) \
    -b $(cat ~/gdrive/project-support/solana-validating-testnet/base58-keypairs/pubkey2.txt) \
        $(cat ~/gdrive/project-support/solana-validating-testnet/base58-keypairs/pubkey3.txt) \
        $(cat ~/gdrive/project-support/solana-validating-testnet/base58-keypairs/pubkey4.txt) \
    --ledger /tmp/ledger-dir \
    --faucet-lamports "5000000000000"
```
with the contents private keypairs stored in the `genesis_config.rs` file.

The docker images used by Kurtosis for Solana testnets come with these configurations pre-loaded, allowing faucet and bootstrap nodes to start the networks.

### Solana Testnets
A Solana testnet consists of a faucet node, a bootstrap node, and then the rest of the validators.
The faucet spins up first, the bootstrap spins up referencing the faucet.
