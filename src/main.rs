mod play;
mod log;
use std::process;
use clap::Parser;
use log::printinfo;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    files: Vec<String>,
}

fn main() {
    printinfo("Starting Terminal-Play");

    ctrlc::set_handler(move || {
        printinfo("Received Ctrl+C exiting.");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let args = Args::parse();

    for x in &args.files {
        //to print info
        let playinginfo = format!("Playing {}" , x);
        printinfo(&playinginfo);

        //to play the files
       play::play(x)
    }

    printinfo("Exiting")
}