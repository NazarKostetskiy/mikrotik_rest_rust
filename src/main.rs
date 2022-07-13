use client::Client;
use reqwest::Url;

mod client;

// TODO: Use environment variables here
const ROUTER_URL: &'static str = "https://192.168.88.1";
const USERNAME: &'static str = "admin";
const PASSWORD: &'static str = "";

fn main() {
    let mikrotik_client = Client::new(
        Url::parse(ROUTER_URL).unwrap(),
        USERNAME.to_string(),
        PASSWORD.to_string(),
        true,
    );
}
