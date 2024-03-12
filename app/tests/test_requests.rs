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
        // post_order::PostOrderParameters
        put_ws_auth::PutWsAuthParameters,
        delete_ws_auth::DeleteWsAuthParameters,
    }, public::{
        get_klines::GetKlinesParameters,
        get_orderbooks::GetOrderbooksParameters,
        get_ticker::GetTickerParameters,
        get_trades::GetTradesParameters
    }
};

use rsgmo::v1::ws::{
    public::connect_ticker::ConnectTickerParameters,
    public::connect_ticker::ConnectTickerResponse,
    public::connect_orderbooks::ConnectOrderbooksParameters,
    public::connect_orderbooks::ConnectOrderbooksResponse,
    public::connect_trades::ConnectTradesParameters,
    public::connect_trades::ConnectTradesResponse,
    private::connect_execution_events::ConnectExecutionEventsParameters,
    Channel,
    CommandType,
};
use futures::stream::StreamExt;

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

// #[tokio::test]
// async fn test_post_order() -> Result<()> {
//     let api = common::setup_api_private();

//     let result = api.post_order(
//         PostOrderParameters::new("BTC_JPY", "BUY", "LIMIT", 0.01)
//             .with_price(8_000_000.0)
//     ).await;
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

#[tokio::test]
async fn test_ws_auth() -> Result<()> {
    let api = common::setup_api_private();

    let result = api.post_ws_auth().await;
    match result {
        Ok(response) => {
            println!("{:?}", response);
            assert_eq!(response.status(), 0);

            let result = api.put_ws_auth(PutWsAuthParameters::new(response.data())).await;
            match result {
                Ok(response) => {
                    println!("{:?}", response);
                    assert_eq!(response.status(), 0);
                }
                Err(e) => {
                    panic!("Error: {:?}", e);
                }
            }

            let result = api.delete_ws_auth(DeleteWsAuthParameters::new(response.data())).await;
            match result {
                Ok(response) => {
                    println!("{:?}", response);
                    assert_eq!(response.status(), 0);
                }
                Err(e) => {
                    panic!("Error: {:?}", e);
                }
            }

        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
    common::delay_for_a_while().await;
    Ok(())
}



#[tokio::test]
async fn test_connect_ticker() {
    let ws = common::setup_ws_public().await;
    let (_, mut read) = ws.connect_ticker(ConnectTickerParameters::new(CommandType::Subscribe, "BTC")).await.unwrap();
    while let Some(response) = read.next().await {
        match response {
            Ok(message) => {
                let body = message.to_text().unwrap();
                println!("Received message: {}", body);
                let res: ConnectTickerResponse = serde_json::from_str(body).unwrap();
                assert_eq!(res.channel(), Channel::Ticker.to_string());
                assert_eq!(res.symbol(), "BTC");
                break;
            }
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
    common::delay_for_a_while_long().await;
}

#[tokio::test]
async fn test_connect_orderbooks() {
    let ws = common::setup_ws_public().await;
    let (_, mut read) = ws.connect_orderbooks(ConnectOrderbooksParameters::new(CommandType::Subscribe, "BTC")).await.unwrap();
    while let Some(response) = read.next().await {
        match response {
            Ok(message) => {
                let body = message.to_text().unwrap();
                println!("Received message: {}", body);
                let res: ConnectOrderbooksResponse = serde_json::from_str(body).unwrap();
                assert_eq!(res.channel(), Channel::Orderbooks.to_string());
                assert_eq!(res.symbol(), "BTC");
                break;
            }
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
    common::delay_for_a_while_long().await;
}

#[tokio::test]
async fn test_connect_trades() {
    let ws = common::setup_ws_public().await;
    let (_, mut read) = ws.connect_trades(ConnectTradesParameters::new(CommandType::Subscribe, "BTC")).await.unwrap();
    while let Some(response) = read.next().await {
        match response {
            Ok(message) => {
                let body = message.to_text().unwrap();
                println!("Received message: {}", body);
                let res: ConnectTradesResponse = serde_json::from_str(body).unwrap();
                assert_eq!(res.channel(), Channel::Trades.to_string());
                assert_eq!(res.symbol(), "BTC");
                break;
            }
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
    common::delay_for_a_while_long().await;
}

#[tokio::test]
async fn test_connect_execution_events() {
    let ws = common::setup_ws_private().await.unwrap();
    let (_, _) = ws.connect_execution_events(ConnectExecutionEventsParameters::new(CommandType::Subscribe)).await.unwrap();
    common::delay_for_a_while_long().await;
}