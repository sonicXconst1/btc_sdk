pub type Balance = Vec<Currency>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Currency {
    pub currency: String,
    pub available: String,
    pub reserved: String,
}

pub type Orders = Vec<Order>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Order {
    id: u64,
    #[serde(rename="clientOrderId")]
    client_order_id: String,
    symbol: String,
    side: String,
    status: String,
    #[serde(rename="type")]
    order_type: String,
    #[serde(rename="timeInForce")]
    time_in_force: String,
    quantity: String,
    price: String,
    #[serde(rename="cumQuantity")]
    cim_quantity: String,
    #[serde(rename="createdAt")]
    created_at: String,
    #[serde(rename="udpatedAt")]
    updated_at: String,
    #[serde(rename="postOnly")]
    post_only: bool,
    #[serde(rename="expireTime")]
    expire_time: Option<String>,
}

pub type Symbols = Vec<Symbol>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Symbol {
    id: String,
    #[serde(rename="baseCurrency")]
    base_currency: String,
    #[serde(rename="quoteCurrency")]
    quote_currency: String,
    #[serde(rename="quantityIncrement")]
    quantity_increment: String,
    #[serde(rename="tickSize")]
    tick_size: String,
    #[serde(rename="takeLiquidityRate")]
    take_liquidity_rate: String,
    #[serde(rename="provideLiquidityRate")]
    provide_liquidity_rate: String,
    #[serde(rename="feeCurrency")]
    fee_currency: String,
}

pub type OrderBook = std::collections::HashMap<String, OrderBookPage>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct OrderBookPage {
    symbol: String,
    ask: Prices,
    bid: Prices,
    timestamp: String,
}

pub type Prices = Vec<Price>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Price {
    price: String,
    size: String,
}

pub mod typed {
    use std::str::FromStr;

    pub struct Currency {
        pub currency: String,
        pub available: f64,
        pub reserved: f64,
    }

    impl From<super::Currency> for Currency {
        fn from(from: super::Currency) -> Currency {
            Currency {
                currency: from.currency,
                available: f64::from_str(&from.available).unwrap(),
                reserved: f64::from_str(&from.reserved).unwrap(),
            }
        }
    }
}
