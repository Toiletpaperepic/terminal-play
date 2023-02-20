//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================
#![feature(panic_update_hook)]
use std::{process, env, path::Path};
use crate::log::Console;
use std::fs::File;
use clap::Parser;
use about::about;
use play::play;
mod about;
mod set_ctrlc;
mod play;
mod log;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    about: bool, //for about.rs

    #[arg(short, long)]
    quiet: bool,

    #[arg(long)]
    debug: bool,

    #[arg(short,long)]
    _loop: bool,

    files: Vec<String> //get all audio files for the Command line
}

pub(crate) static mut QUIET: &bool = &false;
pub(crate) static mut DEBUG: &bool = &false;

//todo: wait until panic::update_hook Gets out of development.

fn main() {
    let file = File::create("log.txt").expect("failed to create the log file.");
    let args = Args::parse(); let mut console = Console::new(file, false);
    if args.about {about()} if args.quiet {unsafe {QUIET = &true;}}
    let bootmessage = format!("Starting Terminal-Play. (version: {})", env!("CARGO_PKG_VERSION"));
    console.print(&bootmessage);
    drop(&bootmessage);
    set_debug(&args, &mut console);

    //Throw error if there is no files to play.
    if args.files.is_empty() {
        console.error("No file found. exiting.");
        process::exit(0)
    }

    loop {
        for file_path in &args.files {
            //Find's the file name
            let path= Path::new(file_path).file_name().unwrap();

            let playinginfo = format!("Playing {:?}" , path);
            drop(path);
            console.print(&playinginfo);
            drop(playinginfo);

            //play's the files
            play(&file_path, &mut console)
        }

        if args._loop {
            //
        }
        else {
            break;
        }
    }

    console.print("Exiting");
    drop(console);
    drop(args);
}

fn set_debug(args: &Args, console: &mut Console) {
    if args.debug {
        env::set_var("RUST_BACKTRACE", "1");
        unsafe {DEBUG = &true;}
    }

    set_ctrlc::set_ctrlc();

    console.debug(&format!("OS: {} | Family: {} | CPU: {} | Debug: {}",
        std::env::consts::OS,
        std::env::consts::FAMILY,
        std::env::consts::ARCH,
        cfg!(debug_assertions).to_string()
    ));

    console.debug(&format!(
        "&args.files = {:#?}\n&args.quiet = {}\n&args.debug = {}\n&args._loop = {}\n&args.about = {}", 
        args.files, 
        args.quiet, 
        args.debug,
        args._loop,
        args.about
    ));
}