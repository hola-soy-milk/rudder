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
}
