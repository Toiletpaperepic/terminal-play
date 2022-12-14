use chrono::{Datelike, Timelike, Utc};
use ansi_term::Colour;

///print info to the terminal
pub(crate) fn print(_arg:&str){
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    let (_is_pm, hour) = now.hour12();

    println!("[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [INFO]: {}" ,
    year,
    now.month(),
    now.day(),
    now.weekday(),
    hour,
    now.minute(),
    now.second(),
    _arg);
}

pub(crate) fn wprint(_arg:&str){
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    let (_is_pm, hour) = now.hour12();
    
    let warnmessage = format!("[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [WARN]: {}" ,
    year,
    now.month(),
    now.day(),
    now.weekday(),
    hour,
    now.minute(),
    now.second(),
    _arg);
    
    eprintln!("{}", Colour::Yellow.paint(warnmessage))
}

pub(crate) fn eprint(_arg:&str){
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    let (_is_pm, hour) = now.hour12();
    
    let errormessage = format!("[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [ERROR]: {}" ,
    year,
    now.month(),
    now.day(),
    now.weekday(),
    hour,
    now.minute(),
    now.second(),
    _arg);
    
    eprintln!("{}", Colour::Red.paint(errormessage));
}