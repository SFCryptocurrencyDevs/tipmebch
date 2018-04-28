use std;
use serde_json;
use reqwest;

// TODO: figure out how to deal with errors correctly
// TODO: is f64 the correct return val?
pub fn get_xlm_price() -> std::result::Result<(f64), reqwest::Error> {
    let url = "https://min-api.cryptocompare.com/data/price?fsym=XLM&tsyms=USD";

    let client = reqwest::Client::new();
    let resp = client.get(url).send()?.text()?;

    let v: serde_json::Value = serde_json::from_str(&resp).expect("Should have a value.");
    let price = v["USD"].as_f64().unwrap();
    Ok(price)
}