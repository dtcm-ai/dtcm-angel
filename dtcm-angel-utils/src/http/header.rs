use std::fmt::Display;

use reqwest::header::{self, HeaderMap};

use crate::{
    sys::{local_ip, mac_addr, public_ip},
    Result,
};

/// Http headers to be sent to the API
pub struct HttpHeader(HeaderMap);

impl HttpHeader {
    /// Returns a new instance for the http headers
    pub async fn new<A, J>(api_key: A, jwt_token: Option<J>) -> Result<Self>
    where
        A: AsRef<str>,
        J: Display,
    {
        let mut headers = HeaderMap::new();

        let public_ip = public_ip().await?.to_string();
        let mac_addr = mac_addr()?.to_string();
        let api_key = api_key.as_ref();

        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("X-ClientLocalIP", local_ip()?.to_string().parse().unwrap());
        headers.insert("X-ClientPublicIP", public_ip.parse().unwrap());
        headers.insert("X-MACAddress", mac_addr.parse().unwrap());
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());
        headers.insert("X-UserType", "USER".parse().unwrap());
        headers.insert("X-SourceID", "WEB".parse().unwrap());

        headers.insert("X-PrivateKey", api_key.parse().unwrap());

        if let Some(token) = jwt_token {
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {token}").parse().unwrap(),
            );
        }

        Ok(Self(headers))
    }

    /// Returns the inner HeaderMap
    pub fn into_inner(self) -> HeaderMap {
        self.0
    }
}
