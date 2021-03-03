Solana Testing
==============
Uses Kurtosis to run holistic integration tests on Solana, referencing Solana's benchmark local cluster documentation: https://docs.solana.com/cluster/bench-tps . To execute the testsuite, run `scripts/build-and-run.sh all`.

### Genesis and Faucet Configuration Files
The genesis configuration was created using the following script:

```bash
set -euo pipefail

NUM_BOOTSTRAPPERS=10
KEYPAIRS_DIRPATH="${HOME}/gdrive/project-support/solana-validating-testnet/base58-keypairs"

function get_pubkey_by_index() {
    index="${1}"
    cat "${KEYPAIRS_DIRPATH}/pubkey${index}.txt"
}

function get_keypair_by_index() {
    index="${1}"
    cat "${KEYPAIRS_DIRPATH}/keypair${index}.json"
}

rm -rf /tmp/ledger-dir
bootstrapper_args=()
next_key_index=2
echo "pub const GENESIS_BOOTSTRAPPER_KEYPAIRS: &'static [GenesisBootstrapperKeypairs] = &["
for i in $(seq 1 10); do
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

$HOME/code/solana/target/debug/solana-genesis \
    --cluster-type testnet \
    --enable-warmup-epochs \
    --faucet-pubkey $(cat ~/gdrive/project-support/solana-validating-testnet/base58-keypairs/pubkey1.txt) \
    ${bootstrapper_args[@]} \
    --ledger /tmp/ledger-dir \
    --faucet-lamports "5000000000000"
```
with the script printing the exact array that needs to be stored in the `genesis_config.rs` file.

The docker images used by Kurtosis for Solana testnets come with these configurations pre-loaded, allowing faucet and bootstrap nodes to start the networks.

### Solana Testnets
A Solana testnet consists of a faucet node, a bootstrap node, and then the rest of the validators.
The faucet spins up first, the bootstrap spins up referencing the faucet.
