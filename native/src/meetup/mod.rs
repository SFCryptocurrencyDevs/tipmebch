use std::env;
use reqwest;

/// Really bad rust code -- this is just the beginning :)
pub fn get_all_events() -> Result<String, reqwest::Error> {
    let base = "https://api.meetup.com/SF-Cryptocurrency-Devs/events";
    let url = &format!("{}?key={}", base, &env::var("MEETUP_API_KEY").unwrap());
    let resp = reqwest::get(url)?.text()?;

    return Ok(resp);
}