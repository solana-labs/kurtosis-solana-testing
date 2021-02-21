extern crate rand;
extern crate ed25519_dalek;

use std::borrow::BorrowMut;

use anyhow::{Context, Result};
use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, KEYPAIR_LENGTH};

pub (super) struct Ed25519KeypairJsonProvider {}

impl Ed25519KeypairJsonProvider {
    pub fn new() -> Ed25519KeypairJsonProvider {
        return Ed25519KeypairJsonProvider{};
    }

    pub fn provide_keypair_json(&mut self) -> Result<String> {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(csprng.borrow_mut());
        let keypair_bytes: [u8; KEYPAIR_LENGTH] = keypair.to_bytes();
        let keypair_json = serde_json::to_string(&keypair_bytes.to_vec())
            .context("An error occurred serializing the ED25519 keypair bytes to JSON")?;
        debug!("Keypair JSON: {}", keypair_json);
        return Ok(keypair_json);
    }
}