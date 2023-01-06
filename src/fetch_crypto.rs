use serde::Deserialize;
use reqwest::Error;

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Crypto {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub last_price: String,
    pub volume: String,
    pub quote_volume: String,
    pub open_time: i64,
    pub close_time: i64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: i64,
}

fn binance_url(first_symbol: &str, second_symbol: &str) -> String {
    format!("https://api.binance.com/api/v3/ticker?symbol={}{}", first_symbol, second_symbol)
}

pub async fn fecth_crypto_symbol(first_symbol: &str, second_symbol: &str) -> Result<Crypto, Error> {
    let result = reqwest::get(&binance_url(first_symbol, second_symbol))
            .await?
            .json()
            .await?;
        Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binance_url() {
        assert_eq!(binance_url("BNB", "USDT"), format!("https://api.binance.com/api/v3/ticker?symbol=BNBUSDT"));
        assert_eq!(binance_url("FTM", "USDT"), format!("https://api.binance.com/api/v3/ticker?symbol=FTMUSDT"));
    }
}
