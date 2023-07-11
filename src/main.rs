//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

use log::{debug, error, info, warn};
use crate::decoder::decoder_all;
use crate::common::set_debug;
use std::{process, env, path::PathBuf};
use crate::play::play;
use clap::Parser;
use about::about;
mod decoder;
mod common;
mod about;
mod play;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Prints about and exit
    #[arg(short, long)]
    about: bool, 

    ///opens documentation
    #[arg(short, long)]
    documentation: bool,

    ///loops
    #[arg(short, long)]
    _loop: bool,

    //get all audio files for the Command line
    files: Vec<PathBuf> 
}

fn main() {
    let args = Args::parse(); set_debug(); if args.about {about()}
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

    let sources = decoder_all(args.files);

    loop {
        for file_path in &sources {
            let playinginfo = format!("Playing {:?}" , file_path.path);
            info!("{}", &playinginfo);
            drop(playinginfo);

            //play's the files
            play(file_path);
        }

        if args._loop {
            //
        }
        else {
            break;
        }
    }

    info!("Exiting");
}