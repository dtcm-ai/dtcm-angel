use dtcm_angel::{types::ExchangeType, SmartConnect};

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();

    let ltp_data_req = SmartConnect::new_ltp_data(ExchangeType::NSE, "SBIN-EQ", "3045");
    let ltp = sc.ltp_data(&ltp_data_req).await.unwrap();

    println!("{:?}", ltp);
}
