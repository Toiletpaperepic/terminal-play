use std::{fs::File, process};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use crate::log::eprint;

///plays files using rodio
pub(crate) fn play(_arg:&str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = File::open(_arg).unwrap_or_else(|err| {
        eprint(&format!("{err}"));
        //todo: make it Grab the exit code From {err}
        process::exit(3)
    });
    
    // Decode that sound file into a source
    let source = Decoder::new(BufReader::new(file)).unwrap_or_else(|err| {
        eprint(&format!("{err}"));
        //todo: make it Grab the exit code From {err}
        process::exit(3)
    });
    
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
