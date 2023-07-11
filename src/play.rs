use rodio::{OutputStream, Sink};
use log::{info, error, debug};
use std::{io::stdin, thread};
use crate::decoder::Audio;

///plays files using rodio
pub(crate) fn play(audio_sources: &Audio) {
    //play's the files
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
            
    let mut s_buffer = String::new();
    let stdin = stdin();
    
    sink.append(audio_sources.source.clone());

        
    loop {
        // if sink.empty() {
        //     dbg!(sink.empty());
        //     break;
        // }
        println!("nut{}", sink.empty());

        s_buffer.clear();
        stdin.read_line(&mut s_buffer).unwrap();
        let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");
    
        if line.starts_with("e") {
            sink.stop();
            break;
        } else if line.starts_with("h") {
            println!("help: print of this command\nexit: exits the program\nvolume: set the volume Level")
        } else if line.starts_with("v") {
            let v: Vec<&str> = line.split(' ').collect();
            if v.len() == 1 {
                error!("unexpected argument: empty message.")
            } else if v[1].parse::<f32>().is_ok() {
                debug!("set volume to {}", v[1]); 
                let v_f32:f32 = v[1].parse::<f32>().unwrap();
            sink.set_volume(v_f32);
            } else {
                error!("Not a number")
            }
        } else {
            error!("Unknown command")
        }
    }

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    // sink.sleep_until_end();
}