#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Coin {
    TON,
    BTC,
    USDT,
    Unknown(String),
}

impl Coin {
    pub fn to_string(self) -> String {
        match self {
            Coin::TON => "TON".into(),
            Coin::BTC => "BTC".into(),
            Coin::USDT => "USD".into(),
            Coin::Unknown(name) => name,
        }
    }
}

impl From<&str> for Coin {
    fn from(coin: &str) -> Coin {
        match coin {
            "TON" => Coin::TON,
            "BTC" => Coin::BTC,
            "USD" => Coin::USDT,
            other => Coin::Unknown(other.into()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub left: Coin,
    pub right: Coin,
}

impl Symbol {
    pub fn new(left: Coin, right: Coin) -> Symbol {
        Symbol {
            left,
            right,
        }
    }

    pub fn reversed(self) -> Symbol {
        Symbol {
            left: self.right,
            right: self.left
        }
    }

    pub fn to_string(self) -> String {
        format!("{}{}", self.left.to_string(), self.right.to_string())
    }
}
