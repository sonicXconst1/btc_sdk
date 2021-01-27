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
