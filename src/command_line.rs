use chrono::{Datelike, Timelike, Utc};



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