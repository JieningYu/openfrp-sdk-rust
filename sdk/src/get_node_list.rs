use std::collections::HashMap;

use serde_json::Value;

use super::api_url;
use super::prelude::*;

#[tokio::main]
pub async fn get_node_list(auth: &Auth, client: reqwest::Client) -> reqwest::Result<HashMap<String,Value>>{
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());
    let json = serde_json::json!({
        "session": auth.session_id,
    });
    let response = request_post(client, api_url::GET_NODE_LIST, headers, &json).await?;
    let json = get_json_by_response(response).await?;
    Ok(json)
}