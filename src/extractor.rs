use super::models;

pub async fn extract_balance(body: hyper::Body) -> Option<models::Balance> {
    read_body(body).await
}

async fn read_body<TResult>(body: hyper::Body) -> Option<TResult> 
where TResult: serde::de::DeserializeOwned
{
    let bytes = hyper::body::to_bytes(body).await
        .expect("Failed to convert body to bytes");
    match serde_json::from_slice(&bytes) {
        Ok(result) => Some(result),
        Err(error) => {
            log::error!("Error on reading the body: {:#?}", error);
            None
        }
    }
}
