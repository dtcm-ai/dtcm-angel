/// Logout request
#[derive(Debug, Serialize)]
#[api(POST, Logout)]
pub struct LogoutReq {
    /// Client code
    #[serde(rename = "clientcode")]
    pub client_code: String,
}

impl LogoutReq {
    /// Returns a new instance for [`LogoutReq`]
    pub fn new<C>(client_code: C) -> Self
    where
        C: Into<String>,
    {
        Self {
            client_code: client_code.into(),
        }
    }
}
