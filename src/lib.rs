macro_rules! req {
    ($c:expr, $u:expr, $h:expr => $j:expr) => {
        $c.post($u).headers($h).json(&$j).send().await
    };
    ($c:expr, $u:expr, $h:expr) => {
        $c.get($u).headers($h).send().await
    };
}

pub mod api_url;
pub mod login;

pub use login::{login, Account};

use std::collections::HashMap;

use reqwest::redirect::Policy;
use serde::Serialize;

pub fn client() -> reqwest::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .redirect(Policy::limited(2))
        .cookie_store(true)
        .build()?
        .clone())
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Auth {
    pub session_id: String,
    pub authorization: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(reqwest::Error),
    #[error("login with oauth2 failed")]
    OAuth2,
    #[error("http header to str error: {0}")]
    HttpHeaderToStr(reqwest::header::ToStrError),
}

macro_rules! err_from {
    ($t:ty => $v:ident) => {
        impl From<$t> for Error {
            #[inline]
            fn from(value: $t) -> Self {
                Self::$v(value)
            }
        }
    };
}

err_from!(reqwest::Error => Reqwest);
err_from!(reqwest::header::ToStrError => HttpHeaderToStr);

pub type Result<T> = std::result::Result<T, Error>;

pub async fn sign(
    auth: &Auth,
    client: reqwest::Client,
) -> Result<HashMap<String, serde_json::Value>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());
    let json = serde_json::json!({
        "session": auth.session_id,
    });
    let response = req!(client, api_url::SIGN_API, headers=> &json)?;
    response
        .json::<HashMap<String, serde_json::Value>>()
        .await
        .map_err(From::from)
}

pub async fn user_proxies(
    auth: &Auth,
    client: reqwest::Client,
) -> Result<HashMap<String, serde_json::Value>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());

    #[derive(Debug, Serialize)]
    struct SessionReq<'a> {
        session: &'a str,
    }

    let response = req!(
        client,
        api_url::GET_USER_PROXIES,
        headers =>
        &SessionReq {
            session: &auth.session_id,
        }
    )?;
    response
        .json::<HashMap<String, serde_json::Value>>()
        .await
        .map_err(From::from)
}

pub async fn user_info(
    auth: &Auth,
    client: reqwest::Client,
) -> reqwest::Result<HashMap<String, serde_json::Value>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());
    let json = serde_json::json!({
        "session": auth.session_id,
    });
    let response = req!(client, api_url::GET_USER_INFO, headers => &json)?;
    response
        .json::<HashMap<String, serde_json::Value>>()
        .await
        .map_err(From::from)
}

pub async fn node_list(
    auth: &Auth,
    client: reqwest::Client,
) -> reqwest::Result<HashMap<String, serde_json::Value>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("authorization", auth.authorization.parse().unwrap());
    let json = serde_json::json!({
        "session": auth.session_id,
    });
    let response = req!(client, api_url::GET_NODE_LIST, headers => &json)?;
    response
        .json::<HashMap<String, serde_json::Value>>()
        .await
        .map_err(From::from)
}
