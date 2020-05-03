use structopt::StructOpt;

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
    println!("{}", text);
}
