use std::net::{AddrParseError, IpAddr};

use crate::Result;

const URL: &str = "https://myexternalip.com/raw";

/// Returns the public ip address by connecting to https://myexternalip.com/raw
pub async fn public_ip() -> Result<IpAddr> {
    reqwest::get(URL)
        .await?
        .text()
        .await?
        .parse()
        .map_err(|e: AddrParseError| {
            error!("Failed to get public IP: {:?}", e);
            e.into()
        })
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn public_ip_works() {
        assert!(super::public_ip().await.is_ok())
    }
}
