//=================================================
//                 Terminal-Play          
//
//               A simple MP3 Player        
//
//https://github.com/Toiletpaperepic/terminal-play
//
//=================================================

use std::{fs::File, process, io::BufReader, path::{Path, PathBuf}};
use rodio::{Decoder, Source, source::Buffered};

#[derive(Clone)]
pub(crate) struct Audio {
    pub source: Buffered<Decoder<BufReader<File>>>,
    pub path: String
}

pub(crate) fn decoder_all(args: Vec<PathBuf>) -> Vec<Audio> {
    let mut vec_audio: Vec<Audio> = Vec::new();

    for file in args { 
        let file_clone = file.clone();
        let path= Path::new(&file_clone).file_name().unwrap();

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = File::open(file).unwrap_or_else(|err| {
            error!("{err}");
            process::exit(3)
        });
    
        // Decode that sound file into a source
        let source = Decoder::new(BufReader::new(file)).unwrap_or_else(|err| {
            error!("{err}");
            process::exit(1)
        }).buffered();

        vec_audio.push(Audio {
            source,
            path: path.to_str().unwrap().to_string(),
        });
    }

    return vec_audio;
}