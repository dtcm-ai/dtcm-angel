use dtcm_angel::{
    ws::{AngelOneWs, Message, SubscriptionBuilder, SubscriptionExchange, SubscriptionMode},
    SmartConnect,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, &client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();
    let feed_token = sc.current_feed_token().unwrap();

    let ao_ws = AngelOneWs::new(client_code, feed_token);

    let mut ws_stream = ao_ws.stream::<Message>().await.unwrap();

    let subscription_message = SubscriptionBuilder::new("abcde12345")
        .mode(SubscriptionMode::SnapQuote)
        .subscribe(SubscriptionExchange::NSECM, vec!["10626", "5290"])
        .subscribe(
            SubscriptionExchange::MCXFO,
            vec!["234230", "234235", "234219"],
        )
        .build()
        .unwrap();

    ws_stream.subscribe(subscription_message).await.unwrap();
    while let Some(m) = ws_stream.next().await {
        println!("{:?}", m)
    }
}
