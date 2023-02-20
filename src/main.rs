//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

use env_logger::Env;
use log::{debug, error, info};
use std::io::Write;
use chrono::{Datelike, Timelike, Utc};
use std::{process, env, path::Path};
use clap::Parser;
use about::about;
use play::play;
mod about;
mod play;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    about: bool, //for about.rs

    // #[arg(short, long)]
    // quiet: bool,

    #[arg(long)]
    debug: bool,

    #[arg(short,long)]
    _loop: bool,

    files: Vec<String> //get all audio files for the Command line
}

pub(crate) static mut DEBUG: &bool = &false;

//todo: wait until panic::update_hook Gets out of development.

fn main() {
    let args = Args::parse(); set_debug(&args); if args.about {about()}
    info!("Starting Terminal-Play. (version: {})", env!("CARGO_PKG_VERSION"));

    //Throw error if there is no files to play.
    if args.files.is_empty() {
        error!("No file found. exiting.");
        process::exit(0)
    }

    loop {
        for file_path in &args.files {
            //Find's the file name
            let path= Path::new(file_path).file_name().unwrap();

            let playinginfo = format!("Playing {:?}" , path);
            drop(path);
            info!("{}", &playinginfo);
            drop(playinginfo);

            //play's the files
            play(&file_path)
        }

        if args._loop {
            //
        }
        else {
            break;
        }
    }

    info!("Exiting");
    drop(args);
}

fn set_debug(args: &Args) {
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    let (_is_pm, hour) = now.hour12();

    if args.debug {
        env::set_var("RUST_BACKTRACE", "1");
        unsafe {DEBUG = &true;}
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).format(move |buf, record| {
        writeln!(buf, "[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [{}]: {}",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        hour,
        now.minute(),
        now.second(), record.level(), record.args())
    }).init();

    debug!("OS: {} | Family: {} | CPU: {} | Debug: {}",
        std::env::consts::OS,
        std::env::consts::FAMILY,
        std::env::consts::ARCH,
        cfg!(debug_assertions).to_string()
    );

    debug!("&args.files = {:#?}\n&args.debug = {}\n&args._loop = {}\n&args.about = {}", 
        args.files, 
        args.debug,
        args._loop,
        args.about
    );
}