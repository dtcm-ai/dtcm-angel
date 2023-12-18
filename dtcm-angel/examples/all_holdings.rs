use dtcm_angel::SmartConnect;

#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();

    let res = sc.all_holdings().await.unwrap();
    println!("{:?}", res);
}
