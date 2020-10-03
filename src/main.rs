use structopt::StructOpt;
use std::io::BufReader;
use std::io::Error;

use atom_syndication::Feed;
use dialoguer::Select;
use rss::Channel;
use html2md::parse_html;

mod term;
use term::run;
use term::clear;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[structopt(name = "rudder", about = "A dumb thing I guess")]
struct Cli {
    /// The URL
    url: String,
}

fn main() {
    let args = Cli::from_args();
    clear().unwrap();
    run(article(args.url)).unwrap()
}

fn article(url: String) -> String {
    let result = reqwest::blocking::get(&url);
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
                    let titles = result.items().iter().map(|item| item.title().unwrap()).collect::<Vec<_>>();
                    let selected = match Select::new().items(&titles).with_prompt("pick a title").interact() {
                        Ok(selected) => { selected }
                        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
                    };
                    let selected_item = &result.items()[selected];
                    parse_html(&format!("<h1>{}</h1>{}", selected_item.title().unwrap(), selected_item.description().unwrap()))
                }
                Err(error) => { panic!("Can't deal with {}, just exit here", error); }
            }
        }
    }
}

fn read_feed(feed: Feed) -> String {
    let titles = feed.entries().iter().map(|entry| entry.title()).collect::<Vec<_>>();

    let selected = match Select::new().items(&titles).with_prompt("pick a title").interact() {
        Ok(selected) => { selected }
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };

    let selected_entry = &feed.entries()[selected];
    parse_html(&format!("<h1>{}</h1>{}", selected_entry.title(), selected_entry.content().unwrap().value().unwrap()))
}

