/// User profile
#[api(GET, UserProfile)]
#[derive(Debug, Deserialize)]
pub struct Profile {
    /// Client code
    #[serde(rename = "clientcode")]
    pub client_code: String,
    /// Name
    pub name: String,
    /// Email
    pub email: String,
    /// Mobile number
    #[serde(rename = "mobileno")]
    pub mobile_no: String,
    /// Exchanges available
    pub exchanges: Vec<String>,
    /// Products available
    pub products: Vec<String>,
    /// Last login time
    #[serde(rename = "lastlogintime")]
    pub last_login_time: String,
    /// Broker ID
    #[serde(rename = "brokerid")]
    pub broker_id: Option<String>,
}
