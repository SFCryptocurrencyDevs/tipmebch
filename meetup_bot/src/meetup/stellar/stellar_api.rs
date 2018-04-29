// Test account
// GCNCCXU2UI7D6KVSOEM7OY3KDD5E3VKST3XSCH2A22BEGZALC22GKTH7
// SCDQ3EN54H4KQ5O4QO3MUFIYIJK5IYHL6Y7VYYSTBKW63QS5QZ7VMS5T

use stellar_client::sync::{Client};
use stellar_client::endpoint::{account, Limit, Order, Direction, Cursor};
use std::time::Duration;
use futures::prelude::*;
use futures_timer::Interval;
use std::str::FromStr;

static mut CURRENT_CURSOR: i64 = 0;

pub fn poll() {
    Interval::new(Duration::from_secs(1))
      .for_each(|()| Ok(get_payments()))
      .wait()
      .unwrap();
}

pub fn get_payments() {
    // Establish whether connecting to testnet or mainnet
    let client = Client::horizon_test().unwrap();

    // TODO: remove unsafe code
    unsafe {
        let endpoint;
        // If this is the first time, get latest cursor (ie order=desc and latest is top)
        if CURRENT_CURSOR == 0 {
            endpoint = account::Payments::new("GCNCCXU2UI7D6KVSOEM7OY3KDD5E3VKST3XSCH2A22BEGZALC22GKTH7").with_limit(200).with_order(Direction::Desc).with_cursor("now");
        } else {
            endpoint = account::Payments::new("GCNCCXU2UI7D6KVSOEM7OY3KDD5E3VKST3XSCH2A22BEGZALC22GKTH7").with_limit(200).with_order(Direction::Asc).with_cursor(&CURRENT_CURSOR.to_string());
        }

        let payments = client.request(endpoint).unwrap();

        if payments.records().len() > 0 {
            let last_cursor = payments.records()[0].paging_token();
            CURRENT_CURSOR = FromStr::from_str(last_cursor).unwrap();
        }
    }
}