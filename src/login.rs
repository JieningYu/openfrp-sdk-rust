use crate::Auth;
use crate::Error;

use super::api_url;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct Account {
    pub user: String,
    pub password: String,
}

pub async fn login(account: &Account, client: reqwest::Client) -> crate::Result<Auth> {
    login_oauth2(client.clone(), account).await?;

    let oauth2_callback = oauth2_callback(client.clone()).await?;
    let login_callback = login_by_callback(client.clone(), oauth2_callback).await?;

    let session_id = login_callback
        .1
        .get("data")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let authorization = login_callback
        .0
        .get("authorization")
        .unwrap()
        .to_str()?
        .to_string();

    Ok(Auth {
        session_id,
        authorization,
    })
}

pub async fn login_oauth2(client: reqwest::Client, account: &Account) -> crate::Result<()> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    let response = req!(client.clone(), api_url::OAUTH2_URL, headers => &account)?;

    #[derive(Debug, Deserialize)]
    struct Res {
        flag: bool,
    }

    if response.json::<Res>().await?.flag {
        Ok(())
    } else {
        Err(Error::OAuth2)
    }
}

pub async fn oauth2_callback(
    client: reqwest::Client,
) -> crate::Result<HashMap<String, serde_json::Value>> {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let response = req!(client.clone(), api_url::OAUTH2_CALLBACK, headers => "")?;
    let json = response
        .json::<HashMap<String, serde_json::Value>>()
        .await?;
    if json.get("flag").unwrap().as_bool().unwrap_or_default() {
        Ok(json)
    } else {
        Err(Error::OAuth2)
    }
}

pub async fn login_by_callback(
    client: reqwest::Client,
    oauth2_callback: HashMap<String, serde_json::Value>,
) -> crate::Result<(HeaderMap, HashMap<String, serde_json::Value>)> {
    let headers = HeaderMap::new();
    let code = oauth2_callback
        .get("data")
        .unwrap()
        .get("code")
        .unwrap()
        .as_str()
        .unwrap();
    let url = format!("{0}{1}", api_url::LOGIN_CALLBACK, code);
    let response = req!(client.clone(), url.as_str(), headers => "")?;
    let headers = response.headers().to_owned();
    let json = response
        .json::<HashMap<String, serde_json::Value>>()
        .await?;
    if json.get("flag").unwrap().as_bool().unwrap_or_default() {
        Ok((headers, json))
    } else {
        Err(Error::OAuth2)
    }
}
