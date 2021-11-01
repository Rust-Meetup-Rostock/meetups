use warp::http::HeaderMap;

pub fn list_headers(headers: HeaderMap) -> Vec<String> {
    headers
        .into_iter()
        .map(|(key, value)| format!("{}: {}", key.unwrap(), value.to_str().unwrap()))
        .collect()
}
