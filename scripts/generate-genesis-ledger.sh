# Generates a new genesis configuration with the given numbe of nodes
set -euo pipefail
script_dirpath="$(cd "$(dirname "${0}")" && pwd)"

# ====================================================================================================
#                                                Consts
# ====================================================================================================
GENESIS_BZ2_FILENAME="genesis.tar.bz2"
LEDGER_TGZ_FILENAME="test-ledger.tgz"

# ====================================================================================================
#                                         Arg-parsing & validating
# ====================================================================================================
if [ "${#}" -ne 2 ]; then
    echo "Usage: $(basename "${0}") /path/to/directory/containing/solana/clis num_genesis_bootstrappers"
    exit 1
fi

solana_clis_dirpath="${1}"
num_nodes="${2}"

if ! [ -d "${solana_clis_dirpath}" ]; then
    echo "Error: No directory '${solana_clis_dirpath}'" >&2
    exit 1
fi
if [ "${num_nodes}" -le 0 ]; then
    echo "Error: Number of genesis bootstrappers must be > 0" >&2
    exit 1
fi


# ====================================================================================================
#                                              Main code
# ====================================================================================================
ledger_dirpath="$(mktemp -d)"
if [ -d "${ledger_dirpath}" ]; then
    rm -rf "${ledger_dirpath}"
fi
solana_keygen_filepath="${solana_clis_dirpath}/solana-keygen"
bootstrapper_args=()
next_key_index=2
echo "vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv COPY TO genesis_config.rs vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"
echo "pub const GENESIS_BOOTSTRAPPER_KEYPAIRS: &'static [GenesisBootstrapperKeypairs] = &["
for i in $(seq 1 "${num_nodes}"); do
    bootstrapper_arg="-b "
    echo "    GenesisBootstrapperKeypairs{"
    for keypair_type in identity vote_account stake_account; do
        keypair_filepath="$(mktemp)"
        "${solana_keygen_filepath}" new --no-passphrase -fso "${keypair_filepath}" > /dev/null
        keypair="$(cat "${keypair_filepath}")"
        pubkey="$("${solana_keygen_filepath}" pubkey "${keypair_filepath}")"

        echo "        ${keypair_type}: GenesisKeypair {"
        echo "            keypair_json: \"${keypair}\","
        echo "            pubkey: \"${pubkey}\","
        echo "        },"
        bootstrapper_arg="${bootstrapper_arg} ${pubkey}"
    done
    echo "    },"

    bootstrapper_args+=("${bootstrapper_arg}")
done
echo "];"

faucet_keypair_filepath="$(mktemp)"
"${solana_keygen_filepath}" new --no-passphrase -fso "${faucet_keypair_filepath}" > /dev/null
faucet_keypair="$(cat "${faucet_keypair_filepath}")"
faucet_pubkey="$("${solana_keygen_filepath}" pubkey "${faucet_keypair_filepath}")"

# WARNING: Do NOT use --enable-warmup-epochs here!! If it's used, spurious failures will be thrown while under network partition
"${solana_clis_dirpath}/solana-genesis" \
    --cluster-type testnet \
    `# Tells the validators to sleep, rather than hash, in order to form the logical clock` \
    --hashes-per-tick sleep \
    --faucet-pubkey "${faucet_pubkey}" \
    ${bootstrapper_args[@]} \
    --ledger "${ledger_dirpath}" \
    --faucet-lamports "5000000000000" \
    > /dev/null

echo "pub const GENESIS_HASH: &str = \"$(RUST_LOG=none "${solana_clis_dirpath}/solana-ledger-tool" genesis-hash -l "${ledger_dirpath}")\";"
echo "pub const BANK_HASH: &str = \"$(RUST_LOG=none "${solana_clis_dirpath}/solana-ledger-tool" bank-hash -l "${ledger_dirpath}")\";"
echo "pub const SHRED_VERSION: u64 = $(RUST_LOG=none "${solana_clis_dirpath}/solana-ledger-tool" shred-version -l "${ledger_dirpath}");"
cat << EOF
pub const FAUCET_KEYPAIR: GenesisKeypair = GenesisKeypair{
    keypair_json: "${faucet_keypair}",
    pubkey: "${faucet_pubkey}",
};
EOF
echo "^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ COPY TO genesis_config.rs ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
echo ""

(
    cd "${ledger_dirpath}"
    # TODO Once Kurtosis supports unzipping bz2 artifacts, we won't need to remove the bz2 and re-tar it - just use the generated one!
    rm "${GENESIS_BZ2_FILENAME}"
    tar -czvf "${LEDGER_TGZ_FILENAME}" * > /dev/null
)

echo ""
echo "Ledger generated successfully!"
echo "ACTION NEEDED: Copy-paste the outputted Rust code above into genesis_config.rs"
echo "ACTION NEEDED: Upload the TGZ at '${ledger_dirpath}/${LEDGER_TGZ_FILENAME}' to your file host for Kurtosis to use"
