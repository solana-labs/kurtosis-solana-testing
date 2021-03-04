use kurtosis_rust_lib::services::{service::Service, service_context::ServiceContext};

pub (super) const FAUCET_PORT: u32 = 9900;

pub struct FaucetService {
    service_context: ServiceContext,
}

impl FaucetService {
    pub fn new(service_context: ServiceContext) -> FaucetService {
        return FaucetService{
            service_context,
        };
    }

    pub fn get_ip_address(&self) -> &str {
        return self.service_context.get_ip_address();
    }

    pub fn get_port(&self) -> u32 {
        return FAUCET_PORT;
    }
}

impl Service for FaucetService {
    fn is_available(&self) -> bool {
        // Faucet operates on UDP - no guarantees of availability
        return true;
    }
}