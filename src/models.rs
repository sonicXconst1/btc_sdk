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
    pub quantity: String,
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

#[derive(serde::Serialize, Clone, Debug)]
pub struct CreateOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="clientOrderId")]
    pub client_order_id: Option<String>,
    pub symbol: String,
    pub side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="type")]
    pub order_type: Option<String>, // default: limit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="timeInForce")]
    pub time_in_force: Option<String>, // default GTC
    pub quantity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>, // only for limit type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="stopPrice")]
    pub stop_price: Option<String>, // only for stop-limit and stop-market type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="expireTime")]
    pub expire_time: Option<String>, // only for time in force GTD
    #[serde(rename="strictValidate")]
    pub strict_validate: String,
    #[serde(rename="postOnly")]
    pub post_only: bool,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct CreateMarketOrder {
    pub symbol: String,
    pub side: String,
    pub quantity: String,
    #[serde(rename="type")]
    pub order_type: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct CreateLimitOrder {
    pub symbol: String,
    pub side: String,
    pub quantity: String,
    pub price: String,
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

    pub struct CreateMarketOrder {
        symbol: coin::Symbol,
        side: base::Side,
        quantity: f64,
    }

    impl CreateMarketOrder {
        pub fn new(
            symbol: coin::Symbol,
            side: base::Side,
            quantity: f64,
        ) -> CreateMarketOrder {
            CreateMarketOrder {
                symbol,
                side,
                quantity,
            }
        }

        pub fn to_model(self) -> super::CreateMarketOrder {
            let symbol = self.symbol.to_string();
            let side = self.side.to_string().to_owned();
            let quantity = format!("{}", self.quantity);
            let order_type = base::Type::Market.to_string().to_owned();
            super::CreateMarketOrder {
                symbol,
                side,
                quantity,
                order_type,
            }
        }
    }

    pub struct CreateLimitOrder {
        symbol: coin::Symbol,
        side: base::Side,
        quantity: f64,
        price: f64,
    }

    impl CreateLimitOrder {
        pub fn new(
            symbol: coin::Symbol,
            side: base::Side,
            quantity: f64,
            price: f64,
        ) -> CreateLimitOrder {
            CreateLimitOrder {
                symbol,
                side,
                quantity,
                price,
            }
        }

        pub fn to_model(self) -> super::CreateLimitOrder {
            super::CreateLimitOrder {
                symbol: self.symbol.to_string(),
                side: self.side.to_string().to_owned(),
                quantity: format!("{}", self.quantity),
                price: format!("{}", self.price),
            }
        }
    }
}
