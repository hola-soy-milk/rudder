use structopt::StructOpt;
use std::io::BufReader;

use atom_syndication::Feed;
use dialoguer::Select;
use rss::Channel;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[structopt(name = "rudder", about = "A dumb thing I guess")]
struct Cli {
    /// The URL
    url: String,
}

fn main() {
    let args = Cli::from_args();
    println!("url: {}", args.url);
    let result = reqwest::blocking::get(&args.url);
    let text_result = match result {
        Ok(text_result) => { text_result }
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };

    let text = match text_result.text() {
        Ok(text) => { text }
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };
    let file = BufReader::new(text.as_bytes());
    match Feed::read_from(file) {
        Ok(feed) => {
            read_feed(feed)
        }
        Err(_) => {
            let data = BufReader::new(text.as_bytes());
            match Channel::read_from(data) {
                Ok(result) => {
                }
                Err(error) => { panic!("Can't deal with {}, just exit here", error); }
            }
        }
    };
}

fn read_feed(feed: Feed) {
    let titles = feed.entries().iter().map(|entry| entry.title()).collect::<Vec<_>>();

    let selected = match Select::new().items(&titles).with_prompt("pick a title").interact() {
        Ok(selected) => { selected }
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };

    println!("{}", feed.entries()[selected].content().unwrap().value().unwrap())
}
