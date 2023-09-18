use std::{io::stdin, thread, sync::mpsc::channel};
use clap::{Command, value_parser, arg};
use rodio::{OutputStream, Sink};
use super::decoder::Audio;

enum Commands {
    SetVolume(f32),
    Stop,
    Skip,
    Quit // IT'S NOT FUCKING DEAD
}

///plays files using rodio
pub(crate) fn play(audio_sources: Vec<Audio>, _loop: bool) {
    
    let (sender, receiver) = channel::<Result<Commands, String>>();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sources_iter = audio_sources.clone().into_iter();
    //start sink
    let sink = Sink::try_new(&stream_handle).unwrap();

    let media = thread::Builder::new().name("Media controls".to_string()).spawn(move || { loop {
        let mut s_buffer = String::new();
        let stdin = stdin();

        loop {
            s_buffer.clear();
            stdin.read_line(&mut s_buffer).unwrap();
            let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");
    
            sender.send(respond(line.as_str())).unwrap();
        }
    }}).unwrap();

    //todo: this loop Takes up way too much CPU resources. fix that
    loop {
        if sink.empty() {
            if let Some(audio) = sources_iter.next() {
                debug!("sink is empty. adding next audio '{}'.", audio.path);
                info!("{}", format!("Playing {:?}" , audio.path));
                sink.append(audio.source);
            } else {
                if _loop {
                    sources_iter = audio_sources.clone().into_iter();
                } else {
                    debug!("sink is empty, no next audio. exiting");
                    break;
                }
            }
        }

        if media.is_finished() {
            error!("Media controls thread is Panicked or stopped!")
        }

        // thread::sleep(Duration::from_millis(1000));

        match receiver.try_recv() {
            Ok(com) => {
                match com {
                    Ok(Commands::Quit) => {
                        sink.stop();
                        break;
                    }
                    Ok(Commands::SetVolume(v)) => {
                        sink.set_volume(v);
                    }
                    Ok(Commands::Stop) => {
                        debug!("sink is stoping.");
                        sink.stop();
                        break;
                    },
                    Ok(Commands::Skip) => {
                        sink.skip_one()
                    }
                    Err(e) => {
                        error!("{}", e)
                    },
                }
            }
            Err(e) => {
                trace!("{}", e)
            },
        }
    }
}

fn respond(line: &str) -> Result<Commands, String> {
    debug!("user send's {}", line);
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    debug!("{:#?}", args);
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("volume", matches)) => {
            Ok(Commands::SetVolume(*matches.get_one::<f32>("args").unwrap()))
        }
        Some(("skip", _matches)) => {
            Ok(Commands::Skip)
        }
        Some(("quit", _matches)) => {
            Ok(Commands::Stop)
        }
        Some((name, _matches)) => unimplemented!("{name}"),
        None => unreachable!("subcommand required"),
    }
}

fn cli() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("Terminal-Play")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("Terminal-Play")
        .subcommand_help_heading("Terminal-Play")
        .help_template(PARSER_TEMPLATE)
        .subcommands([
            Command::new("volume")
                .help_template(APPLET_TEMPLATE)
                .short_flag('v')
                .arg(
                    arg!(args: [NUM] "the command line arguments for qemu.")
                        .num_args(1)
                        .required(true)
                        .value_parser(value_parser!(f32))
                ),
            Command::new("skip")
                .help_template(APPLET_TEMPLATE)
                .short_flag('s'),
            Command::new("quit")
                .help_template(APPLET_TEMPLATE)
                .short_flag('e')
                .aliases(["exit", "stop"]),
        ])
}