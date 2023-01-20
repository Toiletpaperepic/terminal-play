//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

mod play;
mod about;
mod log;
use std::{process, env, path::Path};
use clap::Parser;
use log::*;
use play::play;
use about::about;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    quiet: bool,

    #[arg(long)]
    debug: bool,

    #[arg(long)]
    noaudio: bool, //for disable audio

    #[arg(short,long)]
    _loop: bool,

    #[arg(short, long)]
    about: bool, //for about.rs

    files: Vec<String> //get all audio files for the Command line
}

pub(crate) static mut QUIET: &bool = &false;

fn main() {
    let args = Args::parse();
    if args.about {about()}
    if args.quiet {unsafe {QUIET = &true;}}
    let bootmessage = format!("Starting Terminal-Play. (version: {})", env!("CARGO_PKG_VERSION"));
    print(&bootmessage);

    if args.debug {
        env::set_var("RUST_BACKTRACE", "1");
        dbg!(&args.files);
        dbg!(&args.quiet);
        dbg!(&args.noaudio);
        dbg!(&args.about);
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
        process::exit(0)
    }

    if args.debug {
        //Don't do anything
    } else {
        if args.noaudio {
            warn("No audio will play, --noaudio is Enabled")
        }
    }

    loop {
        for file in &args.files {
            //Find's the file name
            let path= Path::new(file).file_name().unwrap();
            //print the playing info
            let playinginfo = format!("Playing {:?}" , path);
            print(&playinginfo);

            //play's the files
            if args.noaudio {
                if args.debug {
                    eprint("Can't play, --noaudio is Enabled. Skipping.")
                }
            } else {
                play(&file)
            }
        }

        if args._loop {
            //
        }
        else {
            break;
        }
    }
    
    print("Exiting")
}