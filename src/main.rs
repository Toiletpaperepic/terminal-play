mod play;
mod log;
use std::process;
use clap::Parser;
use log::{printinfo, printerrorinfo};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    files: Vec<String> //get all Argument
}

fn main() {
    let args = Args::parse();
    printinfo("Starting Terminal-Play");
    
    //set the Ctrl+C Message.
    ctrlc::set_handler(move || {
        printinfo("Received Ctrl+C exiting.");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    //Throw in error if there is no files to play.
    if args.files.is_empty() {
        printerrorinfo("No file found. exiting.")
    }

    for x in &args.files {
        //to print info
        let playinginfo = format!("Playing {}" , x);
        printinfo(&playinginfo);

        //to play the files
       play::play(x)
    }

    printinfo("Exiting")
}