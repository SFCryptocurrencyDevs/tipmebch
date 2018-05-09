use stellar_client::sync::{Client};
use stellar_client::endpoint::{account, Limit, Order, Direction, Cursor};
use stellar_client::resources::Memo;
use std::str::FromStr;
use std::{thread, time};

#[derive(Debug)]
pub struct Poll {
    cursor: i64,
    memo: String,
}

impl Poll {
    pub fn new(memo: String) -> Poll {
        Poll {
            cursor: 0,
            memo,
        }
    }

    // Initalize a new Poll and a new listening thread
    pub fn init(memo: String) {
        // TODO: close/destroy thread?
        // TODO: is this dropping correct?
        // TODO: end the thread in case the deposit "times out"
         thread::spawn(move|| {
             let mut poll = Poll::new(memo.to_owned());
            loop {
                thread::sleep(time::Duration::from_millis(1000));
                // Once the memo has been received, it will break
                // out of the loop.
                let is_done = poll.get_payments();
                if is_done {
                    break;
                }
            }
            println!("Exiting thread");
            drop(memo);
            drop(poll);
        });
    }

    fn get_payments(&mut self) -> bool {
        
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
                    if &self.memo == value {
                        println!("🔥 Received transaction with memo: {} 🔥", value);
                        // Update the balance of this person's account in postgres
                        return true
                    }
                }           
            }
        }

        false
    }
}

