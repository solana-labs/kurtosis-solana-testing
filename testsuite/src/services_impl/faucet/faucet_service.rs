use kurtosis_rust_lib::services::service::Service;

pub (super) const FAUCET_PORT: u32 = 9900;

pub struct FaucetService {
    service_id: String,
    ip_addr: String,
}

impl FaucetService {
    pub fn new(service_id: String, ip_addr: String) -> FaucetService {
        return FaucetService{
            service_id,
            ip_addr,
        };
    }

    pub fn get_port(&self) -> u32 {
        return FAUCET_PORT;
    }
}

impl Service for FaucetService {
    fn get_service_id(&self) -> &str {
        return &self.service_id;
    }

    fn get_ip_address(&self) -> &str {
        return &self.ip_addr;
    }

    fn is_available(&self) -> bool {
        // Faucet operates on UDP - no guarantees of availability
        return true;
    }
}