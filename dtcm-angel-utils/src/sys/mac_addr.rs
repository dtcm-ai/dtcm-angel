use crate::Result;

/// Returns the mac address for the system
pub fn mac_addr() -> Result<mac_address::MacAddress> {
    mac_address::get_mac_address()?.ok_or({
        let msg = "Failed to find mac address";
        error!("{msg}");
        msg.into()
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn mac_addr_works() {
        assert!(super::mac_addr().is_ok());
    }
}
