use std::fmt::Display;

use crate::types::{DurationType, ExchangeType, OrderType, OrderVariety, ProductType};

use super::OrderInner;

/// Sets the order values
pub trait OrderSetter {
    /// Returns the mutable reference to the order object
    fn inner_mut(&mut self) -> &mut OrderInner
    where
        Self: Sized;

    /// Sets the variety for the order placement
    fn variety(mut self, variety: OrderVariety) -> Self
    where
        Self: Sized,
    {
        self.inner_mut().variety = variety;
        self
    }

    /// Sets the exchange forthe order placement
    fn exchange(mut self, exchange: ExchangeType) -> Self
    where
        Self: Sized,
    {
        self.inner_mut().exchange = exchange;
        self
    }

    /// Sets the order_type for the order placement
    fn order_type(mut self, order_type: OrderType) -> Self
    where
        Self: Sized,
    {
        self.inner_mut().order_type = order_type;
        self
    }

    /// Sets the product_type for the order placement
    fn product_type(mut self, product_type: ProductType) -> Self
    where
        Self: Sized,
    {
        self.inner_mut().product_type = product_type;
        self
    }

    /// Sets the duration for the order placement
    fn duration(mut self, duration: DurationType) -> Self
    where
        Self: Sized,
    {
        self.inner_mut().duration = duration;
        self
    }

    /// Sets the price for the order placement
    fn price<P>(mut self, price: P) -> Self
    where
        P: Display,
        Self: Sized,
    {
        self.inner_mut().price = price.to_string();
        self
    }

    /// Sets the trigger_price for the order placement
    fn trigger_price<P>(mut self, trigger_price: P) -> Self
    where
        P: Display,
        Self: Sized,
    {
        self.inner_mut().trigger_price = Some(trigger_price.to_string());
        self
    }

    /// Sets the square_off for the order placement
    fn square_off<S>(mut self, square_off: S) -> Self
    where
        S: Display,
        Self: Sized,
    {
        self.inner_mut().square_off = Some(square_off.to_string());
        self
    }

    /// Sets the stop_loss for the order placement
    fn stop_loss<S>(mut self, stop_loss: S) -> Self
    where
        S: Display,
        Self: Sized,
    {
        self.inner_mut().stop_loss = Some(stop_loss.to_string());
        self
    }

    /// Sets the trailing_stop_loss for the order placement
    fn trailing_stop_loss<T>(mut self, trailing_stop_loss: T) -> Self
    where
        T: Display,
        Self: Sized,
    {
        self.inner_mut().trailing_stop_loss = Some(trailing_stop_loss.to_string());
        self
    }

    /// Sets the disclosed_quantity for the order placement, must be called after the quantity setter
    fn disclosed_quantity<Q>(mut self, quantity: Q) -> Self
    where
        Q: Display,
        Self: Sized,
    {
        self.inner_mut().disclosed_quantity = quantity.to_string();
        self
    }

    /// Sets the quantity for the order placement
    fn quantity<Q>(mut self, quantity: Q) -> Self
    where
        Q: Display,
        Self: Sized,
    {
        self.inner_mut().quantity = quantity.to_string();
        self.inner_mut().disclosed_quantity = quantity.to_string();
        self
    }
}
