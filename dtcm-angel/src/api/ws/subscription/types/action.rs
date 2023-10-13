use serde_repr::Serialize_repr;

/// Subscription action types
#[derive(Debug, Serialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum SubscriptionAction {
    /// Unsubscribe action
    UnSubscribe = 0,
    /// Subscribe action
    Subscribe,
}

impl Default for SubscriptionAction {
    fn default() -> Self {
        Self::Subscribe
    }
}
