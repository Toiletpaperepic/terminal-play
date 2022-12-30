///////////////////////////////////////////
//                                       //
//                                       //
//      Whoever is using this this       //
//        is Just spaghetti code.        //
//                                       //
//                                       //
///////////////////////////////////////////
mod play;
mod about;
mod log;
use std::{process, env, path::Path};
use clap::Parser;
use log::{print, eprint, wprint};
use play::play;
use about::about;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    quiet: bool,
    #[arg(short, long)]
    debug: bool,
    #[arg(long)]
    noaudio: bool,
    #[arg(short, long)]
    about: bool,
    files: Vec<String> //get all Argument
}

fn main() {
    let args = Args::parse();
    if args.about {about()} 
    let bootmessage = format!("Starting Terminal-Play. (version: {})", env!("CARGO_PKG_VERSION"));
    print(&bootmessage);

    if args.debug {
        env::set_var("RUST_BACKTRACE", "1");
        dbg!(&args.files);
        dbg!(&args.quiet);
        dbg!(&args.noaudio);
        dbg!(&args.about);
    }
    if args.quiet {
        wprint("i Haven't implemented --quiet Yet")
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

    if args.debug {
        //Don't do anything
    } else {
        if args.noaudio {
            wprint("No audio will play, --noaudio is Enabled")
        }
    }

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

    print("Exiting")
}