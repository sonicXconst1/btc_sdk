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

    pub fn new(
        client: std::sync::Arc<hyper::Client<TConnector>>,
        base_url: url::Url,
    ) -> PublicClient<TConnector> {
        PublicClient { client, base_url }
    }

    pub async fn get_all_symbols(&self) -> Option<models::Symbols> {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect(client::BAD_URL)
            .push(Self::PUBLIC)
            .push(Self::SYMBOL);
        let request = hyper::Request::builder()
            .header("Accept", "application/json")
            .uri(url.to_string())
            .method(http::Method::GET)
            .body(hyper::Body::empty())
            .expect("Failed to build request!");
        let response = self.client.request(request).await.unwrap();
        log::info!("Get all symbols: {:#?}", response);
        let body = response.into_body();
        extractor::extract_symbols(body).await
    }

    pub async fn get_orderbook(
        &self,
        limit: Option<u64>,
        symbols: Option<Vec<coin::Symbol>>,
    ) -> Option<models::OrderBook> {
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
        let request = hyper::Request::builder()
            .header("Accept", "application/json")
            .uri(url.to_string())
            .method(http::Method::GET)
            .body(hyper::Body::empty())
            .expect("Failed to build request!");
        let (header, body) = self.client.request(request).await.unwrap().into_parts();
        log::info!("Header: {:#?}", header);
        extractor::extract_orderbook(body).await
    }

    pub async fn get_symbol_from_orderbook(
        &self,
        symbol: coin::Symbol,
        limit: Option<u64>,
        volume: Option<f64>,
    ) -> Option<models::OrderbookExactSymbol> {
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
        let request = hyper::Request::builder()
            .header("Accept", "application/json")
            .uri(url.to_string())
            .method(http::Method::GET)
            .body(hyper::Body::empty())
            .expect("Failed to build request!");
        let (header, body) = self.client.request(request).await.unwrap().into_parts();
        log::info!("Header: {:#?}", header);
        extractor::extract_orderbook_exact_symbol(body).await
    }
}
