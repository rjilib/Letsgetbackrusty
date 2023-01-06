use serde::Deserialize;
use reqwest::{Error, header};

use crate::serialize_own_vec::SerdeVec;

const URL_COINMARKETCAP_NEWS: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";
const URL_COINMARKETCAP_CONTRACT: &str = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/map";

pub enum COINMARKETCAP_ERROR {
    FetchNewsError,
    FetchAddressError,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub data: Vec<Data>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Data {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub cmc_rank: i64,
    pub quote: Quote,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Quote {
    #[serde(rename = "USD")]
    pub usd: USD,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct USD {
    pub price: f64,
    pub volume_24h: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub market_cap: f64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub data: Vec<Daum>
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Daum {
    pub rank: Option<i64>,
    pub name: String,
    pub symbol: String,
    pub platform: Option<Platform>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Platform {
    pub slug: String,
    pub token_address: String,
}


fn build_header(content: &str, key: &str) -> Result<reqwest::Client, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Accepts", content.parse().unwrap());
    headers.insert("X-CMC_PRO_API_KEY", key.parse().unwrap());
    let client = reqwest::Client::builder()        
        .default_headers(headers)
        .build()?;
    Ok(client)
}

pub async fn fetch_news_coinmarketcap(content: &str, key: &str) -> Result<Symbol, reqwest::Error> {
    let response = match build_header(content, key).unwrap()
            .get(URL_COINMARKETCAP_NEWS)
            .query(&[("sort", "percent_change_1h")])
            .query(&[("limit", "10")])
            .send().await?
            .json().await {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    Ok(response)
}

pub async fn fetch_address_crypto(content: &str, key: &str, list: &SerdeVec) -> Result<Address, reqwest::Error> {   
    let response = match build_header(content, key).unwrap()
            .get(URL_COINMARKETCAP_CONTRACT)
            .query(&[("symbol", convert_vec_to_str(&list))])
            .send().await?
            .json().await {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    Ok(response)
}

fn convert_vec_to_str(list: &SerdeVec) -> String {
    format!("{}", &list.list.join(","))
}