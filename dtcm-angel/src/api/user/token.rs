/// Token request
#[derive(Debug, Serialize)]
#[api(POST, Token)]
pub struct TokenReq {
    /// Refresh token
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

impl TokenReq {
    /// Returns a new instance for [`TokenReq`]
    pub fn new<R>(refresh_token: R) -> Self
    where
        R: Into<String>,
    {
        Self {
            refresh_token: refresh_token.into(),
        }
    }
}
