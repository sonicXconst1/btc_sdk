pub type Balance = Vec<Currency>;

#[derive(serde::Deserialize, Clone)]
pub struct Currency {
    pub currency: String,
    pub available: f64,
    pub reserved: f64,
}
