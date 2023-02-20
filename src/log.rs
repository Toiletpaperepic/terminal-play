//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

use std::fs::File;
use std::io::prelude::*;
use chrono::{Datelike, Timelike, Utc};
use ansi_term::Colour;
use crate::{QUIET, DEBUG};

pub(crate) struct Console {
    pub file: File
}

impl<'a> Console {
    pub fn new(mut file: File, remove_warn: bool) -> Console { 
        if remove_warn {
            //
        } else {
            file.write(b"WARNING: Log File are still in development.").expect("failed to write to the log file.");
        }

        Console {
            file
        }
    }

    pub fn write(&mut self, log: &str) { 
        let string = format!("\n{}", log);
        self.file.write(string.as_bytes()).expect("failed to write to the log file.");
    }

    pub fn debug(&mut self, log: &str)  { 
        let now = Utc::now();
        let (_is_common_era, year) = now.year_ce();
        let (_is_pm, hour) = now.hour12();

        let string = format!("[{}-{:02}-{:02} {:?} {:02}:{:02}:{:02}] [debug]: {}" ,
        year,
        now.month(),
        now.day(),
        now.weekday(),
        hour,
        now.minute(),
        now.second(),
        log);

        self.write(&string);

        unsafe {
            if *DEBUG {
                println!("{}", string)
            }
        }  
    }

    pub(crate) fn print(&mut self,log:&str)  {
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
        log);
        
        self.write(&println_message);
    
        unsafe {
            if *QUIET {
            
            }
            else {
                println!("{}", println_message)
            }
        }  
    }
    
    pub(crate) fn warn(&mut self,log:&str)  {
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
        log);

        self.write(&warn_message);
        
        unsafe {
            if *QUIET {
            
            }
            else {
                eprintln!("{}", Colour::Yellow.paint(warn_message))
            }
        } 
    }
    
    pub(crate) fn error(&mut self,log:&str)  {
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
        log);

        self.write(&error_message);
        
        unsafe {
            if *QUIET {
            
            }
            else {
                eprintln!("{}", Colour::Red.paint(error_message));
            }
        } 
    }
}