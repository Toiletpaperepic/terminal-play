//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

#[macro_use] extern crate log;

use crate::playback::{decoder::decoder_all, play::play};
use common::{logger::init, about::about};
use std::{process, env, path::PathBuf};
use clap::Parser;
use log::LevelFilter;
mod playback;
mod common;

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Args {
    ///Prints about and exit
    #[arg(short, long)]
    version: bool, 

    ///opens documentation
    #[arg(short, long)]
    documentation: bool,

    ///loops
    #[arg(short, long)]
    repeat: bool,

    //get all audio files for the Command line
    files: Vec<PathBuf>,

    ///set the log level.
    #[arg(short, long)]
    log_level: Option<LevelFilter>
}

fn main() {
    let args = Args::parse(); 
    init(args.log_level.unwrap_or_else(|| LevelFilter::Info)).unwrap(); 
    if args.version {
        about()
    }
    info!("Starting Terminal-Play. (version: {})", env!("CARGO_PKG_VERSION"));

    //set the Ctrl+C Message.
    ctrlc::set_handler(move || {
        warn!("Received Ctrl+C exiting.");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    //Throw error if there is no files to play.
    if args.files.is_empty() {
        error!("No file found. exiting.");
        process::exit(0)
    }

    play(decoder_all(args.files), args.repeat);

    info!("Exiting");
}