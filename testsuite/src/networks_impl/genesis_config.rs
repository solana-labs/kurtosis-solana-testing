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
//       NOTE: Everything below here is copied from the output of the ledger-generating 
//             script in this repo's README
// ==========================================================================================
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
pub const GENESIS_HASH: &str = "Dz1NG3GPwbQrjcmQ1jvDRho2GxeQr88HNJNT1z3d5ZTq";
pub const BANK_HASH: &str = "2uubeSeCGNmavydYLTBhJCT5oKHP5C8V6SWLK6PAAAJM";
pub const SHRED_VERSION: u64 = 38381;