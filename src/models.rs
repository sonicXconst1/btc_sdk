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
    pub id: u64,
    #[serde(rename="clientOrderId")]
    pub client_order_id: String,
    pub symbol: String,
    pub side: String,
    pub status: String,
    #[serde(rename="type")]
    pub order_type: String,
    #[serde(rename="timeInForce")]
    pub time_in_force: String,
    quantity: String,
    pub price: String,
    #[serde(rename="cumQuantity")]
    pub cim_quantity: String,
    #[serde(rename="createdAt")]
    pub created_at: String,
    #[serde(rename="udpatedAt")]
    pub updated_at: String,
    #[serde(rename="postOnly")]
    pub post_only: bool,
    #[serde(rename="expireTime")]
    pub expire_time: Option<String>,
}

pub type Symbols = Vec<Symbol>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Symbol {
    pub id: String,
    #[serde(rename="baseCurrency")]
    pub base_currency: String,
    #[serde(rename="quoteCurrency")]
    pub quote_currency: String,
    #[serde(rename="quantityIncrement")]
    pub quantity_increment: String,
    #[serde(rename="tickSize")]
    pub tick_size: String,
    #[serde(rename="takeLiquidityRate")]
    pub take_liquidity_rate: String,
    #[serde(rename="provideLiquidityRate")]
    pub provide_liquidity_rate: String,
    #[serde(rename="feeCurrency")]
    pub fee_currency: String,
}

pub type OrderBook = std::collections::HashMap<String, OrderBookPage>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct OrderBookPage {
    pub symbol: String,
    pub ask: Prices,
    pub bid: Prices,
    pub timestamp: String,
}

pub type Prices = Vec<Price>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Price {
    pub price: String,
    pub size: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct OrderbookExactSymbol {
    pub ask: Prices,
    pub bid: Prices,
    pub timestamp: String,
    #[serde(rename="askAveragePrice")]
    pub ask_average_price: String,
    #[serde(rename="bidAveragePrice")]
    pub bid_average_price: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct CreateOrder {
    pub symbol: String,
    pub side: String,
    pub quantity: String,
    pub price: String,
}

pub mod typed {
    use std::str::FromStr;
    use super::super::base;
    use super::super::coin;


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

    pub struct CreateOrder {
        symbol: coin::Symbol,
        side: base::Side,
        quantity: f64,
        price: f64,
    }

    impl CreateOrder {
        pub fn new(
            symbol: coin::Symbol,
            side: base::Side,
            quantity: f64,
            price: f64
        ) -> CreateOrder {
            CreateOrder {
                symbol,
                side,
                quantity,
                price,
            }
        }

        pub fn to_model(self) -> super::CreateOrder {
            let symbol = self.symbol.to_string();
            let side = self.side.to_string().to_owned();
            let quantity = format!("{}", self.quantity);
            let price = format!("{}", self.price);
            super::CreateOrder {
                symbol,
                side,
                quantity,
                price,
            }
        }
    }
}
