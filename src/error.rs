#[derive(serde::Deserialize, Clone, Debug)]
pub struct Error {
    pub error: DefaultError,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DefaultError {
    pub code: u32,
    pub message: String,
    pub description: Option<String>,
}
