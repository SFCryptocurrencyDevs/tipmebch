use std::env;
use std;
use serde_json;
use ws::{connect, Handler, Handshake, Message, Result, Sender};
use reqwest;
use std::collections::HashMap;

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("Hello WebSocket")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // The below code is garbage, I should deal
        // with the response in a better way
        match deserialize_string(&msg) {
            Ok(()) => Ok(()),
            _ => Ok(()),
        }
    }
}

// TODO: close stream at some point
// Connect to the Meetup API ws
pub fn connect_to_stream() {
    let base = "ws://stream.meetup.com/2/event_comments";
    let url = format!(
        "{}?event_id={}",
        base,
        &env::var("MEETUP_EVENT_ID").unwrap()
    );

    connect(url, |out| Client { out: out }).unwrap()
}

// Deserialize the received data
fn deserialize_string(msg: &Message) -> std::result::Result<(), serde_json::Error> {
    let thing: &str = match msg.as_text() {
        Ok(res) => res,
        _ => "",
    };

    let v: serde_json::Value = serde_json::from_str(thing)?;
    let messenger_id = &v["id"];
    let bot_id = &env::var("MEETUP_EVENT_ID").unwrap();
    if messenger_id != bot_id {
        // The below code is garbage, I should deal
        // with the response in a better way
        match respond(&v["id"], &v["comment"]) {
            Ok(()) => (),
            _ => (),
        };
    }

    Ok(())
}

// Ingest the response and respond accordingly
// Really bad rust code -- this is just the beginning :)
pub fn respond(
    id: &serde_json::Value,
    comment: &serde_json::Value,
) -> std::result::Result<(), reqwest::Error> {
    let words = &comment.as_str().unwrap();
    let split_word: Vec<&str> = words.split(' ').collect();
    let command = split_word[0];

    // By far the nicest code in this rat nest
    let response = match command {
        "/help" => format!("he wants help"),
        "/about" => format!("he wants to know about"),
        "/price" => format!(
            "The current price of XLM is ${:?}",
            get_xlm_price().unwrap()
        ),
        _ => format!("err"),
    };

    if response != "err" {
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

    Ok(())
}

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
