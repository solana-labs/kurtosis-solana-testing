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
with the following pub/priv files:

`keypair1.json`:
```
[8,239,195,145,153,173,163,233,39,61,180,167,59,141,13,79,236,79,134,138,150,57,159,237,131,89,74,97,157,23,99,177,138,99,98,180,110,233,182,244,196,23,61,14,148,202,15,81,130,66,97,44,5,185,100,126,137,70,107,25,175,114,245,104]
```

`pubkey1.txt`:
```
AKDAFBjxJ5hZ1YzDLcZsTsgs4XkoZQjacyWrSZmQobTV
```

`keypair2.json`:
```
[138,73,94,205,144,202,189,78,39,216,83,192,205,86,177,230,34,122,72,48,200,48,227,24,164,102,125,184,113,160,35,78,232,252,122,226,195,180,7,250,120,129,79,52,132,145,149,31,89,240,243,117,11,117,138,77,5,83,212,110,137,200,233,168]
```

`pubkey2.txt`:
```
GgUt4yfXYboFzithBaaAjwqtSACH9aTHnREg8Lm12XxK
```

`keypair3.json`:
```
[212,20,161,123,58,72,232,147,81,27,144,71,69,192,105,14,35,78,253,241,126,233,208,13,20,52,123,180,134,231,104,95,163,162,29,250,196,96,90,240,208,253,212,188,171,219,43,216,222,231,149,55,48,20,5,188,239,177,197,150,10,243,238,143]
```

`pubkey3.txt`:
```
C1kqFbviEVdVy1UMNPP3SjzVdKEkrnPB38yGm3MWNzsY
```

`keypair4.json`:
```
[98,61,131,19,100,112,23,19,71,180,15,158,53,126,145,5,207,115,185,112,191,97,98,215,74,90,219,67,105,31,244,131,84,41,25,25,130,143,182,20,63,58,155,135,162,60,11,106,226,105,185,104,136,236,60,225,214,46,120,127,13,6,22,226]
```

`pubkey4.txt`:
```
6fXbSzD2ZW7cVwPdquiFg78xqA3ZaFRVfEB6Z2Zntavu
```

The docker images used by Kurtosis for Solana testnets come with these configurations pre-loaded, allowing faucet and bootstrap nodes to start the networks.

### Solana Testnets
A Solana testnet consists of a faucet node, a bootstrap node, and then the rest of the validators.
The faucet spins up first, the bootstrap spins up referencing the faucet.
