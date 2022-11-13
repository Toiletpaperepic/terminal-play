mod play;
mod log;
use std::process;
use clap::Parser;
use log::{print, eprint};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    quiet: bool,
    files: Vec<String> //get all Argument
}

fn main() {
    let args = Args::parse();
    let bootmessage = format!("Starting Terminal-Play. (version: {})", env!("CARGO_PKG_VERSION"));
    print(&bootmessage);

    if args.quiet {
        //
        print("i Haven't implemented --quiet Yet lol")
    }
    
    //set the Ctrl+C Message.
    ctrlc::set_handler(move || {
        print("Received Ctrl+C exiting.");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    //Throw error if there is no files to play.
    if args.files.is_empty() {
        eprint("No file found. exiting.")
    }

    for x in &args.files {
        //to print info
        let playinginfo = format!("Playing {}" , x);
        print(&playinginfo);

        //to play the files
       play::play(x)
    }

    print("Exiting")
}