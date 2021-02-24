use anyhow::{anyhow, Result};
use super::{genesis_config::{GENESIS_BOOTSTRAPPER_KEYPAIRS, GenesisBootstrapperKeypairs}};


pub (super) struct GenesisBootstrapperKeypairsProvider {
    elems_provided: usize,
}

impl GenesisBootstrapperKeypairsProvider {
    pub fn new() -> GenesisBootstrapperKeypairsProvider {
        return GenesisBootstrapperKeypairsProvider{
            elems_provided: 0,
        }
    }

    pub fn get_genesis_bootstrapper_keypairs(&mut self) -> Result<&GenesisBootstrapperKeypairs> {
        if self.elems_provided >= GENESIS_BOOTSTRAPPER_KEYPAIRS.len() {
            return Err(anyhow!(
                "Ledger was generated with {} bootstrapper keypairs, and those have all already been provided",
                self.elems_provided,
            ));
        }
        let next_elem_idx = self.elems_provided;
        let result = &(GENESIS_BOOTSTRAPPER_KEYPAIRS[next_elem_idx]);
        self.elems_provided = self.elems_provided + 1;
        return Ok(result);
    }
}