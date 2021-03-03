extern crate rand;
extern crate ed25519_dalek;

use std::borrow::BorrowMut;

use anyhow::{Context, Result};
use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, KEYPAIR_LENGTH};

pub (super) struct GenerativeEd25519KeypairJsonProvider {}

impl GenerativeEd25519KeypairJsonProvider {
    pub fn new() -> GenerativeEd25519KeypairJsonProvider {
        return GenerativeEd25519KeypairJsonProvider{};
    }

    fn generate_keypair_json(&mut self) -> Result<String> {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(csprng.borrow_mut());
        let keypair_bytes: [u8; KEYPAIR_LENGTH] = keypair.to_bytes();
        let keypair_json = serde_json::to_string(&keypair_bytes.to_vec())
            .context("An error occurred serializing the ED25519 keypair bytes to JSON")?;
        debug!("Keypair JSON: {}", keypair_json);
        return Ok(keypair_json);
    }
}