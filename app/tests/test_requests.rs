mod common;
use anyhow::Result;
use rsgmo::v1::api::public::{
    get_ticker::GetTickerParameters,
    get_orderbooks::GetOrderbooksParameters,
    get_trades::GetTradesParameters,
    get_klines::GetKlinesParameters,
};

#[tokio::test]
async fn test_authenticated_get_requests() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_account_margin().await;
    match result {
        Ok(response) => {
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_public_requests() -> Result<()> {
    let api = common::setup_api_public();

    let result = api.get_exchange_status().await;
    match result {
        Ok(response) => {
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_get_ticker() -> Result<()> {
    let api = common::setup_api_public();

    let parameters = Some(GetTickerParameters::new("BTC"));
    let result = api.get_ticker(parameters).await;
    match result {
        Ok(response) => {
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_get_orderbooks() -> Result<()> {
    let api = common::setup_api_public();

    let parameters = Some(GetOrderbooksParameters::new("BTC"));
    let result = api.get_orderbooks(parameters).await;
    match result {
        Ok(response) => {
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_get_trades() -> Result<()> {
    let api = common::setup_api_public();

    let parameters = Some(GetTradesParameters::new("BTC"));
    let result = api.get_trades(parameters).await;
    match result {
        Ok(response) => {
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_get_klines() -> Result<()> {
    let api = common::setup_api_public();


    let parameters = Some(GetKlinesParameters::new("BTC", "1hour", common::get_today().as_str()));
    let result = api.get_klines(parameters).await;
    match result {
        Ok(response) => {
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_get_symbols() -> Result<()> {
    let api = common::setup_api_public();

    let result = api.get_symbols().await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    Ok(())
}