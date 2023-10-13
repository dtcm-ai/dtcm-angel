use std::collections::HashMap;

use dtcm_angel::{types::ExchangeType, SmartConnect};

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();

    let exchange_tokens = HashMap::from([(ExchangeType::NSE, vec![String::from("3045")])]);
    let market_data_req =
        SmartConnect::new_market_data(dtcm_angel::types::MarketMode::Full, exchange_tokens);
    let md = sc.market_data(&market_data_req).await.unwrap();

    println!("{:?}", md);
}
