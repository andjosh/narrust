extern crate rss;
extern crate request;

use rss::Rss;
use std::collections::HashMap;

fn main() {

    let url = "http://www.andjosh.com/rss.xml";
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert(
        "Connection".to_string(),
        "close".to_string()
    );

    let resp = match request::get(&url, &mut headers) {
        Ok(resp) => resp,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let rss_str = resp.body;
    let Rss(channel) = rss_str.parse::<Rss>().unwrap();

    println!("{}", resp.status_code);
    match channel.items[0].title {
        Some(ref x) => println!("{}", x),
        None    => println!("-- No title found --")
    }
}
