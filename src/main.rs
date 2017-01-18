extern crate eventsource;
extern crate hyper;
extern crate regex;
extern crate telegram_bot;
extern crate json;

extern crate url;

use url::percent_encoding::{
    percent_encode, QUERY_ENCODE_SET
};

use hyper::Url;
use regex::Regex;

use json::JsonValue::Null;

fn main() {
    let re = [
        Regex::new(r"psychonaut\s?wiki").unwrap(),
        Regex::new(r"psychonaut.?wiki").unwrap(),
        Regex::new(r"disregard\s?everything\s?i\s?say").unwrap()
    ];

    let reddit_rgx = Regex::new(r"www.reddit.com").unwrap();

    let api = telegram_bot::Api::from_env("TELEGRAM_TOKEN").unwrap();

    let url = Url::parse("http://stream.pushshift.io").unwrap();
    let client = eventsource::Client::new(url);

    for event in client {
        let uw_event = event.unwrap();

        let evt = json::parse(&uw_event.data).unwrap();

        if re.iter().any(|ref exp| exp.is_match(&uw_event.data)) {
            let link = {
                let linkstr = evt["link_url"].to_string();

                let link_id = evt["link_id"].to_string();
                let mut r_linkid = link_id.chars();
                r_linkid.by_ref().nth(2);
                let link_id = r_linkid.as_str();

                if evt["permalink"] != Null {
                    format!("https://www.reddit.com{}", evt["permalink"].to_string())
                } else if !reddit_rgx.is_match(&linkstr) {
                    format!("https://www.reddit.com/r/{}/comments/{}/_/{}/", evt["subreddit"], link_id, evt["id"])
                } else {
                    format!("{}{}", evt["link_url"].to_string(), evt["link_id"].to_string())
                }
            };

            let msg = format!(
                "User {:?} wrote into subreddit {:?} with title {:?}:\n\n» {} «\n\nPermalink: {}",
                evt["author"].to_string(),
                evt["subreddit"].to_string(),
                evt["link_title"].to_string(),
                evt["body"].to_string(),
                link.to_string()
            );

            let _ = api.send_message(
                -1001084499328 as i64,
                percent_encode(msg.as_bytes(), QUERY_ENCODE_SET).collect::<String>(),
                None, None, None, None
            );
        }
    }
}
