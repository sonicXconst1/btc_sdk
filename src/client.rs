use super::coin;
use super::context;
use super::extractor;
use super::models;

pub const BAD_URL: &'static str = "Unable to modify path of url";
pub const BAD_BODY: &'static str = "Bad body. Failed to get bytes.";

pub struct BTCClient<TConnector> {
    client: std::sync::Arc<hyper::Client<TConnector>>,
    auth_context: std::sync::Arc<context::AuthContext>,
}

impl<TConnector> BTCClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    const ACCOUNT: &'static str = "account";
    const TRADING: &'static str = "trading";
    const FEE: &'static str = "fee";
    const BALANCE: &'static str = "balance";
    const ORDER: &'static str = "order";

    pub fn new(
        client: std::sync::Arc<hyper::Client<TConnector>>,
        auth_context: std::sync::Arc<context::AuthContext>
    ) -> BTCClient<TConnector> {
        BTCClient {
            client,
            auth_context,
        }
    }

    pub async fn get_account_balance(&self) -> Result<models::Balance, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect(BAD_URL)
            .push(Self::ACCOUNT)
            .push(Self::BALANCE);
        let (_header, body) = process_with_empty_body(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::GET).await?;
        match extractor::extract_balance(body).await {
            Some(balance) => Ok(balance),
            None => Err("Deserialization error".to_owned()),
        }
    }

    pub async fn get_trading_balance(&self) -> Result<models::Balance, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect(BAD_URL)
            .push(Self::TRADING)
            .push(Self::BALANCE);
        let (_header, body) = process_with_empty_body(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::GET).await?;
        match extractor::extract_balance(body).await {
            Some(balance) => Ok(balance),
            None => Err("Deserialization error".to_owned()),
        }
    }

    pub async fn get_active_orders(
        &self,
        coins: Option<coin::Symbol>
    ) -> Result<models::Orders, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut().expect(BAD_URL).push(Self::ORDER);
        if let Some(coins) = coins {
            url.query_pairs_mut()
                .append_pair("symbol", &coins.to_string());
        }
        let (_header, body) = process_with_empty_body(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::GET).await?;
        match extractor::extract_orders(body).await {
            Some(orders) => Ok(orders),
            None => Err("Deserialization error".to_owned()),
        }
    }

    pub async fn get_order_by_id(
        &self,
        id: &str,
        wait: Option<u64>,
    ) -> Result<models::Order, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect(BAD_URL)
            .push(Self::ORDER)
            .push(id);
        if let Some(wait) = wait {
            url.query_pairs_mut()
                .append_pair("wait", &format!("{}", wait));
        }
        let (_header, body) = process_with_empty_body(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::GET).await?;
        match extractor::extract_order(body).await {
            Some(order) => Ok(order),
            None => Err("Deserialization error".to_owned()),
        }
    }

    pub async fn create_market_order(
        &self,
        order: models::typed::CreateMarketOrder,
    ) -> Result<models::Order, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect(BAD_URL)
            .push(Self::ORDER);
        let body = serde_json::to_vec(&order.to_model())
            .expect("Failed to serialize CreateMarketOrder");
        let (header, response_body) = process(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::POST,
            body).await?;
        if header.status == hyper::StatusCode::OK{
            match extractor::extract_order(response_body).await {
                Some(order) => Ok(order),
                None => Err("Deserialization error".to_owned()),
            }
        } else {
            let error = extractor::extract_error(response_body).await;
            Err(format!("Error on creating order: {:#?}", error))
        }
    }

    pub async fn create_limit_order(
        &self,
        order: models::typed::CreateLimitOrder,
    ) -> Result<models::Order, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect(BAD_URL)
            .push(Self::ORDER);
        let body = serde_json::to_vec(&order.to_model())
            .expect("Failed to serialize CreateLimitOrder");
        let (header, response_body) = process(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::POST,
            body).await?;
        if header.status == hyper::StatusCode::OK{
            match extractor::extract_order(response_body).await {
                Some(order) => Ok(order),
                None => Err("Deserialization error".to_owned()),
            }
        } else {
            let error = extractor::extract_error(response_body).await;
            Err(format!("Error on creating order: {:#?}", error))
        }
    }

    pub async fn cancel_all_orders(
        &self,
        symbol: Option<coin::Symbol>,
    ) -> Result<models::Orders, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect(BAD_URL)
            .push(Self::ORDER);
        if let Some(symbol) = symbol {
            url.query_pairs_mut()
                .append_pair("symbol", &symbol.to_string());
        }
        let (_header, body) = process_with_empty_body(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::DELETE).await?;
        match extractor::extract_orders(body).await {
            Some(orders) => Ok(orders),
            None => Err("Deserialization error".to_owned()),
        }
    }
    
    pub async fn cancel_order_by_id(
        &self,
        id: &str
    ) -> Result<models::Order, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect(BAD_URL)
            .push(Self::ORDER)
            .push(id);
        let (_header, body) = process_with_empty_body(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::DELETE).await?;
       match extractor::extract_order(body).await {
            Some(order) => Ok(order),
            None => Err("Deserialization error".to_owned()),
        }
    }

    pub async fn get_trading_commission(
        &self,
        symbol: coin::Symbol,
    ) -> Result<models::TradingCommission, String> {
        let mut url = self.auth_context.base_url.clone();
        url.path_segments_mut()
            .expect(BAD_URL)
            .push(Self::TRADING)
            .push(Self::FEE)
            .push(&symbol.to_string());
        let (_header, body) = process_with_empty_body(
            &self.client,
            &self.auth_context,
            url,
            hyper::Method::GET).await?;
        match extractor::extract_trading_commission(body).await {
            Some(comission) => Ok(comission),
            None => Err("Deserialization error".to_owned()),
        }
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

async fn process_with_empty_body<TConnector>(
    client: &hyper::Client<TConnector>,
    auth_context: &context::AuthContext,
    url: url::Url,
    method: hyper::Method,
) -> Result<(http::response::Parts, hyper::Body), String>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    let body = hyper::Body::empty();
    let body = hyper::body::to_bytes(body).await.expect(BAD_BODY);
    let body = String::from_utf8(body.to_vec()).expect("Body must be valid UTF-8");
    let timestamp = chrono::Utc::now().timestamp().to_string();
    let path_with_query = &url[url::Position::BeforePath..];
    let message = get_message(method.clone(), &timestamp, path_with_query, &body);
    let jwt = auth_context.sign(message, timestamp);
    let request = match hyper::Request::builder()
        .header("Accept", "application/json")
        .header("Authorization", jwt)
        .uri(url.to_string())
        .method(method)
        .body(hyper::Body::empty()) {
        Ok(request) => request,
        Err(error) => return Err(format!("Failed create request: {:#?}", error)),
    };
    match client.request(request).await {
        Ok(response) => Ok(response.into_parts()),
        Err(error) => Err(format!("Failed to craete request: {:#?}", error)),
    }
}

async fn process<TConnector>(
    client: &hyper::Client<TConnector>,
    auth_context: &context::AuthContext,
    url: url::Url,
    method: hyper::Method,
    body_bytes: Vec<u8>,
) -> Result<(http::response::Parts, hyper::Body), String>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    let body = String::from_utf8(body_bytes)
        .expect("Body must be valid UTF-8");
    let timestamp = chrono::Utc::now().timestamp().to_string();
    let path_with_query = &url[url::Position::BeforePath..];
    let message = get_message(
        method.clone(),
        &timestamp,
        path_with_query,
        &body);
    let jwt = auth_context.sign(message, timestamp);
    let request = match hyper::Request::builder()
        .header("Content-Type", "application/json")
        .header("Authorization", jwt)
        .uri(url.to_string())
        .method(method)
        .body(hyper::Body::from(body)) {
        Ok(request) => request,
        Err(error) => return Err(format!("Failed create request: {:#?}", error)),
    };
    match client.request(request).await {
        Ok(response) => Ok(response.into_parts()),
        Err(error) => Err(format!("Failed to craete request: {:#?}", error)),
    }
}
