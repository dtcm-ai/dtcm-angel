use super::types::SubscriptionExchange;

/// Placeholder for the subscription token
#[derive(Default, Debug, Serialize)]
pub struct SubscriptionToken {
    /// Exchange type
    #[serde(rename = "exchangeType")]
    pub exchange: SubscriptionExchange,
    /// List of tokens per exchange
    pub tokens: Vec<String>,
}

impl SubscriptionToken {
    /// Returns a new instance for [`SubscriptionToken`]
    pub fn new<T>(exchange: SubscriptionExchange, token: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            exchange,
            tokens: vec![token.into()],
        }
    }

    /// Checks if [`SubscriptionToken`] contains token
    pub fn contains_token(&self, token: &String) -> bool {
        self.tokens.contains(token)
    }

    /// Adds the token to the [`SubscriptionToken`]
    pub fn push<T>(&mut self, token: T)
    where
        T: Into<String>,
    {
        let token = token.into();
        if !self.contains_token(&token) {
            self.tokens.push(token)
        }
    }
}
