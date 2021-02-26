pub struct GenesisKeypair {
    pub keypair_json: &'static str,
    pub pubkey: &'static str,
}

pub struct GenesisBootstrapperKeypairs {
    pub identity: GenesisKeypair,
    pub vote_account: GenesisKeypair,
    pub stake_account: GenesisKeypair,
}

pub const FAUCET_KEYPAIR: GenesisKeypair = GenesisKeypair{
    keypair_json: "[8,239,195,145,153,173,163,233,39,61,180,167,59,141,13,79,236,79,134,138,150,57,159,237,131,89,74,97,157,23,99,177,138,99,98,180,110,233,182,244,196,23,61,14,148,202,15,81,130,66,97,44,5,185,100,126,137,70,107,25,175,114,245,104]",
    pubkey: "AKDAFBjxJ5hZ1YzDLcZsTsgs4XkoZQjacyWrSZmQobTV",
};

// These properties are found by running `RUST_LOG=none solana-ledger-tool PROPERY --ledger /the/ledger/dir` on 
// the ledger directory
// TODO Figure out why the bank hashes are different on different machines: Interestingly, downloading the 
//  ledger dir to my local machine (Mac) and getting the bank-hash returns this "Gk2Q...." value, but 
//  inside the Docker container the bank-hash of the same ledger dir is different!
// pub const BANK_HASH: &str = "Gk2QcHRMk4tumk7cp89mBFVSKBE1746zEertwrrpgvmP"; // bank-hash of ledger dir on local machine's solana-ledger-tool
pub const BANK_HASH: &str = "8XfjTvVehssGtshD1byUfnag7Wh6kdaUqVNuYMj2nfCb"; // bank-hash of ledger dir on Docker container's solana-ledger-tool
pub const GENESIS_HASH: &str = "4ywU35nDwQXGUNPkihherScMa7CMHZWCRibqgikTs1J1"; // genesis-hash
pub const SHRED_VERSION: u64 = 63878; // shred-version

// This bootstrapper keypair will have been funded in this genesis configuration
// See the README of this repo to view the exact configuration used to generate the ledger
const BOOTSTRAPPER1_IDENTITY_KEYPAIR: GenesisKeypair = GenesisKeypair{
    keypair_json: "[138,73,94,205,144,202,189,78,39,216,83,192,205,86,177,230,34,122,72,48,200,48,227,24,164,102,125,184,113,160,35,78,232,252,122,226,195,180,7,250,120,129,79,52,132,145,149,31,89,240,243,117,11,117,138,77,5,83,212,110,137,200,233,168]",
    pubkey: "GgUt4yfXYboFzithBaaAjwqtSACH9aTHnREg8Lm12XxK",
};
const BOOTSTRAPPER1_VOTE_ACCOUNT_KEYPAIR: GenesisKeypair = GenesisKeypair{
    keypair_json: "[212,20,161,123,58,72,232,147,81,27,144,71,69,192,105,14,35,78,253,241,126,233,208,13,20,52,123,180,134,231,104,95,163,162,29,250,196,96,90,240,208,253,212,188,171,219,43,216,222,231,149,55,48,20,5,188,239,177,197,150,10,243,238,143]",
    pubkey: "C1kqFbviEVdVy1UMNPP3SjzVdKEkrnPB38yGm3MWNzsY",
};
const BOOTSTRAPPER1_STAKE_ACCOUNT_KEYPAIR: GenesisKeypair = GenesisKeypair{
    keypair_json: "[98,61,131,19,100,112,23,19,71,180,15,158,53,126,145,5,207,115,185,112,191,97,98,215,74,90,219,67,105,31,244,131,84,41,25,25,130,143,182,20,63,58,155,135,162,60,11,106,226,105,185,104,136,236,60,225,214,46,120,127,13,6,22,226]",
    pubkey: "6fXbSzD2ZW7cVwPdquiFg78xqA3ZaFRVfEB6Z2Zntavu",
};
const BOOTSTRAPPER1_KEYPAIRS: GenesisBootstrapperKeypairs = GenesisBootstrapperKeypairs{
    identity: BOOTSTRAPPER1_IDENTITY_KEYPAIR,
    vote_account: BOOTSTRAPPER1_VOTE_ACCOUNT_KEYPAIR,
    stake_account: BOOTSTRAPPER1_STAKE_ACCOUNT_KEYPAIR,
};



const BOOTSTRAPPER2_IDENTITY_KEYPAIR: GenesisKeypair = GenesisKeypair{
    keypair_json: "[232,53,71,26,4,13,9,162,177,240,51,99,113,165,226,129,62,47,188,95,90,237,221,14,113,62,128,51,165,169,206,234,228,4,237,247,193,218,213,90,237,72,14,56,248,2,171,189,79,18,54,9,176,204,215,25,43,25,179,209,103,253,80,30]",
    pubkey: "GM6KG3xKurtuj6183Tb4JinQnQ3CfSRWmM3rZna8AnA9",
};
const BOOTSTRAPPER2_VOTE_ACCOUNT_KEYPAIR: GenesisKeypair = GenesisKeypair{
    keypair_json: "[99,176,126,93,245,166,74,100,254,146,35,12,188,175,159,207,235,10,152,195,24,163,243,174,62,36,1,215,76,150,221,50,134,136,248,202,255,103,105,59,38,31,71,47,142,146,252,146,101,41,43,16,244,197,229,148,196,66,113,199,221,212,138,74]",
    pubkey: "A4Amfc2fowh9Qx1itP4LqCi3JeCDoXM6JtxjHG6QF2xq",
};
const BOOTSTRAPPER2_STAKE_ACCOUNT_KEYPAIR: GenesisKeypair = GenesisKeypair{
    keypair_json: "[8,132,110,75,70,243,165,25,235,85,175,208,203,19,43,9,8,208,21,17,25,115,138,93,124,141,119,239,102,31,189,159,168,5,57,180,117,46,241,35,233,182,48,125,100,239,28,140,43,143,230,5,244,32,72,0,144,95,178,235,219,0,16,93]",
    pubkey: "CJt7eXPNKKfJkUiTYVYS237gqJMrZs76xXq8NromtJHa",
};
const BOOTSTRAPPER2_KEYPAIRS: GenesisBootstrapperKeypairs = GenesisBootstrapperKeypairs{
    identity: BOOTSTRAPPER2_IDENTITY_KEYPAIR,
    vote_account: BOOTSTRAPPER2_VOTE_ACCOUNT_KEYPAIR,
    stake_account: BOOTSTRAPPER2_STAKE_ACCOUNT_KEYPAIR,
};

pub const GENESIS_BOOTSTRAPPER_KEYPAIRS: &'static [GenesisBootstrapperKeypairs] = &[
    BOOTSTRAPPER1_KEYPAIRS,
    BOOTSTRAPPER2_KEYPAIRS,
];