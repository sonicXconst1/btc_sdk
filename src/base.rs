#[derive(Clone, Copy, Debug)]
pub enum Side {
    Sell,
    Buy,
}

impl Side {
    pub fn to_string(self) -> &'static str {
        match self {
            Side::Sell => "sell",
            Side::Buy => "buy",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Limit,
    Market,
    StopLimit,
    StopMarket,
}

impl Type {
    pub fn to_string(self) -> &'static str {
        match self {
            Type::Limit => "limit",
            Type::Market => "market",
            Type::StopLimit => "stopLimit",
            Type::StopMarket => "stopMarket",
        }
    }
}
