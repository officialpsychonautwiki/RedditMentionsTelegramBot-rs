extern crate eventsource;
extern crate hyper;
extern crate regex;
extern crate telegram_bot;
extern crate json;

use hyper::Url;
use regex::Regex;

fn main() {
    let re = [
        Regex::new(r"psychonaut\s?wiki").unwrap(),
        Regex::new(r"psychonaut.?wiki").unwrap(),
        Regex::new(r"disregard\s?everything\s?i\s?say").unwrap()
    ];

    let api = telegram_bot::Api::from_env("TELEGRAM_TOKEN").unwrap();

    let url = Url::parse("http://stream.pushshift.io").unwrap();
    let client = eventsource::Client::new(url);

    for event in client {
        let uw_event = event.unwrap();

        if re.iter().any(|ref exp| exp.is_match(&uw_event.data)) {
            let evt = json::parse(&uw_event.data).unwrap();

            let msg = format!("User {:?} wrote into subreddit {:?} with title {:?}:\n\n{:?}", evt["author"].to_string(), evt["subreddit_id"].to_string(), evt["link_title"].to_string(), evt["body"].to_string());

            let _ = api.send_message(
                -1001084499328 as i64,
                msg.to_string(),
                None, None, None, None
            );
        }
    }
}