use dtcm_angel_utils::{
    http::{EndPoint, HttpClient},
    Result,
};

use super::OrderBook;

/// Retrieve the status of individual orders using the "uniqueorderid"
/// received in the response when placing, modifying, or canceling orders.
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
pub struct IndividualOrderStatus {
    #[serde(flatten)]
    pub order: OrderBook,
    #[serde(rename = "uniqueorderid")]
    pub unique_order_id: String,
}

impl IndividualOrderStatus {
    /// Fetches the order status for the given order ID
    pub async fn fetch_data<O>(http: &HttpClient, unique_order_id: O) -> Result<Self>
    where
        O: Into<String>,
    {
        let ep = EndPoint::IndividualOrderDetails(unique_order_id.into());
        http.get(ep, &{}).await.and_then(|res| res.into_data())
    }
}
