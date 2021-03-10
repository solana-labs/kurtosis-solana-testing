Solana Testing
==============
Uses Kurtosis to run holistic integration tests on Solana, referencing Solana's benchmark local cluster documentation: https://docs.solana.com/cluster/bench-tps . To execute the testsuite, run `scripts/build-and-run.sh all`.

### Genesis and Faucet Configuration Files
The genesis configuration was created using the following script:

```bash
set -euo pipefail

NUM_BOOTSTRAPPERS=5
KEYPAIRS_DIRPATH="${HOME}/gdrive/project-support/solana-validating-testnet/base58-keypairs"
SOLANA_CLIS_DIRPATH="$HOME/code/solana/target/debug"
OUTPUT_LEDGER_DIRPATH="/tmp/ledger-dir"

function get_pubkey_by_index() {
    index="${1}"
    cat "${KEYPAIRS_DIRPATH}/pubkey${index}.txt"
}

function get_keypair_by_index() {
    index="${1}"
    cat "${KEYPAIRS_DIRPATH}/keypair${index}.json"
}

rm -rf "${OUTPUT_LEDGER_DIRPATH}"
bootstrapper_args=()
next_key_index=2
echo "pub const GENESIS_BOOTSTRAPPER_KEYPAIRS: &'static [GenesisBootstrapperKeypairs] = &["
for i in $(seq 1 "${NUM_BOOTSTRAPPERS}"); do
    identity_pubkey="$(get_pubkey_by_index "${next_key_index}")"
    identity_keypair="$(get_keypair_by_index "${next_key_index}")"
    next_key_index="$((next_key_index + 1))"

    vote_account_pubkey="$(get_pubkey_by_index "${next_key_index}")"
    vote_account_keypair="$(get_keypair_by_index "${next_key_index}")"
    next_key_index="$((next_key_index + 1))"

    stake_account_pubkey="$(get_pubkey_by_index "${next_key_index}")"
    stake_account_keypair="$(get_keypair_by_index "${next_key_index}")"
    next_key_index="$((next_key_index + 1))"

    cat << EOF
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "${identity_keypair}",
            pubkey: "${identity_pubkey}",
        },
        vote_account: GenesisKeypair{
            keypair_json: "${vote_account_keypair}",
            pubkey: "${vote_account_pubkey}",
        },
        stake_account: GenesisKeypair{
            keypair_json: "${stake_account_keypair}",
            pubkey: "${stake_account_pubkey}",
        },
    },
EOF

    bootstrapper_args+=("-b ${identity_pubkey} ${vote_account_pubkey} ${stake_account_pubkey}")
done
echo "];"

# WARNING: Do NOT use --enable-warmup-epochs here!! If it's used, spurious failures will be thrown while under network partition
"${SOLANA_CLIS_DIRPATH}/solana-genesis" \
    --cluster-type testnet \
    `# Tells the validators to sleep, rather than hash, in order to form the logical clock` \
    --hashes-per-tick sleep \
    --faucet-pubkey $(cat ~/gdrive/project-support/solana-validating-testnet/base58-keypairs/pubkey1.txt) \
    ${bootstrapper_args[@]} \
    --ledger "${OUTPUT_LEDGER_DIRPATH}" \
    --faucet-lamports "5000000000000" \
    > /dev/null

echo "pub const GENESIS_HASH: &str = \"$(RUST_LOG=none "${SOLANA_CLIS_DIRPATH}/solana-ledger-tool" genesis-hash -l "${OUTPUT_LEDGER_DIRPATH}")\";"
echo "pub const BANK_HASH: &str = \"$(RUST_LOG=none "${SOLANA_CLIS_DIRPATH}/solana-ledger-tool" bank-hash -l "${OUTPUT_LEDGER_DIRPATH}")\";"
echo "pub const SHRED_VERSION: u64 = $(RUST_LOG=none "${SOLANA_CLIS_DIRPATH}/solana-ledger-tool" shred-version -l "${OUTPUT_LEDGER_DIRPATH}");"
```

with the script printing the Rust code that needs to be stored in the `genesis_config.rs` file.

The docker images used by Kurtosis for Solana testnets come with these configurations pre-loaded, allowing faucet and bootstrap nodes to start the networks.

### Solana Testnets
A Solana testnet consists of a faucet node, a bootstrap node, and then the rest of the validators.
The faucet spins up first, the bootstrap spins up referencing the faucet.
