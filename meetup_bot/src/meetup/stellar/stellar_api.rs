use stellar_client::sync::{Client};
use stellar_client::endpoint::{account, Limit, Order, Direction, Cursor};
use std::time::Duration;
use futures::prelude::*;
use futures_timer::Interval;
use std::str::FromStr;

pub fn init_poll() {
    let mut poll = Poll{ cursor: 0};
    Interval::new(Duration::from_secs(1))
      .for_each(|()| Ok(poll.get_payments()))
      .wait()
      .unwrap();
}

struct Poll {
    cursor: i64,
}

impl Poll {
    fn get_payments(&mut self) {
        
    // Establish whether connecting to testnet or mainnet
    let client = Client::horizon_test().unwrap();

        let endpoint;
        // If this is the first time, get latest cursor (ie order=desc and latest is top)
        if self.cursor == 0 {
            endpoint = account::Payments::new("GCJY6RHN3SOUKBZXTDNWEJUSOH5PY7GV5Q44OK4BGKYHRM7EE5FXVHW7").with_limit(200).with_order(Direction::Desc).with_cursor("now");
        } else {
            endpoint = account::Payments::new("GCJY6RHN3SOUKBZXTDNWEJUSOH5PY7GV5Q44OK4BGKYHRM7EE5FXVHW7").with_limit(200).with_order(Direction::Asc).with_cursor(&self.cursor.to_string());
        }

        let payments = client.request(endpoint).unwrap();
        
        println!("{:?}", payments.records());

        if payments.records().len() > 0 {
            let last_cursor = payments.records()[0].paging_token();
            self.cursor = FromStr::from_str(last_cursor).unwrap();
        }
    }
}

