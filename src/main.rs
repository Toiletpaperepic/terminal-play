//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

use std::{process, env, path::Path, io, time::Duration};
use clap::Parser;
use about::about;
use crate::crossterm::run;
use play::play;
use log::*;
mod crossterm;
mod about;
mod play;
mod log;
mod app;
mod ui;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    quiet: bool,

    #[arg(long)]
    debug: bool,

    #[arg(long)]
    noaudio: bool, ///for disable audio

    #[arg(short,long)]
    _loop: bool,

    #[arg(short, long)]
    about: bool, ///to run about.rs

    #[arg(short, long)]
    tui: bool,

    /// whether unicode symbols are used to improve the overall look of the app
    #[arg(short, long, default_value = "true")]
    enhanced_graphics: bool,

    files: Vec<String> //get all audio files for the Command line
}

pub(crate) static mut QUIET: &bool = &false;

fn main() {
    let args = Args::parse();
    if args.about {about()} if args.quiet {unsafe {QUIET = &true;}}
    let bootmessage = format!("Starting Terminal-Play. (version: {})", env!("CARGO_PKG_VERSION"));
    print(&bootmessage);
    let preset_tick_rate:u64 = 250;

    if args.debug {
        env::set_var("RUST_BACKTRACE", "1");
        dbg!(&args.files);
        dbg!(&args.quiet);
        dbg!(&args.noaudio);
        dbg!(&args.about);
        dbg!(preset_tick_rate);
    }

    if args.enhanced_graphics {
        let tick_rate = Duration::from_millis(preset_tick_rate);
        run(tick_rate, args.enhanced_graphics);
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

    print("Exiting");
}