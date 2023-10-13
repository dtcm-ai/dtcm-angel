use dtcm_angel_utils::Result;

use super::{
    types::SubscriptionMode, SubscriptionAction, SubscriptionExchange, SubscriptionParam,
    SubscriptionRequest,
};

/// Builder for [`SubscriptionRequest`]
pub struct SubscriptionBuilder {
    correlation_id: String,
    action: SubscriptionAction,
    param: SubscriptionParam,
}

impl SubscriptionBuilder {
    /// Returns a new instance of [`SubscriptionBuilder`]
    pub fn new<C>(correlation_id: C) -> Self
    where
        C: Into<String>,
    {
        Self {
            correlation_id: correlation_id.into(),
            action: Default::default(),
            param: Default::default(),
        }
    }

    /// Sets the [`SubscriptionAction`]
    pub fn action(mut self, action: SubscriptionAction) -> Self {
        self.action = action;
        self
    }

    /// Sets the [`SubscriptionMode`]
    pub fn mode(mut self, mode: SubscriptionMode) -> Self {
        self.param.mode = mode;
        self
    }

    /// Sets the token in [`SubscriptionExchange`]
    pub fn token<T>(mut self, exchange: SubscriptionExchange, token: T) -> Self
    where
        T: Into<String>,
    {
        self.param.push(exchange, token);

        self
    }

    /// Sets the tokens in [`SubscriptionExchange`]
    pub fn tokens<T>(mut self, exchange: SubscriptionExchange, tokens: Vec<T>) -> Self
    where
        T: Into<String>,
    {
        for token in tokens.into_iter() {
            self.param.push(exchange, token);
        }

        self
    }

    /// Returns a subscribe request for [`SubscriptionExchange`]
    pub fn subscribe<T>(mut self, exchange: SubscriptionExchange, tokens: Vec<T>) -> Self
    where
        T: Into<String>,
    {
        self.action = SubscriptionAction::Subscribe;
        self.tokens(exchange, tokens)
    }

    /// Returns a unsubscribe request for [`SubscriptionExchange`]
    pub fn unsubscribe<T>(mut self, exchange: SubscriptionExchange, tokens: Vec<T>) -> Self
    where
        T: Into<String>,
    {
        self.action = SubscriptionAction::UnSubscribe;
        self.tokens(exchange, tokens)
    }

    /// Builds the [`SubscriptionRequest`]
    pub fn build(self) -> Result<SubscriptionRequest> {
        let param = self.param;

        if param.token_list.is_empty() {
            return Err("Params required for the subscription request".into());
        }

        if param.token_list.iter().any(|tl| tl.tokens.is_empty()) {
            return Err("Token required for the token list".into());
        }

        let req = SubscriptionRequest {
            correlation_id: self.correlation_id,
            action: self.action,
            param,
        };

        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::{SubscriptionAction, SubscriptionBuilder, SubscriptionExchange};

    #[test]
    fn default_builder_fails() {
        let builder = SubscriptionBuilder::new("123");
        assert!(builder.build().is_err())
    }

    #[test]
    fn empty_token_fails() {
        let builder = SubscriptionBuilder::new("123")
            .subscribe(crate::ws::SubscriptionExchange::BSEFO, Vec::<String>::new());
        assert!(builder.build().is_err())
    }

    #[test]
    fn single_exchange_single_token_works() {
        let builder =
            SubscriptionBuilder::new("123").subscribe(SubscriptionExchange::BSEFO, vec!["token1"]);
        let request = builder.build().unwrap();

        assert_eq!(request.action, SubscriptionAction::Subscribe);
        assert!(request.find_exchange(SubscriptionExchange::BSEFO).is_some());

        assert_eq!(request.param.token_list[0].tokens, vec!["token1"]);
    }

    #[test]
    fn single_exchange_multiple_token_works() {
        let builder = SubscriptionBuilder::new("123")
            .unsubscribe(SubscriptionExchange::BSEFO, vec!["token1"])
            .token(SubscriptionExchange::BSEFO, "token2");
        let request = builder.build().unwrap();

        assert_eq!(request.action, SubscriptionAction::UnSubscribe);
        assert!(request
            .param
            .find_exchange(SubscriptionExchange::BSEFO)
            .is_some());

        assert_eq!(request.param.token_list[0].tokens, vec!["token1", "token2"]);
    }

    #[test]
    fn multiple_exchange_multiple_token_works() {
        let builder = SubscriptionBuilder::new("123")
            .unsubscribe(SubscriptionExchange::BSEFO, vec!["token1", "token2"])
            .token(SubscriptionExchange::NSEFO, "token2");

        let request = builder.build().unwrap();
        assert_eq!(request.action, SubscriptionAction::UnSubscribe);

        assert!(request.find_exchange(SubscriptionExchange::BSEFO).is_some());
        assert!(request.find_exchange(SubscriptionExchange::NSEFO).is_some());

        assert_eq!(request.param.token_list[0].tokens, vec!["token1", "token2"]);
        assert_eq!(request.param.token_list[1].tokens, vec!["token2"]);
    }
}
