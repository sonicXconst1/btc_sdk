use super::error;
use super::models;

pub async fn extract_balance(body: hyper::Body) -> Option<models::Balance> {
    read_body(body).await
}

pub async fn extract_currencies(body: hyper::Body) -> Option<Vec<models::PublicCurrency>> {
    read_body(body).await
}

pub async fn extract_orders(body: hyper::Body) -> Option<models::Orders> {
    read_body(body).await
}

pub async fn extract_order(body: hyper::Body) -> Option<models::Order> {
    read_body(body).await
}

pub async fn extract_symbols(body: hyper::Body) -> Option<models::Symbols> {
    read_body(body).await
}

pub async fn extract_orderbook(body: hyper::Body) -> Option<models::OrderBook> {
    read_body(body).await
}

pub async fn extract_orderbook_exact_symbol(
    body: hyper::Body,
) -> Option<models::OrderbookExactSymbol> {
    read_body(body).await
}

pub async fn extract_error(body: hyper::Body) -> Option<error::Error> {
    read_body(body).await
}

pub async fn extract_trading_commission(body: hyper::Body) -> Option<models::TradingCommission> {
    read_body(body).await
}

async fn read_body<TResult>(body: hyper::Body) -> Option<TResult>
where
    TResult: serde::de::DeserializeOwned,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(error) => {
            log::error!("Failed to convert Body to bytes: {:#?}", error);
            return None;
        },
    };
    match serde_json::from_slice(&bytes) {
        Ok(result) => Some(result),
        Err(error) => {
            log::error!("Error on reading the body: {:#?}", error);
            log::error!("Json: {:#?}", String::from_utf8(bytes.to_vec()));
            None
        }
    }
}
