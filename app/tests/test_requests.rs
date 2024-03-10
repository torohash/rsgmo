mod common;
use anyhow::Result;
use rsgmo::v1::api::{
    private::{
        get_account_deposit_history::GetDepositHistoryParameters,
        get_account_fiat_deposit_history::GetFiatDepositHistoryParameters,
        get_account_withdrawal_history::GetWithdrawalHistoryParameters,
        get_active_orders::GetActiveOrdersParameters,
        get_orders::GetOrdersParameters,
        get_executions::GetExecutionsParameters,
        get_latest_executions::GetLatestExecutionsParameters,
        get_open_positions::GetOpenPositionsParameters,
        get_position_summary::GetPositionSummaryParameters,
        // post_account_transfer::PostAccountTransferParameters
    }, public::{
        get_klines::GetKlinesParameters,
        get_orderbooks::GetOrderbooksParameters,
        get_ticker::GetTickerParameters,
        get_trades::GetTradesParameters
    }
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
    common::delay_for_a_while().await;
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
    common::delay_for_a_while().await;
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
    common::delay_for_a_while().await;
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
    common::delay_for_a_while().await;
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
    common::delay_for_a_while().await;
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
    common::delay_for_a_while().await;
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
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_account_assets() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_account_assets().await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_account_trading_volume() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_account_trading_volume().await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_account_fiat_deposit_history() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_account_fiat_deposit_history(GetFiatDepositHistoryParameters::new("2022-01-02T00:00:00.000Z")).await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

// ドキュメント上では存在するか、実際にリクエストを送ると404エラーとなる
// https://api.coin.z.com/docs/#fiatWithdrawalHistory
// #[tokio::test]
// async fn test_get_account_fiat_withdrawal_history() -> Result<()> {
//     let api = common::setup_api_private();

//     let result = api.get_account_fiat_withdrawal_history(GetFiatWithdrawalHistoryParameters::new("2022-01-02T00:00:00.000Z")).await;
//     match result {
//         Ok(response) => {
//             println!("{:?}", response);
//             assert_eq!(response.status(), 0);
//         }
//         Err(e) => {
//             panic!("Error: {:?}", e);
//         }
//     }
//     Ok(())
// }

#[tokio::test]
async fn test_get_account_deposit_history() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_account_deposit_history(GetDepositHistoryParameters::new("BTC", "2022-01-02T00:00:00.000Z")).await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_account_withdrawal_history() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_account_withdrawal_history(GetWithdrawalHistoryParameters::new("BTC", "2022-01-02T00:00:00.000Z")).await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_orders() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_orders(GetOrdersParameters::new("123456789")).await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_active_orders() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_active_orders(GetActiveOrdersParameters::new("BTC")).await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_executions() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_executions(GetExecutionsParameters::new().with_order_id("123456789")).await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_latest_executions() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_latest_executions(GetLatestExecutionsParameters::new("BTC")).await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_open_positions() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_open_positions(GetOpenPositionsParameters::new("BTC_JPY")).await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

#[tokio::test]
async fn test_get_position_summary() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.get_position_summary(GetPositionSummaryParameters::new()).await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}

// テストを通すにはFX口座が必要
// また一度リクエストを送ると3分以上経過しないと再度リクエストを送ることができない。（too many requestとなる）
// #[tokio::test]
// async fn test_post_account_transfer() -> Result<()> {
//     let api = common::setup_api_private();

//     let result = api.post_account_transfer(PostAccountTransferParameters::new(100.0, "DEPOSIT")).await;
//     match result {
//         Ok(response) => {
//             println!("{:?}", response);
//             assert_eq!(response.status(), 0);
//         }
//         Err(e) => {
//             panic!("Error: {:?}", e);
//         }
//     }
//     common::delay_for_a_while().await;
//     Ok(())
// }