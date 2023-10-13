use super::{
    types::SubscriptionAction, SubscriptionExchange, SubscriptionParam, SubscriptionToken,
};

/// Subscription request to be made to the [`crate::api::ws::AngelOneWs`]
#[derive(Debug, Serialize)]
pub struct SubscriptionRequest {
    /// A 10 character alphanumeric ID client may provide which will be returned by the server
    /// in error response to indicate which request generated error response.
    #[serde(rename = "correlationID")]
    pub correlation_id: String,
    /// Subscription action
    pub action: SubscriptionAction,
    /// Subscription parameters
    #[serde(rename = "params")]
    pub param: SubscriptionParam,
}

impl SubscriptionRequest {
    /// Checks for existing [`SubscriptionExchange`] in [`Vec<SubscriptionToken>`] and returns [`Option<&mut SubscriptionToken>`]
    pub fn find_exchange(&self, exchange: SubscriptionExchange) -> Option<&SubscriptionToken> {
        self.param.find_exchange(exchange)
    }

    /// Checks for existing [`SubscriptionExchange`] in [`Vec<SubscriptionToken>`] and returns [`bool`]
    pub fn contains_exchange(&self, exchange: SubscriptionExchange) -> bool {
        self.param.find_exchange(exchange).is_some()
    }

    /// Checks if the [`SubscriptionRequest`] is [`SubscriptionAction::Subscribe`]
    pub fn is_subscribe(&self) -> bool {
        self.action == SubscriptionAction::Subscribe
    }

    /// Checks if the [`SubscriptionRequest`] is [`SubscriptionAction::UnSubscribe`]
    pub fn is_unsubscribe(&self) -> bool {
        self.action == SubscriptionAction::UnSubscribe
    }
}
