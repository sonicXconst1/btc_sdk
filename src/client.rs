use super::coin;
use super::context;
use super::extractor;
use super::models;

pub const BAD_URL: &'static str = "Unable to modify path of url";
pub const BAD_BODY: &'static str = "Bad body. Failed to get bytes.";

pub struct BTCClient<'a, TConnector> {
    client: &'a hyper::Client<TConnector>,
    auth_context: &'a context::AuthContext,
}

impl<'a, TConnector> BTCClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    const ACCOUNT: &'static str = "account";
    const BALANCE: &'static str = "balance";
    const ORDER: &'static str = "order";

    pub fn new(
        client: &'a hyper::Client<TConnector>,
        auth_context: &'a context::AuthContext
    ) -> BTCClient<'a, TConnector> {
        BTCClient {
            client,
            auth_context,
        }
    }

    pub async fn get_balance(&self) -> Option<models::Balance> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect(BAD_URL)
            .push(Self::ACCOUNT)
            .push(Self::BALANCE);
        let (header, body) = process(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::GET,
            hyper::Body::empty(),
        )
        .await;
        log::info!("Header: {:#?}", header);
        extractor::extract_balance(body).await
    }

    pub async fn get_all_orders(&self, coins: Option<coin::Symbol>) -> Option<models::Orders> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut().expect(BAD_URL).push(Self::ORDER);
        if let Some(coins) = coins {
            url.query_pairs_mut()
                .append_pair("symbol", &coins.to_string());
        }
        let (header, body) = process(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::GET,
            hyper::Body::empty(),
        )
        .await;
        log::info!("Header: {:#?}", header);
        extractor::extract_orders(body).await
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

async fn process<TConnector>(
    client: &hyper::Client<TConnector>,
    auth_context: &context::AuthContext,
    url: url::Url,
    method: hyper::Method,
    body: hyper::Body,
) -> (http::response::Parts, hyper::Body)
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    let body = hyper::body::to_bytes(body).await.expect(BAD_BODY);
    let body = String::from_utf8(body.to_vec()).expect("Body must be valid UTF-8");
    let timestamp = chrono::Utc::now().timestamp().to_string();
    let path_with_query = &url[url::Position::BeforePath..];
    let message = get_message(method.clone(), &timestamp, path_with_query, &body);
    let jwt = auth_context.sign(message, timestamp);
    let request = hyper::Request::builder()
        .header("Accept", "application/json")
        .header("Authorization", jwt)
        .uri(url.to_string())
        .method(method)
        .body(hyper::Body::empty())
        .expect("Failed to build request!");
    client.request(request).await.unwrap().into_parts()
}
