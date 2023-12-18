use dtcm_angel::SmartConnect;

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();

    let unique_order_id = "b9dda396-a6e9-4992-be67-5373aad193b4";
    let ob = sc.order_status(unique_order_id).await.unwrap();
    println!("{:?}", ob);
}
