//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

use chrono::{Datelike, Timelike, Utc};
use log::{debug, error, info, warn};
use std::{process, env, path::Path};
use env_logger::{Env, fmt::Color};
use std::io::Write;
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

//todo: wait until panic::update_hook Gets out of development.

fn main() {
    let args = Args::parse(); set_debug(&args); if args.about {about()}
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

    loop {
        for file_path in &args.files {
            //Find's the file name
            let path= Path::new(file_path).file_name().unwrap();

            let playinginfo = format!("Playing {:?}" , path);
            drop(path);
            info!("{}", &playinginfo);
            drop(playinginfo);

            //play's the files
            play(&file_path);
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
    if args.debug {
        env::set_var("RUST_BACKTRACE", "1");
        env::set_var("RUST_LOG", "debug");
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).format(move |buf, record| {
        let now = Utc::now();
        let (_is_common_era, year) = now.year_ce();
        let (_is_pm, hour) = now.hour12();
        let mut level_style = buf.style();

        if format!("{}", record.level()) == "DEBUG" {
            level_style.set_color(Color::Rgb(80, 80, 80)).set_bold(true);
        } else if format!("{}", record.level()) == "ERROR" {
            level_style.set_color(Color::Red).set_bold(true);
        } else if format!("{}", record.level()) == "WARN" {
            level_style.set_color(Color::Yellow).set_bold(true);
        }


        writeln!(buf, "[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [{}]: {}",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        hour,
        now.minute(),
        now.second(), 
        level_style.value(record.level()), 
        record.args())
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