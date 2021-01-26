use hmac::NewMac;
use hmac::Mac;

pub struct AuthContext {
    public_key: String,
    private_key: String,
    pub base_url: url::Url,
}

impl AuthContext {
    pub fn new(
        public_key: String,
        private_key: String,
        base_url: url::Url,
    ) -> Self {
        AuthContext {
            public_key: public_key.into(),
            private_key: private_key.into(),
            base_url,
        }
    }

    pub fn sign(&self, message: String, timestamp: String) -> String {
        let mut signature = hmac::Hmac::<sha2::Sha256>::new_varkey(
            self.private_key.as_bytes())
            .expect("HMAC can take key of any size");
        signature.update(message.as_bytes());
        let signature = signature.finalize();
        let result = base64::encode(&format!(
            "{}:{}:{}",
            self.public_key,
            timestamp,
            hex::encode(signature.into_bytes())));
        format!("HS256 {}", result)
    }
}
