mod play;
mod clitools;
use std::env;
use play::play;
use clitools::printinfo;

fn main() {
    printinfo("Starting Terminal-Play");

    let args: Vec<String> = env::args().collect(); 
    let query = &args[1];

    //to print info
    let playinginfo = format!("playing {}" , query);
    printinfo(&playinginfo);
    //to play
    play(query);
    //
    printinfo("Exiting")
}