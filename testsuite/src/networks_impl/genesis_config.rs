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


// ==========================================================================================
//                                 5-node config
// ==========================================================================================
pub const BANK_HASH: &str = "GAV6uMXpye5SnyD6X5twPy8rnFWuMjSw2pkV7c2yCHJr"; // bank-hash of ledger dir on Docker container's solana-ledger-tool
pub const GENESIS_HASH: &str = "GRhKUMbxkR6KraW3MVXX1DY4B4k2JAfVRihcmUg2y53r"; // genesis-hash
pub const SHRED_VERSION: u64 = 54373; // shred-version
pub const GENESIS_BOOTSTRAPPER_KEYPAIRS: &'static [GenesisBootstrapperKeypairs] = &[
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[138,73,94,205,144,202,189,78,39,216,83,192,205,86,177,230,34,122,72,48,200,48,227,24,164,102,125,184,113,160,35,78,232,252,122,226,195,180,7,250,120,129,79,52,132,145,149,31,89,240,243,117,11,117,138,77,5,83,212,110,137,200,233,168]",
            pubkey: "GgUt4yfXYboFzithBaaAjwqtSACH9aTHnREg8Lm12XxK",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[212,20,161,123,58,72,232,147,81,27,144,71,69,192,105,14,35,78,253,241,126,233,208,13,20,52,123,180,134,231,104,95,163,162,29,250,196,96,90,240,208,253,212,188,171,219,43,216,222,231,149,55,48,20,5,188,239,177,197,150,10,243,238,143]",
            pubkey: "C1kqFbviEVdVy1UMNPP3SjzVdKEkrnPB38yGm3MWNzsY",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[98,61,131,19,100,112,23,19,71,180,15,158,53,126,145,5,207,115,185,112,191,97,98,215,74,90,219,67,105,31,244,131,84,41,25,25,130,143,182,20,63,58,155,135,162,60,11,106,226,105,185,104,136,236,60,225,214,46,120,127,13,6,22,226]",
            pubkey: "6fXbSzD2ZW7cVwPdquiFg78xqA3ZaFRVfEB6Z2Zntavu",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[232,53,71,26,4,13,9,162,177,240,51,99,113,165,226,129,62,47,188,95,90,237,221,14,113,62,128,51,165,169,206,234,228,4,237,247,193,218,213,90,237,72,14,56,248,2,171,189,79,18,54,9,176,204,215,25,43,25,179,209,103,253,80,30]",
            pubkey: "GM6KG3xKurtuj6183Tb4JinQnQ3CfSRWmM3rZna8AnA9",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[99,176,126,93,245,166,74,100,254,146,35,12,188,175,159,207,235,10,152,195,24,163,243,174,62,36,1,215,76,150,221,50,134,136,248,202,255,103,105,59,38,31,71,47,142,146,252,146,101,41,43,16,244,197,229,148,196,66,113,199,221,212,138,74]",
            pubkey: "A4Amfc2fowh9Qx1itP4LqCi3JeCDoXM6JtxjHG6QF2xq",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[8,132,110,75,70,243,165,25,235,85,175,208,203,19,43,9,8,208,21,17,25,115,138,93,124,141,119,239,102,31,189,159,168,5,57,180,117,46,241,35,233,182,48,125,100,239,28,140,43,143,230,5,244,32,72,0,144,95,178,235,219,0,16,93]",
            pubkey: "CJt7eXPNKKfJkUiTYVYS237gqJMrZs76xXq8NromtJHa",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[193,22,185,9,149,71,19,2,200,114,250,57,108,220,137,1,204,184,210,127,238,9,79,241,140,224,220,192,38,126,113,11,240,53,140,56,143,244,21,253,145,170,27,126,183,173,211,96,163,16,27,119,111,94,182,228,238,2,63,175,67,43,173,164]",
            pubkey: "HAgCuoyNvCNGGeLFGmYWztdL3GzeZH6NuzRRK6PPdu6s",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[158,222,199,19,32,106,101,171,42,122,210,197,55,27,176,242,214,30,231,91,23,117,141,142,111,226,130,72,47,219,83,5,211,233,242,145,143,212,185,208,232,62,251,33,135,23,23,223,74,92,51,150,209,121,63,234,162,24,244,227,247,72,171,68]",
            pubkey: "FGDvrrQSnij8eLGKAxEuCYbZLHngR8r7MaoLtDy8BGMZ",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[247,27,131,238,21,59,233,213,232,92,21,213,46,215,154,122,219,205,128,255,105,172,128,171,82,1,124,241,242,102,161,55,184,135,49,28,91,154,48,8,234,123,56,65,85,48,223,146,252,209,59,231,211,174,108,28,46,242,101,223,55,121,190,246]",
            pubkey: "DRKagbX9PQchU9FKdzDMctobGS6wxYYrmYCVihVBpYtM",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[17,124,106,106,172,112,223,25,81,217,39,47,121,122,56,192,125,121,112,252,125,54,188,154,174,49,74,1,140,170,210,67,156,239,251,15,0,183,34,51,216,198,192,8,26,85,239,230,219,93,214,165,237,162,180,139,152,32,82,43,52,114,134,34]",
            pubkey: "BZcqn7UcwnB4xot5o2UBSVDJhEC1gaoAv91MiubV5pZf",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[245,197,224,19,219,8,141,186,32,52,126,40,55,140,250,86,109,136,181,205,230,173,91,88,103,253,226,162,123,173,223,193,152,11,24,92,199,135,202,253,113,237,118,42,18,189,144,242,173,115,209,209,156,253,158,64,96,201,102,179,54,93,96,232]",
            pubkey: "BEWnPwMmXPrVkWa3i449tLWxwSTygjivRqVjtfj9a2g7",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[198,184,202,201,73,156,55,174,42,29,166,18,97,212,28,46,70,45,157,120,16,137,183,82,1,43,236,26,172,151,223,80,222,88,97,224,94,72,90,230,30,112,227,79,236,231,65,92,57,142,48,161,130,211,227,203,103,81,34,0,249,97,104,119]",
            pubkey: "Fxwg8drhBgXJ1yMtP4gge1kujzuZYAig65dqfPS35cMc",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[28,183,103,78,236,65,146,217,37,217,141,117,42,129,82,53,255,174,224,129,171,197,174,26,10,16,181,190,196,37,162,216,82,211,83,97,31,231,79,243,206,123,105,3,139,47,2,154,32,173,172,176,208,183,186,34,50,60,140,194,240,248,34,30]",
            pubkey: "6aKL4sd9V6rzifSFraMEGApyzSTnMwNhKVQ2qmF1EAYD",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[177,242,2,78,184,70,194,218,231,165,213,67,69,103,91,185,69,96,195,187,204,239,0,7,255,171,211,173,188,120,28,202,109,79,43,212,212,133,85,61,18,156,6,171,17,72,40,151,217,180,161,8,30,12,122,53,15,255,81,243,243,136,51,214]",
            pubkey: "8MhTbL9SevgMBfHHsNq8pyayrGoShquTcPiULCNMw1DF",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[65,114,0,225,109,96,203,55,113,86,46,192,93,78,180,215,98,153,71,230,43,248,240,10,69,0,64,245,76,116,52,81,177,8,28,141,244,170,197,168,230,15,248,142,132,31,5,141,171,73,62,113,250,40,208,12,157,54,47,106,144,126,196,46]",
            pubkey: "Cv4LWd5JS1Z3AyMaMiSPxfJhiVVB8rXrVMF8ufrj8P6V",
        },
    },
];


// ==========================================================================================
//                                 10-node config
// ==========================================================================================
/*
// These properties are found by running `RUST_LOG=none solana-ledger-tool PROPERY --ledger /the/ledger/dir` on 
// the ledger directory
// TODO Figure out why the bank hashes are different on different machines: Interestingly, downloading the 
//  ledger dir to my local machine (Mac) and getting the bank-hash returns this "Gk2Q...." value, but 
//  inside the Docker container the bank-hash of the same ledger dir is different!
pub const BANK_HASH: &str = "7YaBG4RF8zuzNZgbRkAo39ws6GhE9ejE9huKVg6hMvF1"; // bank-hash of ledger dir on Docker container's solana-ledger-tool
pub const GENESIS_HASH: &str = "DXh38V6yHjkvKnf6MTnguD8HrzrGYhuKLUU5o3M3LRrw"; // genesis-hash
pub const SHRED_VERSION: u64 = 6967; // shred-version

// This array was auto-generated by the script that 
// This bootstrapper keypair will have been funded in this genesis configuration
// See the README of this repo to view the exact configuration used to generate the ledger
pub const GENESIS_BOOTSTRAPPER_KEYPAIRS: &'static [GenesisBootstrapperKeypairs] = &[
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[138,73,94,205,144,202,189,78,39,216,83,192,205,86,177,230,34,122,72,48,200,48,227,24,164,102,125,184,113,160,35,78,232,252,122,226,195,180,7,250,120,129,79,52,132,145,149,31,89,240,243,117,11,117,138,77,5,83,212,110,137,200,233,168]",
            pubkey: "GgUt4yfXYboFzithBaaAjwqtSACH9aTHnREg8Lm12XxK",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[212,20,161,123,58,72,232,147,81,27,144,71,69,192,105,14,35,78,253,241,126,233,208,13,20,52,123,180,134,231,104,95,163,162,29,250,196,96,90,240,208,253,212,188,171,219,43,216,222,231,149,55,48,20,5,188,239,177,197,150,10,243,238,143]",
            pubkey: "C1kqFbviEVdVy1UMNPP3SjzVdKEkrnPB38yGm3MWNzsY",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[98,61,131,19,100,112,23,19,71,180,15,158,53,126,145,5,207,115,185,112,191,97,98,215,74,90,219,67,105,31,244,131,84,41,25,25,130,143,182,20,63,58,155,135,162,60,11,106,226,105,185,104,136,236,60,225,214,46,120,127,13,6,22,226]",
            pubkey: "6fXbSzD2ZW7cVwPdquiFg78xqA3ZaFRVfEB6Z2Zntavu",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[232,53,71,26,4,13,9,162,177,240,51,99,113,165,226,129,62,47,188,95,90,237,221,14,113,62,128,51,165,169,206,234,228,4,237,247,193,218,213,90,237,72,14,56,248,2,171,189,79,18,54,9,176,204,215,25,43,25,179,209,103,253,80,30]",
            pubkey: "GM6KG3xKurtuj6183Tb4JinQnQ3CfSRWmM3rZna8AnA9",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[99,176,126,93,245,166,74,100,254,146,35,12,188,175,159,207,235,10,152,195,24,163,243,174,62,36,1,215,76,150,221,50,134,136,248,202,255,103,105,59,38,31,71,47,142,146,252,146,101,41,43,16,244,197,229,148,196,66,113,199,221,212,138,74]",
            pubkey: "A4Amfc2fowh9Qx1itP4LqCi3JeCDoXM6JtxjHG6QF2xq",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[8,132,110,75,70,243,165,25,235,85,175,208,203,19,43,9,8,208,21,17,25,115,138,93,124,141,119,239,102,31,189,159,168,5,57,180,117,46,241,35,233,182,48,125,100,239,28,140,43,143,230,5,244,32,72,0,144,95,178,235,219,0,16,93]",
            pubkey: "CJt7eXPNKKfJkUiTYVYS237gqJMrZs76xXq8NromtJHa",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[193,22,185,9,149,71,19,2,200,114,250,57,108,220,137,1,204,184,210,127,238,9,79,241,140,224,220,192,38,126,113,11,240,53,140,56,143,244,21,253,145,170,27,126,183,173,211,96,163,16,27,119,111,94,182,228,238,2,63,175,67,43,173,164]",
            pubkey: "HAgCuoyNvCNGGeLFGmYWztdL3GzeZH6NuzRRK6PPdu6s",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[158,222,199,19,32,106,101,171,42,122,210,197,55,27,176,242,214,30,231,91,23,117,141,142,111,226,130,72,47,219,83,5,211,233,242,145,143,212,185,208,232,62,251,33,135,23,23,223,74,92,51,150,209,121,63,234,162,24,244,227,247,72,171,68]",
            pubkey: "FGDvrrQSnij8eLGKAxEuCYbZLHngR8r7MaoLtDy8BGMZ",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[247,27,131,238,21,59,233,213,232,92,21,213,46,215,154,122,219,205,128,255,105,172,128,171,82,1,124,241,242,102,161,55,184,135,49,28,91,154,48,8,234,123,56,65,85,48,223,146,252,209,59,231,211,174,108,28,46,242,101,223,55,121,190,246]",
            pubkey: "DRKagbX9PQchU9FKdzDMctobGS6wxYYrmYCVihVBpYtM",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[17,124,106,106,172,112,223,25,81,217,39,47,121,122,56,192,125,121,112,252,125,54,188,154,174,49,74,1,140,170,210,67,156,239,251,15,0,183,34,51,216,198,192,8,26,85,239,230,219,93,214,165,237,162,180,139,152,32,82,43,52,114,134,34]",
            pubkey: "BZcqn7UcwnB4xot5o2UBSVDJhEC1gaoAv91MiubV5pZf",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[245,197,224,19,219,8,141,186,32,52,126,40,55,140,250,86,109,136,181,205,230,173,91,88,103,253,226,162,123,173,223,193,152,11,24,92,199,135,202,253,113,237,118,42,18,189,144,242,173,115,209,209,156,253,158,64,96,201,102,179,54,93,96,232]",
            pubkey: "BEWnPwMmXPrVkWa3i449tLWxwSTygjivRqVjtfj9a2g7",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[198,184,202,201,73,156,55,174,42,29,166,18,97,212,28,46,70,45,157,120,16,137,183,82,1,43,236,26,172,151,223,80,222,88,97,224,94,72,90,230,30,112,227,79,236,231,65,92,57,142,48,161,130,211,227,203,103,81,34,0,249,97,104,119]",
            pubkey: "Fxwg8drhBgXJ1yMtP4gge1kujzuZYAig65dqfPS35cMc",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[28,183,103,78,236,65,146,217,37,217,141,117,42,129,82,53,255,174,224,129,171,197,174,26,10,16,181,190,196,37,162,216,82,211,83,97,31,231,79,243,206,123,105,3,139,47,2,154,32,173,172,176,208,183,186,34,50,60,140,194,240,248,34,30]",
            pubkey: "6aKL4sd9V6rzifSFraMEGApyzSTnMwNhKVQ2qmF1EAYD",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[177,242,2,78,184,70,194,218,231,165,213,67,69,103,91,185,69,96,195,187,204,239,0,7,255,171,211,173,188,120,28,202,109,79,43,212,212,133,85,61,18,156,6,171,17,72,40,151,217,180,161,8,30,12,122,53,15,255,81,243,243,136,51,214]",
            pubkey: "8MhTbL9SevgMBfHHsNq8pyayrGoShquTcPiULCNMw1DF",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[65,114,0,225,109,96,203,55,113,86,46,192,93,78,180,215,98,153,71,230,43,248,240,10,69,0,64,245,76,116,52,81,177,8,28,141,244,170,197,168,230,15,248,142,132,31,5,141,171,73,62,113,250,40,208,12,157,54,47,106,144,126,196,46]",
            pubkey: "Cv4LWd5JS1Z3AyMaMiSPxfJhiVVB8rXrVMF8ufrj8P6V",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[117,66,137,66,183,22,206,162,188,209,16,26,59,252,187,83,102,155,10,193,199,12,14,23,73,155,222,109,158,247,150,33,170,128,47,185,33,157,206,52,45,116,170,87,241,174,149,28,110,250,187,22,50,83,203,208,125,225,54,33,144,59,70,58]",
            pubkey: "CUZgHZkLn96FmgAcgQECGPMXkgeuJk5JopUWY3roaqpm",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[97,76,192,55,204,113,213,27,118,48,34,148,101,198,186,104,50,170,234,217,46,54,65,139,45,68,53,36,162,70,81,235,20,207,150,74,58,1,95,170,11,74,150,132,138,51,74,188,18,78,141,77,206,81,181,51,161,63,32,46,104,22,184,67]",
            pubkey: "2QEkDDrTfmQV5yc6wb2wcJSXZFwmKJXmhuadPHd9x6up",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[31,222,203,43,114,175,216,38,67,127,113,214,24,233,20,195,150,130,219,185,229,248,193,135,22,31,243,14,90,195,164,145,82,106,224,26,229,150,216,188,10,171,105,177,61,162,211,15,0,214,158,68,83,13,204,250,82,53,188,199,225,101,208,188]",
            pubkey: "6YixEn3h7Bzzd5ASnu3qFbe6FD41TKe8hHBVecsydAg3",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[53,110,26,141,36,126,126,145,36,110,128,89,246,62,22,211,152,9,17,216,154,238,39,167,38,155,112,208,137,161,180,189,187,76,169,45,209,58,78,238,217,7,208,197,223,1,200,155,105,22,239,220,180,250,89,93,72,30,10,14,158,231,223,192]",
            pubkey: "Dc93FHn8CN2QqS9xNZXk3TprsgiUzCiTnUbQR5PmrXQj",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[230,51,131,47,16,232,244,88,196,180,144,5,152,99,157,162,239,6,83,29,163,133,248,19,168,82,211,4,195,149,157,7,246,73,1,94,143,13,5,111,149,68,112,36,175,221,170,60,23,45,81,96,61,3,241,229,67,206,191,206,160,144,235,59]",
            pubkey: "HaPrssavmKaXocqgZYCLiQVH3wgsmM8MqaZhfLb96Zo8",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[210,162,214,171,161,164,15,176,195,177,218,108,173,80,192,254,94,112,22,94,41,181,234,147,158,92,104,161,140,118,106,84,192,40,156,165,77,236,247,43,221,48,180,118,129,225,196,228,193,71,0,135,205,223,254,136,248,86,197,31,231,119,109,47]",
            pubkey: "Dw7CL7oFH93Qafez7nWDnf81o77YS8ZSwLYorEcsR3iW",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[71,18,38,69,218,25,76,137,64,39,112,120,107,226,111,223,247,237,83,222,84,239,40,78,246,183,169,159,177,235,190,100,23,163,157,75,210,41,211,138,98,130,213,203,68,66,56,113,83,206,91,203,87,133,253,159,58,35,191,78,102,2,86,210]",
            pubkey: "2bH5Z2mYh9oau28QqTANhVMbfkxvKzUf3SeJeP8A5c2V",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[209,127,245,187,5,224,18,24,144,224,191,196,225,184,240,240,48,186,117,185,139,151,73,50,222,112,118,95,250,148,68,4,30,28,116,149,48,195,46,173,68,229,149,90,9,188,245,107,94,166,86,95,88,104,194,224,240,181,22,49,64,109,110,69]",
            pubkey: "32YQ2d8KN1BukBCknyzsjiRfz235qLwk4aAhv4caKk6Q",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[94,254,178,78,158,5,30,5,106,153,224,203,202,222,22,171,0,39,57,130,226,175,46,236,251,102,137,41,89,192,147,203,116,28,95,159,49,34,38,82,48,212,36,50,61,183,220,185,68,79,146,123,83,65,18,255,107,200,241,158,230,155,178,179]",
            pubkey: "8pFPRci9B2SQMnL9B2hvtHkB91pVzY6DwB2kznijoCXk",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[11,188,179,185,11,45,17,22,44,145,203,251,88,193,169,26,144,247,50,176,200,91,138,252,35,139,233,82,68,14,129,23,95,233,156,69,122,18,128,38,150,150,74,3,55,127,52,126,100,237,253,1,174,240,244,202,44,64,59,213,206,125,118,140]",
            pubkey: "7TQLaVBhTexmMk7788t11irTbWZrqKPESiwr77DbvWA7",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[210,109,225,29,167,108,132,90,82,0,180,111,235,141,134,127,242,162,200,126,126,134,141,175,221,132,241,158,11,99,153,255,61,237,132,116,224,100,188,48,136,17,37,135,148,247,32,56,1,122,139,215,40,120,91,128,244,30,141,212,14,173,52,209]",
            pubkey: "5Ajw1SAsFArFCReYtF8VhpUdRDJAvbWx1HHn8QLthH5S",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[143,77,116,35,231,13,150,168,190,0,204,20,164,99,88,81,201,162,186,206,179,61,246,36,170,225,207,133,199,61,242,155,39,213,193,246,197,156,82,206,93,203,255,57,139,145,247,30,0,51,74,115,132,226,97,186,84,154,158,115,228,187,134,119]",
            pubkey: "3gVx22ZoGFbHprGH4WKJfxjVqcYutrNNmXpozJZEmK3C",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair{
            keypair_json: "[2,241,153,81,112,3,207,211,108,203,222,78,221,63,171,253,231,201,17,81,210,68,157,40,113,72,252,135,137,14,72,28,3,123,38,149,74,193,108,186,234,93,192,210,167,59,53,15,208,180,199,128,114,243,41,24,1,95,139,90,151,49,216,237]",
            pubkey: "Eb91wWFURDVM3VUeozaFNY5eQKoHPcxX29zLVc2sndr",
        },
        vote_account: GenesisKeypair{
            keypair_json: "[134,62,206,53,75,95,190,64,64,132,216,183,32,125,220,100,235,251,169,229,90,148,153,2,117,8,211,58,4,155,110,207,117,230,75,255,60,115,98,20,172,146,55,218,107,143,78,130,170,40,213,225,174,48,169,67,102,204,196,215,179,76,177,106]",
            pubkey: "8wENq3guXmAt1UvcSDxWLNNtPNdrJvC3SLWBPjwhVNiD",
        },
        stake_account: GenesisKeypair{
            keypair_json: "[175,124,25,42,106,140,36,209,236,175,250,178,93,182,122,22,25,203,204,39,109,210,47,196,115,207,251,251,248,140,132,113,28,119,98,194,93,126,8,240,89,120,168,35,62,247,155,28,167,121,112,233,75,196,114,145,125,211,145,186,212,30,130,176]",
            pubkey: "2v8147Su5z1D85UrrVVrAeD78auNLbSN2vS8ZJ9nt83y",
        },
    },
];
*/