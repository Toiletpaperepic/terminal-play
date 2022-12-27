mod play;
mod log;
use std::{process, env, path::Path};
use clap::Parser;
use log::{print, eprint};
use play::play;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    quiet: bool,
    #[arg(short, long)]
    debug: bool,
    files: Vec<String> //get all Argument
}

fn main() {
    let args = Args::parse();
    let bootmessage = format!("Starting Terminal-Play. (version: {})", env!("CARGO_PKG_VERSION"));
    print(&bootmessage);

    if args.debug {
        env::set_var("RUST_BACKTRACE", "1");
        dbg!(&args.files);
    }
    if args.quiet {
        eprint("i Haven't implemented --quiet Yet lol")
    }
    
    //set the Ctrl+C Message.
    ctrlc::set_handler(move || {
        print("Received Ctrl+C exiting.");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    //Throw error if there is no files to play.
    if args.files.is_empty() {
        eprint("No file found. exiting.");
        process::exit(1)
    }

    for file in &args.files {
        //Find the file name
        let path= Path::new(file).file_name().unwrap().to_str().unwrap();
        //to print info
        let playinginfo = format!("Playing {}" , path);
        print(&playinginfo);

        //to play the files
        play(&file)
    }

    print("Exiting")
}