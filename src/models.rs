pub type Balance = Vec<Currency>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Currency {
    pub currency: String,
    pub available: String,
    pub reserved: String,
}

pub mod typed {
    pub struct Currency {
        pub currency: String,
        pub available: f64,
        pub reserved: f64,
    }

    use std::str::FromStr;

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
