use super::api_url;
use super::prelude::*;
use reqwest;
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json::{self, Value};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct Account {
    pub user: String,
    pub password: String,
}

pub async fn login(account: &Account, client: reqwest::Client) -> Result<Auth, Box<dyn std::error::Error>> {
    login_oauth2(client.clone(), account).await?;

    let oauth2_callback = oauth2_callback(client.clone()).await?;

    let login_callback = login_by_callback(client.clone(), oauth2_callback).await?;

    let session_id = login_callback.1.get("data").unwrap().as_str().unwrap().to_string();

    let authorization = login_callback
        .0
        .get("authorization")
        .unwrap()
        .to_str()?
        .to_string();
    let auth = Auth {
        session_id: session_id,
        authorization: authorization,
    };
    return Ok(auth);
}

pub async fn login_oauth2(
    client: reqwest::Client,
    account: &Account,
) -> Result<HashMap<String,Value>, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    let response = request_post(client.clone(), api_url::OAUTH2_URL, headers, &account).await?;
    let json = get_json_by_response(response).await?;
    if json.get("flag").unwrap() != true{
        return Err(Box::new(Error{
            message: "Failed to request".to_string(),
            kind: "Failed".to_string(),
        }));
    }
    Ok(json)
}

pub async fn oauth2_callback(
    client: reqwest::Client,
) -> Result<HashMap<String,Value>, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let response = request_post(client.clone(), api_url::OAUTH2_CALLBACK, headers, "").await?;
    let json = get_json_by_response(response).await?;
    if json.get("flag").unwrap() != true{
        return Err(Box::new(Error{
            message: "Failed to request".to_string(),
            kind: "Failed".to_string(),
        }));
    }
    Ok(json)
}

pub async fn login_by_callback(
    client: reqwest::Client,
    oauth2_callback: HashMap<String, Value>,
) -> Result<(HeaderMap, HashMap<String, Value>), Box<dyn std::error::Error>> {
    let headers = HeaderMap::new();
    let code = oauth2_callback
        .get("data")
        .unwrap()
        .get("code")
        .unwrap()
        .as_str()
        .unwrap();
    let url = format!("{0}{1}", api_url::LOGIN_CALLBACK, code);
    let response = request_post(client.clone(), url.as_str(), headers, "").await?;
    let headers = get_headers_by_respone(&response);
    let json = get_json_by_response(response).await?;
    if json.get("flag").unwrap() != true{
        return Err(Box::new(Error{
            message: "Failed to request".to_string(),
            kind: "Failed".to_string(),
        }));
    }
    Ok((headers, json))
}
