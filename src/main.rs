mod play;
mod command_line;
use std::{env, process};
use command_line::printinfo;

fn main() {
    printinfo("Starting Terminal-Play");

    ctrlc::set_handler(move || {
        printinfo("Received Ctrl+C exiting.");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for x in &args {
        //to print info
        let playinginfo = format!("Playing {}" , x);
        printinfo(&playinginfo);

        //to play the files
       play::play(x)
    }

    printinfo("Exiting")
}