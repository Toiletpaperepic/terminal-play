//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

use chrono::{Datelike, Utc, Timelike};
use env_logger::{Env, fmt::Color};
use std::io::Write;
use log::debug;
use crate::Args;

pub fn set_debug() {
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

    // debug!("&args.files = {:#?}\n&args.debug = {}\n&args._loop = {}\n&args.about = {}", 
    //     args.files, 
    //     args.documentation,
    //     args._loop,
    //     args.about
    // );
}