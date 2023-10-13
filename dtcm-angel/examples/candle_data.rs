use std::collections::HashMap;

use dtcm_angel::{
    types::{ExchangeType, MarketDataExchange},
    SmartConnect,
};

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();

    let candle_data_req = SmartConnect::new_candle_data(
        MarketDataExchange::NSE,
        "3045",
        dtcm_angel::types::Interval::_1m,
        "2021-02-10 09:15",
        "2021-02-10 09:16",
    )
    .unwrap();
    let cd = sc.candle_data(&candle_data_req).await.unwrap();

    println!("{:?}", cd);
}
