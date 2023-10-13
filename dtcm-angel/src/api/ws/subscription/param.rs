use super::{types::SubscriptionMode, SubscriptionExchange, SubscriptionToken};

/// Placeholder for subscription parameters to be send within the subscription requests
#[derive(Default, Debug, Serialize)]
pub struct SubscriptionParam {
    /// Subscription mode
    pub mode: SubscriptionMode,
    /// Subscription token list
    #[serde(rename = "tokenList")]
    pub token_list: Vec<SubscriptionToken>,
}

impl SubscriptionParam {
    /// Checks for existing [`SubscriptionExchange`] in [`Vec<SubscriptionToken>`] and returns [`Option<&SubscriptionToken>`]
    pub fn find_exchange(&self, exchange: SubscriptionExchange) -> Option<&SubscriptionToken> {
        self.token_list.iter().find(|tl| tl.exchange == exchange)
    }

    /// Checks for existing [`SubscriptionExchange`] in [`Vec<SubscriptionToken>`] and returns [`Option<&mut SubscriptionToken>`]
    pub fn find_exchange_mut(
        &mut self,
        exchange: SubscriptionExchange,
    ) -> Option<&mut SubscriptionToken> {
        self.token_list
            .iter_mut()
            .find(|tl| tl.exchange == exchange)
    }

    /// Adds the token to the [`SubscriptionParam`]
    pub fn push<T>(&mut self, exchange: SubscriptionExchange, token: T)
    where
        T: Into<String>,
    {
        match self.find_exchange_mut(exchange) {
            Some(tl) => tl.push(token),
            None => self
                .token_list
                .push(SubscriptionToken::new(exchange, token)),
        }
    }
}
