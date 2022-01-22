extern crate clap;

use std::process;

use clap::Parser;

/// Procedurally generated hexagon based world
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Display the app in full screen
    #[clap(short, long, raw(false), parse(try_from_str), default_value = "true")]
    full_screen: bool,

    /// Width in pixels
    #[clap(short, long, default_value_t = 1920)]
    width: u32,

    /// Height in pixels
    #[clap(short, long, default_value_t = 1080)]
    height: u32,
}

fn main() {
    let args = Args::parse();
    println!("Running app with {:?}", args);

    if let Err(e) = auto_dungeon::run(args.full_screen, args.width, args.height) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
