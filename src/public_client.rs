use super::client;
use super::coin;
use super::extractor;
use super::models;

pub struct PublicClient<TConnector> {
    client: std::sync::Arc<hyper::Client<TConnector>>,
    base_url: url::Url,
}

impl<TConnector> PublicClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    const PUBLIC: &'static str = "public";
    const SYMBOL: &'static str = "symbol";
    const ORDERBOOK: &'static str = "orderbook";
    const CURRENCY: &'static str = "currency";

    pub fn new(
        client: std::sync::Arc<hyper::Client<TConnector>>,
        base_url: url::Url,
    ) -> PublicClient<TConnector> {
        PublicClient { client, base_url }
    }

    pub async fn get_all_currencies(&self) -> Result<Vec<models::PublicCurrency>, String> {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect(client::BAD_URL)
            .push(Self::PUBLIC)
            .push(Self::CURRENCY);
        let (_header, body) = default_request(&self.client, url).await?;
        match extractor::extract_currencies(body).await {
            Some(currencies) => Ok(currencies),
            None => Err("Deserialization error".to_owned()),
        }
    }

    pub async fn get_all_symbols(&self) -> Result<models::Symbols, String> {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect(client::BAD_URL)
            .push(Self::PUBLIC)
            .push(Self::SYMBOL);
        let (_header, body) = default_request(&self.client, url).await?;
        match extractor::extract_symbols(body).await {
            Some(symbols) => Ok(symbols),
            None => Err("Deserialization error".to_owned()),
        }
    }

    pub async fn get_orderbook(
        &self,
        limit: Option<u64>,
        symbols: Option<Vec<coin::Symbol>>,
    ) -> Result<models::OrderBook, String> {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect(client::BAD_URL)
            .push(Self::PUBLIC)
            .push(Self::ORDERBOOK);
        if let Some(limit) = limit {
            url.query_pairs_mut()
                .append_pair("limit", &format!("{}", limit));
        }
        if let Some(symbols) = symbols {
            let mut symbols =
                symbols
                    .into_iter()
                    .fold(String::with_capacity(100), |mut accumulator, symbol| {
                        accumulator.push_str(&symbol.to_string());
                        accumulator.push(',');
                        accumulator
                    });
            if symbols.len() > 0 {
                let comma = symbols.pop();
                assert_eq!(comma, Some(','));
            }
            url.query_pairs_mut().append_pair("symbols", &symbols);
        }
        let (_header, body) = default_request(&self.client, url).await?;
        match extractor::extract_orderbook(body).await {
            Some(orderbook) => Ok(orderbook),
            None => Err("Deserialization error".to_owned()),
        }
    }

    pub async fn get_symbol_from_orderbook(
        &self,
        symbol: coin::Symbol,
        limit: Option<u64>,
        volume: Option<f64>,
    ) -> Result<models::OrderbookExactSymbol, String> {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect(client::BAD_URL)
            .push(Self::PUBLIC)
            .push(Self::ORDERBOOK)
            .push(&symbol.to_string());
        if let Some(volume) = volume {
            url.query_pairs_mut()
                .append_pair("volume", &format!("{}", volume));
        } else if let Some(limit) = limit {
            url.query_pairs_mut()
                .append_pair("limit", &format!("{}", limit));
        }
        let (_header, body) = default_request(&self.client, url).await?;
        match extractor::extract_orderbook_exact_symbol(body).await {
            Some(orderbook) => Ok(orderbook),
            None => Err("Deserialization error".to_owned()),
        }
    }
}

async fn default_request<TConnector>(
    client: &hyper::Client<TConnector>,
    url: url::Url,
) -> Result<(http::response::Parts, hyper::Body), String>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    let request = match hyper::Request::builder()
        .header("Accept", "application/json")
        .uri(url.to_string())
        .method(http::Method::GET)
        .body(hyper::Body::empty())
    {
        Ok(request) => request,
        Err(error) => return Err(format!("Request failed: {:#?}", error)),
    };
    match client.request(request).await {
        Ok(response) => Ok(response.into_parts()),
        Err(error) => Err(format!("Failed to craete request: {:#?}", error)),
    }
}
