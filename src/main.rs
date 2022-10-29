mod play;
mod clitools;
use std::env;
use play::play;
use clitools::printinfo;

fn main() {
    printinfo("Starting Terminal-Play");

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for x in &args {
        //to print info
        let playinginfo = format!("playing {}" , x);
        printinfo(&playinginfo);

        //to play the files
        play(x)
    }

    printinfo("Exiting")
}