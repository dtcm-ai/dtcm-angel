use dtcm_angel::{order::OrderSetter, SmartConnect};

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();

    // Create new order request
    let mut order_req =
        SmartConnect::new_place_order("SBIN-EQ", "3045", dtcm_angel::types::TransactionType::Buy);
    // Configure order request
    order_req = order_req.quantity(1);

    // Place order
    let resp = sc.place_order(&order_req).await.unwrap();
    // Get order ID from response
    let order_id = resp.order_id.unwrap();

    println!("[*] Order placed for {} with id: {}", resp.script, order_id);

    // Search for the order from API
    let tb = sc.order_book().await.unwrap();
    assert!(tb.iter().find(|o| o.order_id == order_id).is_some());

    // Cancel order
    let cancel_order = SmartConnect::new_cancel_order(order_req.inner.variety, order_id);
    let res = cancel_order.send(&sc.http).await.unwrap();
    println!("{:?}", res);
}
