//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

use std::{fs::File, process, io::{BufReader, stdin}, thread};
use rodio::{Decoder, OutputStream, Sink};
use crate::log::Console;

///plays files using rodio
pub(crate) fn play(_arg:&str, console: &mut Console) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let stdin = stdin();
    let mut s_buffer = String::new();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = File::open(_arg).unwrap_or_else(|err| {
        console.error(&format!("{err}"));
        process::exit(3)
    });
    
    // Decode that sound file into a source
    let source = Decoder::new(BufReader::new(file)).unwrap_or_else(|err| {
        console.error(&format!("{err}"));
        process::exit(1)
    });
    
    sink.append(source);

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                s_buffer.clear();
                stdin.read_line(&mut s_buffer).unwrap();
                let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");
                
                if line == "exit" {
                    sink.stop();
                    break;
                } else if line.starts_with("v") {
                    let v: Vec<&str> = line.split(' ').collect();
                    //if v[1] == f32 {}
                    let v_f32:f32 = v[1].parse::<f32>().unwrap();
                    sink.set_volume(v_f32);
                }
                
            }
        });
    });

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
