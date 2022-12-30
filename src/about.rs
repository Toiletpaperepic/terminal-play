use std::process;

pub(crate) fn about() {
    println!("================================================");
    println!("         Terminal-Play (version: {})          ", env!("CARGO_PKG_VERSION"));
    println!();
    println!("               A simple MP3 Player              ");
    println!();
    println!("https://github.com/Toiletpaperepic/terminal-play");
    println!();
    println!("================================================");
    process::exit(0)
}