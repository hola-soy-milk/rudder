use structopt::StructOpt;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[structopt(name = "rudder", about = "A dumb thing I guess")]
struct Cli {
    /// The URL
    url: String,
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
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
    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}", name);
            }
            Ok(XmlEvent::EndElement { name }) => {
                println!("{}", name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
