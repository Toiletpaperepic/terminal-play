use chrono::{Datelike, Timelike, Utc};
use ansi_term::Colour;
use std::process;

pub(crate) fn printinfo(_arg:&str){
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

pub(crate) fn printerrorinfo(_arg:&str){
    let now = Utc::now();
    let (_is_common_era, year) = now.year_ce();
    let (_is_pm, hour) = now.hour12();
    
    let errorinfo = format!("[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [ERROR]: {}" ,
    year,
    now.month(),
    now.day(),
    now.weekday(),
    hour,
    now.minute(),
    now.second(),
    _arg);
    
    println!("{}", Colour::Red.paint(errorinfo));
    process::exit(1);
}