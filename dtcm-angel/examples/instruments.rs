use dtcm_angel::SmartConnect;

#[tokio::main]
async fn main() {
    let instruments = SmartConnect::instruments().await.unwrap();

    println!("{:?}", instruments);
}
