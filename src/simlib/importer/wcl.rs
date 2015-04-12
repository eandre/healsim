extern crate hyper;
use std::ptr;
use event;

const BASE_URL: &'static str = "https://www.warcraftlogs.com";

pub struct WCLIterator {
    report_id: String,
    fight_id: u32,
    client: hyper::Client,
    buf: Vec<event::Event>,
    buf_idx: usize,
    segment_id: u32,
    err: bool,
}

impl WCLIterator {
    fn new(report_id: String, fight_id: u32) -> WCLIterator {
        let client = hyper::Client::new();

        WCLIterator{
            report_id: report_id,
            fight_id: fight_id,
            client: client,
            buf: vec![],
            buf_idx: 0,
            segment_id: 0,
            err: false,
        }
    }
}

impl WCLIterator {
    fn fetch_segment(&mut self) -> Result<bool, String> {
        if self.segment_id == 0 {
            println!("Fetching summary data...");
            let path = format!("{}/reports/fights_and_participants/{}", BASE_URL, self.report_id.as_str());
            let result = self.get_url(path.as_str());
            println!("Got result {:?}", result);
        }
        Err(String::from_str("Could not fetch!"))
    }

    fn get_url(&mut self, url: &str) -> hyper::HttpResult<hyper::client::Response> {
        self.client.get(url).send()
    }
}

impl Iterator for WCLIterator {
    type Item = event::Event;

    fn next(&mut self) -> Option<event::Event> {
        if self.err {
            return None
        }

        if self.buf_idx >= self.buf.len() {
            // Need a new buf
            match self.fetch_segment() {
                Ok(true) => {},
                Ok(false) => return None,
                Err(s) => {
                    self.err = true;
                    return Some(event::Event{name: format!("Could not fetch segment: {}", s)})
                }
            }
        }

        // Get the event at the current index without borrowing.
        // This is safe since we never read the same index twice.
        let result = unsafe {
            let p = self.buf.as_ptr().offset(self.buf_idx as isize);
            Some(ptr::read(p))
        };
        self.buf_idx += 1;
        result
    }
}

pub fn import(report_id: String, fight_id: u32) -> WCLIterator {
    WCLIterator::new(report_id, fight_id)
}
