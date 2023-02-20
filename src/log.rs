//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

use chrono::{Datelike, Timelike, Utc};
use ansi_term::Colour;
use crate::QUIET;

///print info to the terminal
pub(crate) fn print(_arg:&str){
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    let (_is_pm, hour) = now.hour12();

    let println_message = format!("[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [INFO]: {}" ,
    year,
    now.month(),
    now.day(),
    now.weekday(),
    hour,
    now.minute(),
    now.second(),
    _arg);

    unsafe {
        if *QUIET {
        
        }
        else {
            println!("{}", println_message)
        }
    }  
}

pub(crate) fn warn(_arg:&str){
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    let (_is_pm, hour) = now.hour12();
    
    let warn_message = format!("[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [WARN]: {}" ,
    year,
    now.month(),
    now.day(),
    now.weekday(),
    hour,
    now.minute(),
    now.second(),
    _arg);
    
    unsafe {
        if *QUIET {
        
        }
        else {
            eprintln!("{}", Colour::Yellow.paint(warn_message))
        }
    } 
}

pub(crate) fn eprint(_arg:&str){
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    let (_is_pm, hour) = now.hour12();
    
    let error_message = format!("[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [ERROR]: {}" ,
    year,
    now.month(),
    now.day(),
    now.weekday(),
    hour,
    now.minute(),
    now.second(),
    _arg);
    
    unsafe {
        if *QUIET {
        
        }
        else {
            eprintln!("{}", Colour::Red.paint(error_message));
        }
    } 
}