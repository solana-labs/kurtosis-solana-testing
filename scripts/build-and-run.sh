set -euo pipefail
script_dirpath="$(cd "$(dirname "${0}")" && pwd)"
root_dirpath="$(dirname "${script_dirpath}")"
kurtosis_core_dirpath="${root_dirpath}/.kurtosis"

show_help_and_exit() {
    echo ""
    echo "Usage: $(basename "${0}") action [kurtosis.sh_arg1] [kurtosis.sh_arg2]..."
    echo ""
    echo "  action              The action that should be passed to the underlying build-and-run-core.sh script to tell it which action should be taken (call"
    echo "                          'bash ${kurtosis_core_dirpath}/build-and-run-core.sh help' directly for all available actions)"
    echo "  kurtosis.sh_args    Optional, supplemental args that should be passed to the kurtosis.sh script to modify testsuite execution behaviour (call"
    echo "                          'bash ${kurtosis_core_dirpath}/kurtosis.sh --help' directly for all available args)"
    echo ""
    exit 1  # Exit with error so CI will fail if it accidentally calls this
}

if [ "${#}" -eq 0 ]; then
    show_help_and_exit
fi
action="${1:-}"
shift 1
if [ "${action}" == "help" ]; then
    show_help_and_exit
fi

# >>>>>>>> Add custom testsuite parameters here <<<<<<<<<<<<<
custom_params_json='{
    "normalImage" :"solanalabs/solana:v1.5.10"
}'
# >>>>>>>> Add custom testsuite parameters here <<<<<<<<<<<<<

# TODO Use https://github.com/emk/rust-musl-builder/issues/114 to cache crates.io index
bash "${kurtosis_core_dirpath}/build-and-run-core.sh" \
    "${action}" \
    "solana-testsuite" \
    "${root_dirpath}" \
    "${root_dirpath}/testsuite/Dockerfile" \
    "${kurtosis_core_dirpath}/kurtosis.sh" \
    --custom-params "${custom_params_json}" \
    ${1+"${@}"}
