use std::{sync::mpsc::channel, thread, process};

pub(crate) fn set_ctrlc() {
    let (tx, rx) = channel();

    ctrlc::set_handler(move || {
        tx.send(()).expect("Could not send signal on channel.");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    thread::scope(|s| {
        s.spawn(|| {
            rx.recv().expect("Could not receive from channel.");
        });
    });
}