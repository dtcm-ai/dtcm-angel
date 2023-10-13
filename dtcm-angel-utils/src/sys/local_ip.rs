use std::net::IpAddr;

use crate::Result;

/// Returns the local ip address for the system
pub fn local_ip() -> Result<IpAddr> {
    local_ip_address::local_ip().map_err(|e| {
        error!("Failed to fetch local ip address: {e}");
        e.into()
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn local_ip_works() {
        assert!(super::local_ip().is_ok());
    }
}
