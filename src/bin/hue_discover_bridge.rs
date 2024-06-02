extern crate hueclient;
use hueclient::Bridge;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let bridge = Bridge::discover().await.unwrap();
    println!("Hue bridge found: {:?}", bridge);
}
