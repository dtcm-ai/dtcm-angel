use crate::Result;

/// Placeholder for response received from API calls
#[derive(Debug, Deserialize)]
pub struct Response<T> {
    /// Status for the API call
    pub status: bool,
    /// Message returned by the API
    pub message: String,
    /// Error code in case of any error
    #[serde(rename = "errorcode")]
    pub error_code: String,
    /// Data returned by the API
    pub data: Option<T>,
}

impl<T> Response<T> {
    /// Returns data from response
    pub fn into_data(self) -> Result<T> {
        self.data.ok_or("Missing data in API response".into())
    }
}

impl<T> Response<Vec<T>> {
    /// Returns vector from response
    pub fn into_vec(self) -> Vec<T> {
        self.data.unwrap_or(vec![])
    }
}
