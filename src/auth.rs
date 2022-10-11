use std::borrow::Cow;

use hyper::Method;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::common::*;
use crate::{
    error::{self, Result},
    links,
};

pub(crate) mod raw;

use raw::RequestBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub key: Cow<'static, str>,
    pub secret: Cow<'static, str>,
}

impl KeyPair {

    pub fn new<K, S>(key: K, secret: S) -> KeyPair 
    where
        K: Into<Cow<'static, str>>,
        S: Into<Cow<'static, str>>,
    {
        KeyPair {
            key: key.into(),
            secret: secret.into(),
        }
    }
    
    fn empty() -> KeyPair {
        KeyPair {
            key: "".into(),
            secret: "".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Token {
    Access {
        consumer: KeyPair,
        access: KeyPair,
    },
    Bearer(String),
}

pub async fn request_token<S: Into<String>>(con_token: &KeyPair, callback: S) -> Result<KeyPair> {
    let request = RequestBuilder::new(Method::POST, links::auth::REQUEST_TOKEN)
        .oauth_callback(callback.into())
        .request_keys(con_token, None);

    let (_, body) = raw_request(request).await?;

    let body = std::str::from_utf8(&body).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "stream did not contain valid UTF-8",
        )
    })?;
    let mut key: Option<String> = None;
    let mut secret: Option<String> = None;

    for elem in body.split('&') {
        let mut kv = elem.splitn(2, '=');
        match kv.next() {
            Some("oauth_token") => key = kv.next().map(|s| s.to_string()),
            Some("oauth_token_secret") => secret = kv.next().map(|s| s.to_string())
            Some(_) => (),
            None => {
                return Err(error::Error::InvalidResponse(
                    "unexpected end of request_tokeen response",
                    None,
                ))
            }
        }
    }

    Ok(KeyPair::new(
        key.ok_or(error::Error::MissiongValue("oauth_token"))?,
        secret.ok_or(error::Error::MissingValue("oauth_token_secret"))?,
    ))

}

pub fn authorize_url(request_token: &KeyPair) -> String {
    format!(
        "{}?oauth_token={}",
        links::auth::AUTHORIZE,
        request_token.key
    )
}

pub fn authenticate_url(request_token: &KeyPair) -> String {
    format!(
        "{}?oauth_token={}",
        links::auth::AUTHENTICATE,
        request_token.key
    )
}

