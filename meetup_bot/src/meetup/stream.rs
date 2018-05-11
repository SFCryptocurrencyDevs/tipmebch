use reqwest;
use serde_json;
use std;
use std::collections::HashMap;
use std::env;
use ws::{connect, Handler, Message, Result};

struct Client;

impl Handler for Client {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        deserialize_string(&msg).expect("");
        Ok(())
    }
}

// Connect to the Meetup API ws
pub fn connect_to_stream() {
    let base = "ws://stream.meetup.com/2/event_comments";
    let url = format!(
        "{}?event_id={}",
        base,
        &env::var("MEETUP_EVENT_ID").unwrap()
    );

    connect(url, |_out| Client {}).unwrap()
}

// Deserialize the received data
fn deserialize_string(msg: &Message) -> std::result::Result<(), serde_json::Error> {
    // Get the json string from the message
    let thing: &str = match msg.as_text() {
        Ok(res) => res,
        _ => "",
    };

    // Use serde json to parse the json from the response
    let v: serde_json::Value = serde_json::from_str(thing)?;
    let messenger_id = &v["id"];
    let bot_id = &env::var("MEETUP_EVENT_ID").unwrap();

    // Check that the messenger id isn't bot.
    // This avoids the infinite loop scenario
    // where the bot responds to all its own
    // messages.
    if messenger_id != bot_id {
        respond(&v["id"], &v["comment"]).expect("");
    }

    // TODO: this does not need to return anything
    Ok(())
}

// Ingest the response and respond accordingly
fn respond(
    id: &serde_json::Value,
    comment: &serde_json::Value,
) -> std::result::Result<(), reqwest::Error> {
    let words = &comment.as_str().unwrap();
    let split_word: Vec<&str> = words.split(' ').collect();
    let command = split_word[0];

    // The nicest code here (WOW RUST IS AWESOME AM I RITE?)
    // For new contributers, this is where you add new commands.
    let response = match command {
        "/help" => format!(
            "Welcome to the Stellar Meetup Bot!\n\n
                            Commands:\n
                            /help: how I do dis?\n
                            /about: who, what, when, where of this bot\n
                            /price: get the current XLM price\n"
        ),
        "/about" => format!(
            "Welcome to the Stellar Meetup Bot!\n\n
                            Created by Rob Durst @ 2018 for SF Cryptocurrency Devs\n
                            Send XLM from one to another seemlessly via Meetup\n
                            Why... why not?\n"
        ),
        "/deposit" => {
            // TODO: deal with errors
            let data = get_new_memo().unwrap();
            format!(
                "Send a transaction to {} with memo: {}",
                &env::var("BOT_STELLAR_ADDRESS").unwrap(),
                data
            )
        }
        "/price" => {
            if split_word.len() > 1 {
                let data = get_crypto_price(split_word[1]).expect("No error from cryptocompare");
                match data {
                    Some(price) => format!("The current price is ${:?}", price),
                    _ => format!("Curency not found!"),
                }
            } else {
                format!("Currency not specified.")
            }
        }
        _ => format!("err"),
    };

    if response != "err" {
        // Create a hashmap to store the data we want to
        // send in the http request.
        let event_id = env::var("MEETUP_EVENT_ID").unwrap();
        let id_here = id.as_i64().unwrap().to_string();
        let mut map = HashMap::new();
        map.insert("comment", response);
        map.insert("event_id", event_id);
        map.insert("in_reply_to", id_here);

        let client = reqwest::Client::new();

        let base = "https://api.meetup.com/2/event_comment";
        let url = &format!("{}?key={}", base, &env::var("MEETUP_API_KEY").unwrap());

        client.post(url).form(&map).send()?;
    }

    // TODO: this does not need to return anything
    Ok(())
}

pub fn get_new_memo() -> std::result::Result<(String), reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client.get("http://localhost:8000/gen_memo").send()?.text()?;

    Ok(resp)
}

// TODO: figure out how to deal with errors correctly
// TODO: is f64 the correct return val?
pub fn get_crypto_price(
    ticker: &str,
) -> std::result::Result<std::option::Option<f64>, reqwest::Error> {
    let url = &format!(
        "https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD",
        ticker.to_uppercase()
    );

    let client = reqwest::Client::new();
    let resp = client.get(url).send()?.text()?;

    let v: serde_json::Value = serde_json::from_str(&resp).expect("Should have a value.");

    Ok(v["USD"].as_f64())
}
