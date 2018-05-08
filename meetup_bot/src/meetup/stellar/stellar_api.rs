use stellar_client::sync::{Client};
use stellar_client::endpoint::{account, Limit, Order, Direction, Cursor};
use stellar_client::resources::Memo;
use std::str::FromStr;
use std::{thread, time};
use std::collections::HashMap;

pub fn init_poll() {
    let mut poll = Poll::new();
    poll.add_memo("hello", 10);
    
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        poll.get_payments();
    }
}

struct Poll {
    cursor: i64,
    filter: Filter,
}

impl Poll {
    fn new() -> Poll {
        Poll {
            cursor: 0,
            filter: Filter::new(),
        }
    }

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
                if let &Memo::Text(ref value) = node.memo() {
                    if self.filter.memo_exists(value) {
                        self.filter.remove_memo(value);
                        println!("🔥 Received transaction with memo: {} 🔥", value);
                    } else {
                        println!("Received extraneous transaction with memo: {}", value);
                    }
                    
                }           
            }
        }
    }

    fn add_memo(&mut self, memo: &str, amount: i64) {
        self.filter.add_memo(memo, amount);
    }
}

struct Filter {
    memo_map: HashMap<String, i64>,
}

impl Filter {
    fn new() -> Filter {
        Filter {memo_map: HashMap::new()}
    }
    
    fn add_memo(&mut self, memo: &str, amount: i64) {
        &self.memo_map.insert(memo.to_string(), amount);
    }

    fn remove_memo(&mut self, memo: &str) {
        &self.memo_map.remove(memo);
    }
    
    fn memo_exists(&self, memo: &str) -> bool {
        self.memo_map.contains_key(memo)
    }
}

