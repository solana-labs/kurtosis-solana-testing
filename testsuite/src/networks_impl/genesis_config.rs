pub struct GenesisKeypair {
    pub keypair_json: &'static str,
    pub pubkey: &'static str,
}

pub struct GenesisBootstrapperKeypairs {
    pub identity: GenesisKeypair,
    pub vote_account: GenesisKeypair,
    pub stake_account: GenesisKeypair,
}

// ==========================================================================================
//       NOTE: Everything below here is copied from the output of the ledger-generating
//             script inside this repo
// ==========================================================================================
pub const GENESIS_BOOTSTRAPPER_KEYPAIRS: &'static [GenesisBootstrapperKeypairs] = &[
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[254,255,155,184,101,78,168,181,71,142,36,127,63,33,110,200,212,27,204,76,99,94,188,170,153,231,27,207,216,78,112,198,182,163,13,150,208,4,44,178,92,76,174,75,59,137,146,117,185,234,165,234,206,138,185,173,12,124,18,82,13,210,209,121]",
            pubkey: "DHwQYfhEP1jrX7jptcyuZ4AFbwmwoiQ6peiL1BHFhzSk",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[70,40,246,127,90,206,9,190,121,172,243,16,139,86,11,33,104,198,70,185,198,15,159,244,115,29,171,76,149,105,87,84,108,197,232,97,38,48,64,100,88,227,164,220,181,224,220,176,199,150,145,110,189,43,142,114,229,148,90,5,102,161,53,128]",
            pubkey: "8Kc4bS96NzJu896LMrQKxqSCzTM7AtQJgyor6ZavxQhm",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[85,216,167,20,13,106,190,8,23,205,89,18,106,242,247,209,45,151,129,124,99,108,67,126,230,88,10,94,112,49,253,145,157,242,240,190,68,77,218,28,221,220,212,231,198,150,188,228,245,49,223,228,183,111,155,58,172,203,66,164,212,111,51,160]",
            pubkey: "BdZsF9CYMDPyXrMBpdMpFGhAy7pp3qMWJCS9yc3y9D6F",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[80,245,22,207,57,78,198,6,86,227,2,72,200,205,69,41,244,80,177,71,34,195,73,46,228,61,14,167,91,75,83,56,152,189,143,39,36,9,236,85,71,100,190,48,219,155,51,203,61,37,82,4,27,78,182,123,155,216,105,183,136,8,70,159]",
            pubkey: "BHEcoA7SThAdnRU9u7tmzqtPFNkSo3J16ZkNNiiCw6Bg",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[143,100,21,159,188,79,220,193,61,131,94,182,219,203,160,165,62,108,184,21,123,161,42,197,96,190,154,47,75,64,167,228,180,123,67,208,214,186,207,118,116,89,45,230,250,110,239,40,21,234,58,102,98,251,98,90,94,100,74,93,248,82,177,64]",
            pubkey: "D9XQJLKAPrrBYp8E9UCoe4Mv9qRmbeFkE1uugF9fLW4f",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[253,80,35,146,6,55,244,87,179,7,169,7,37,141,240,18,170,130,32,157,230,49,181,94,83,56,178,226,182,165,12,207,76,134,143,78,167,206,157,143,5,103,193,73,211,55,244,95,72,47,160,44,36,209,138,245,72,212,221,98,165,134,180,154]",
            pubkey: "69izSghtVjUrBAz343BJpXenWdnnZ6a8scwmMQJeHGL1",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[118,37,8,140,9,126,201,3,42,194,141,211,162,98,210,15,2,252,54,67,136,93,142,169,10,77,200,101,182,155,240,85,127,59,119,147,211,124,172,47,176,92,45,229,89,160,231,87,23,96,133,216,96,237,137,18,146,134,91,123,210,232,3,251]",
            pubkey: "9ZfNVmP1zC2AqSDsBfjKEYSjitjiABDWmTKfbbu3wze6",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[180,182,157,253,89,228,7,242,24,6,240,92,239,226,164,167,91,84,237,9,73,57,159,122,54,104,65,50,82,164,124,128,148,135,102,229,126,163,128,2,77,156,233,145,22,115,106,124,55,45,248,209,186,225,47,253,77,9,59,235,136,107,151,133]",
            pubkey: "Azo6BquStWVgzHQkyGHCeGsBJCR7gV9shAULfMuWGZnk",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[80,72,57,168,97,218,79,39,44,110,95,136,191,83,36,91,40,151,245,34,17,120,28,82,79,87,101,67,235,212,53,239,247,187,16,40,111,132,253,93,6,127,204,57,140,186,135,47,102,135,163,82,142,148,135,23,236,231,236,211,64,215,149,94]",
            pubkey: "Hg39B3QJdyA9mgrC9Gad2vySymkqBcrxH7QLQQD87th3",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[186,59,149,229,110,199,224,178,222,54,91,179,96,174,197,76,159,26,241,155,64,149,65,13,85,160,74,220,13,8,156,211,110,75,147,31,88,76,154,33,198,248,3,8,40,73,95,8,107,119,213,240,5,183,124,13,135,219,202,192,139,76,60,197]",
            pubkey: "8RYgkgZ3uRHdouWVYxw2N9ndQTZxQeYPKuGVaBWZwJfv",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[172,110,105,94,69,110,220,79,18,138,20,32,162,77,240,66,82,171,68,85,19,161,144,20,204,88,3,136,201,251,231,139,122,174,148,144,69,248,176,238,3,233,58,62,142,160,73,182,217,119,160,108,208,51,7,226,180,202,249,124,174,78,215,87]",
            pubkey: "9Fu946ojuG9jzwoBDbDwJpAHquYT9vVHJ5KYSXzf84pn",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[51,206,230,234,111,109,93,35,85,204,99,241,255,165,142,216,209,229,68,72,238,112,60,225,0,131,192,133,157,152,81,70,156,169,129,127,0,46,65,37,74,240,121,98,5,54,33,168,131,65,221,190,170,70,122,218,205,172,189,10,158,175,151,239]",
            pubkey: "BYYWjekDSy94wkkHbXKCm63prLyW1PmY23HVKc6Z33Zx",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[67,133,55,201,9,225,148,159,87,231,196,16,227,196,255,0,99,205,9,33,128,251,57,97,112,223,125,164,164,175,19,169,239,50,106,171,91,122,125,19,251,205,138,171,201,142,157,90,203,154,252,221,146,141,203,249,211,142,18,145,4,74,85,72]",
            pubkey: "H6j2eyWpfTEmkXFSBeN3PU229ogdmVd4YAes7zkUEqMh",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[65,212,185,77,227,214,44,9,185,75,103,0,221,6,203,46,205,71,118,177,140,6,235,83,178,201,53,253,139,198,28,249,174,125,139,161,208,76,20,192,88,188,203,22,171,53,211,102,86,234,187,21,133,154,231,148,7,27,223,110,68,38,201,100]",
            pubkey: "Ck8yQGB79PzYcggsK5ytaDfeR7GtRL2Gyj2SvEuUh3qD",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[141,42,131,81,151,210,4,13,189,149,238,251,116,63,30,150,248,102,67,93,84,69,148,252,182,207,231,69,88,183,235,103,70,179,37,39,203,145,251,114,191,83,187,111,13,131,181,167,187,128,21,187,68,221,80,174,64,81,40,63,208,43,133,119]",
            pubkey: "5kyybJ6wF9nHd6CdyqWuhza7aupceA1mvqg3hfrUdiZ4",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[52,232,122,49,71,142,114,27,124,248,53,128,52,7,36,147,214,188,170,12,86,148,73,249,109,104,244,54,239,22,184,240,238,167,78,125,179,212,69,145,165,206,241,52,205,211,19,196,228,122,188,180,163,75,82,225,3,20,136,217,96,194,53,62]",
            pubkey: "H4bzwDwm8gTM6tmcMdtouyjG8QC359EfAN9K4kRhr7gh",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[137,165,166,83,64,108,155,182,233,255,59,243,254,118,226,83,193,195,40,252,249,78,9,235,177,157,124,164,212,112,97,136,188,238,183,218,122,252,242,61,37,97,222,71,38,22,182,210,209,255,87,56,1,145,141,61,55,248,130,95,200,141,183,81]",
            pubkey: "DiWmhngknsVRpMNH3iQT8p6mDX4WjnJJH7X8aekkE9kU",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[183,67,100,128,63,96,178,237,197,32,38,78,73,149,104,46,180,15,189,174,79,38,98,189,175,191,239,199,232,81,169,75,133,100,68,207,242,59,75,219,64,235,190,211,36,119,229,185,173,126,252,200,87,208,71,246,100,111,52,30,163,179,221,152]",
            pubkey: "9yhuKSxBKUqknnySZu88treJMujT5aF7KkZoRfnsBS8o",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[208,212,69,134,2,140,230,43,36,40,0,87,157,176,124,177,132,222,203,80,86,156,86,62,244,226,153,238,80,247,241,176,240,62,210,211,192,163,97,33,31,66,226,183,129,254,193,230,30,90,128,71,207,107,253,141,224,95,35,165,76,36,185,220]",
            pubkey: "HApQicou2SvPM3gcnTLTCuEwEqoo1ijE2b1NzSwTCKBy",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[5,92,92,26,247,255,88,119,1,36,73,143,139,77,227,214,254,135,123,34,179,74,10,209,102,77,27,122,194,62,249,243,212,175,208,215,96,165,15,236,45,48,82,123,120,162,88,11,46,142,60,97,7,55,33,153,192,231,5,244,78,11,88,115]",
            pubkey: "FKEvcLfMy3d5UDM9fc25SHupsUEeDCAfcHNTqnH2zNQJ",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[84,246,110,66,183,203,88,191,253,180,125,145,236,202,137,34,224,43,19,119,247,188,32,99,52,204,19,84,213,69,146,43,207,140,9,57,155,213,176,106,13,21,155,49,122,187,126,197,6,10,251,13,231,104,249,246,58,124,24,62,14,255,203,13]",
            pubkey: "EyBF39F3rNSjXoFVYvehKv5JJnZAN8wsv3V9yw2bo5Jt",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[27,155,232,1,236,75,193,221,207,11,74,88,237,70,207,34,124,10,184,194,139,45,109,248,52,29,113,155,44,30,182,31,93,213,108,160,194,84,66,201,91,145,179,78,149,209,45,142,162,66,70,182,45,43,13,250,25,62,234,245,117,199,163,62]",
            pubkey: "7KHfq2cSfuG2mJsH22h4XbgAKMVqq3bRKSNn1Bnh89kh",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[92,171,133,91,226,207,200,250,224,7,27,59,83,209,218,215,178,12,7,251,29,45,168,160,83,245,162,147,182,94,42,214,54,130,253,81,112,80,46,207,253,176,193,50,33,163,186,111,98,217,148,113,82,43,104,214,16,226,134,103,166,244,241,115]",
            pubkey: "4fns5jwJde1zuUkkyUS9v7Dw3Hf7nyfeGAFcw4QegR3Q",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[234,153,253,187,161,241,53,90,227,45,29,194,166,57,159,78,132,34,131,205,179,171,242,215,107,214,214,251,176,231,143,102,114,215,187,93,135,21,241,190,88,63,161,102,49,237,47,245,169,122,13,174,138,221,110,15,157,126,169,10,247,196,84,100]",
            pubkey: "8jJGmmF7unCLVJm7EXAfeiW5hnsNW4yAy7NNBa2o1cHR",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[68,111,119,156,61,88,93,248,38,185,80,242,29,148,190,47,199,42,241,52,133,225,173,209,178,211,46,209,155,201,208,160,76,113,221,180,85,90,253,241,187,2,158,168,50,18,159,54,136,234,227,66,0,170,3,244,225,174,22,146,72,227,212,59]",
            pubkey: "69Qgwspoempz3hTZ1QG2kJQbWxswA2tc9x11FnaXRcpi",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[96,197,189,154,171,134,38,229,57,109,112,100,179,112,54,4,207,149,238,236,81,175,29,237,154,152,80,242,233,211,10,132,213,141,39,49,132,158,126,187,169,58,250,143,37,200,173,82,50,167,138,207,223,182,29,55,4,174,12,121,215,36,125,23]",
            pubkey: "FNcgDXyrU3zUPa3GzHMVsqza9JMfDXUZT3GrKwLKWm4r",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[74,62,23,115,25,148,147,197,21,86,255,162,29,77,8,140,210,174,70,241,76,109,144,242,61,177,50,187,168,211,45,86,18,193,92,61,141,107,122,70,240,160,238,76,167,211,177,55,181,43,22,183,161,199,161,73,252,92,142,32,105,105,123,220]",
            pubkey: "2GDM9fGDArcVpzeYbTnVYjZVBhPm2TbEargRbazDzxtB",
        },
    },
    GenesisBootstrapperKeypairs{
        identity: GenesisKeypair {
            keypair_json: "[56,131,73,34,60,234,92,146,14,87,121,127,43,80,98,76,249,131,59,43,213,0,148,14,14,164,129,239,199,234,85,228,95,184,39,188,163,88,166,253,60,43,203,71,224,117,128,11,122,21,240,240,231,157,164,89,208,56,55,219,126,23,237,225]",
            pubkey: "7SebkNUBQC6K4eGD3FG88X1YbC7bQqFTe9jLMf4yaVJG",
        },
        vote_account: GenesisKeypair {
            keypair_json: "[134,188,68,191,121,111,183,111,18,52,87,154,189,248,203,68,182,46,182,11,77,91,237,153,173,177,156,174,207,84,136,41,136,69,142,199,71,44,117,113,66,238,208,141,63,7,36,139,193,159,200,122,219,34,152,204,123,226,24,148,135,108,48,91]",
            pubkey: "AAwxvF4khfmD2M1xETzFWGsqVbd9dx6Zjxs8BGu4hoKY",
        },
        stake_account: GenesisKeypair {
            keypair_json: "[144,234,78,31,100,13,136,227,202,246,84,223,21,209,229,178,99,168,11,112,126,224,156,47,45,81,143,224,108,100,78,16,215,21,231,118,141,71,42,90,200,230,94,183,12,147,128,60,90,143,196,203,21,156,159,122,62,9,47,67,169,255,164,45]",
            pubkey: "FUc2aNhnGuLsPDszwjpNQTBi39YvULyf9EU1tUtP8jMe",
        },
    },
];
pub const GENESIS_HASH: &str = "5RtuMhhsNWhnZRNinguwDELgJHPwWLGaY7jrwzcwF2Ad";
pub const BANK_HASH: &str = "6gjZyEg6ssB7mL18TTAtrowZe9qyBqrqaWfzjECPneKq";
pub const SHRED_VERSION: u64 = 58880;
pub const FAUCET_KEYPAIR: GenesisKeypair = GenesisKeypair{
    keypair_json: "[65,112,183,110,74,97,57,221,129,142,59,164,52,73,236,164,129,184,227,222,254,80,8,1,135,3,124,117,161,215,206,145,195,19,249,244,157,95,101,40,200,38,140,70,30,68,86,246,43,84,203,23,120,215,191,110,127,41,27,159,164,241,45,207]",
    pubkey: "E8WAm72FzP4abty5Vn2YtDDovGnG6J2MRNAsADoLxsk2",
};