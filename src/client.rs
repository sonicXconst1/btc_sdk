use super::context;
use super::models;
use super::extractor;

pub struct BTCClient<TConnector> {
    client: hyper::Client<TConnector>,
    auth_context: context::AuthContext,
}

impl<TConnector> BTCClient<TConnector>
where 
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static
{
    const ACCOUNT: &'static str = "account";
    const BALANCE: &'static str = "balance";

    pub fn new(
        client: hyper::Client<TConnector>,
        auth_context: context::AuthContext,
    ) -> Self {
        BTCClient {
            client,
            auth_context
        }
    }

    pub async fn get_balance(&self) -> Option<models::Balance> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect("Unable to modify path of url")
            .push(Self::ACCOUNT)
            .push(Self::BALANCE);
        let method = hyper::Method::GET;
        let body = hyper::Body::empty();
        let body = hyper::body::to_bytes(body)
            .await
            .expect("Body is not valid.");
        let body = String::from_utf8(body.to_vec())
            .expect("Body must be valid UTF-8");
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let path_with_query = &url[url::Position::BeforePath..];
        let message = get_message(method.clone(), &timestamp, path_with_query, &body);
        let jwt = self.auth_context.sign(message, timestamp);
        let request = hyper::Request::builder()
            .header("Accept", "application/json")
            .header("Authorization", jwt)
            .uri(url.to_string())
            .method(method)
            .body(hyper::Body::empty())
            .expect("Failed to build request!");
        let response = self.client.request(request)
            .await
            .unwrap();
        log::info!("{:#?}", response);
        let body = response.into_body();
        extractor::extract_balance(body).await
    }
}

fn get_message(
    method: hyper::Method,
    timestamp: &str,
    path_with_query: &str,
    body: &str,
) -> String {
    format!("{}{}{}{}", method, timestamp, path_with_query, body)
}
