use stellar_client::sync::{Client};
use stellar_client::endpoint::{account, Limit, Order, Direction, Cursor};
use std::str::FromStr;
use std::{thread, time};

pub fn init_poll() {
    let mut poll = Poll{ cursor: 0};
    
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        poll.get_payments();
    }
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
            endpoint = account::Transactions::new("GCJY6RHN3SOUKBZXTDNWEJUSOH5PY7GV5Q44OK4BGKYHRM7EE5FXVHW7").with_limit(200).with_order(Direction::Desc).with_cursor("now");
        } else {
            endpoint = account::Transactions::new("GCJY6RHN3SOUKBZXTDNWEJUSOH5PY7GV5Q44OK4BGKYHRM7EE5FXVHW7").with_limit(200).with_order(Direction::Asc).with_cursor(&self.cursor.to_string());
        }

        let transactions = client.request(endpoint).unwrap();
        
        // If there are new records, update last cursor and parse for new deposits
        if transactions.records().len() > 0 {

            // Update last cursor
            let last_cursor = transactions.records()[0].paging_token();
            self.cursor = FromStr::from_str(last_cursor).unwrap();

            for node in transactions.records().iter() {
                let memo = node.memo();
                println!("{:?}", memo);
            }
        }
    }
}

