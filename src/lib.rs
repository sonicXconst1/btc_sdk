use hmac::NewMac;
use hmac::Mac;
pub mod client;
pub mod context;
pub mod models;
pub mod endpoint;
pub mod extractor;

pub fn authentificate(
    public_key: &str,
    secret_key: &str,
    url: url::Url,
    body: Vec<u8>,
    method: http::Method,
) -> String {
    let timestamp = chrono::Utc::now().timestamp().to_string();
    let path_with_query = &url[url::Position::BeforePath..];
    let body = String::from_utf8(body).expect("Body must be valid UTF-8");
    let method = method.to_string();
    let message = format!("{}{}{}{}", method, timestamp, path_with_query, body);
    let mut signature = hmac::Hmac::<sha2::Sha256>::new_varkey(secret_key.as_bytes())
        .expect("HMAC can take key of any size");
    signature.update(message.as_bytes());
    let signature = signature.finalize();
    let result = base64::encode(&format!(
        "{}:{}:{}",
        public_key,
        timestamp,
        hex::encode(signature.into_bytes())));
    format!("HS256 {}", result)
}

pub fn get_balance(
    base_url: url::Url,
    public_key: &str,
    private_key: &str,
) -> Option<http::Request<hyper::Body>> {
    let mut url = base_url.clone();
    url.path_segments_mut()
        .unwrap()
        .push("account")
        .push("balance");
    let method = hyper::Method::GET;
    let body = hyper::Body::empty();
    hyper::Request::builder()
        .header("Accept", "application/json")
        .header("Authorization", authentificate(
            public_key,
            private_key,
            url.clone(),
            Vec::new(),
            method.clone()))
        .uri(url.to_string())
        .method(method)
        .body(body)
        .ok()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let result = authentificate(
            "PUBLIC_KEY",
            "SECRET_LEY",
            url::Url::parse("https://google.com/hello").unwrap(),
            Vec::new(),
            http::Method::GET);
        println!("{}", result)
    }
}
