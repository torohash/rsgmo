use rsgmo::v1::api::GmoApi;
use std::env;
use chrono::{Local, Datelike};
use tokio::time::{sleep, Duration};

pub fn setup_api_private() -> GmoApi {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let api_secret = env::var("API_SECRET").expect("API_SECRET must be set");

    GmoApi::new(Some(api_key), Some(api_secret))
}

pub fn setup_api_public() -> GmoApi {
    GmoApi::new(None, None)
}

pub fn get_today() -> String {
    let now = Local::now();
    let formatted_date = format!("{:04}{:02}{:02}", now.year(), now.month(), now.day());
    formatted_date
}

pub async fn delay_for_a_while() {
    sleep(Duration::from_millis(250)).await;
}