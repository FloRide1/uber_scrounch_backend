use super::{ApiItem, ApiRoot};

pub fn get_client(api_key: &str) -> Option<reqwest::Client>{
    let mut headers = reqwest::header::HeaderMap::new();
    
    let api_key = reqwest::header::HeaderValue::from_str(api_key).ok()?;
    headers.insert("api-key", api_key);

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .ok()
}

pub async fn get_item(client: &reqwest::Client, id: u32) -> Option<ApiItem> {
    let base_url = std::env::var("API_BASE_URL").unwrap();
    let url = format!("{base_url}/products?&start={id}&include=category");
    let res: ApiRoot = client.get(url)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    Some(res.data[0].clone())
}

pub async fn get_items(client: &reqwest::Client, id_start: u32, _len: u32) -> Option<Vec<ApiItem>> {
    let base_url = std::env::var("API_BASE_URL").unwrap();
    let url = format!("{base_url}/products?start={id_start}&include=category");
    let res: ApiRoot = client.get(url)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    Some(res.data.clone())
}

pub async fn get_all_items(client: &reqwest::Client) -> Option<Vec<ApiItem>> {
    // let base_url = std::env::var("API_BASE_URL").unwrap();
    // let url = format!("{base_url}/products?limit=1&start=1&include=category");

    let res = (0..300).map(|x| get_items(client, x as u32, 20));
    Some(futures::future::join_all(res).await.iter().filter_map(|x| x.clone()).flatten().collect())
}
