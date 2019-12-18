use rodio::Sink;
use std::sync::mpsc::Receiver;

pub fn music_controls(c: char, sink: &Sink) {
    match c {
        'p' => {
            sink.pause();
            println!("paused...");
        }
        'r' => {
            println!("resuming...");
            sink.play();
        }
        's' => {
            sink.stop();
            println!("stopped");
        }
        _ => {}
    }
}
