use rodio::Sink;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

fn main() {
    let (tx, rx) = mpsc::channel();
    let handler = input_thread(tx);

    start_song(rx);
    handler.join().unwrap();
}

fn input_thread(tx: Sender<String>) -> JoinHandle<()> {
    std::thread::spawn(move || loop {
        let _ = stdout().flush();
        let mut s = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let tx1 = mpsc::Sender::clone(&tx);
        tx1.send(s).unwrap();
    })
}

fn start_song(rx: Receiver<String>) {
    let sink = play(load_song("clear-as-water.mp3"));
    println!("Playing song...");
    'song: loop {
        for s in &rx {
            match s.as_ref() {
                "p" => {
                    sink.pause();
                    println!("paused...");
                }
                "r" => {
                    println!("resuming...");
                    sink.play();
                }
                "s" => {
                    sink.stop();
                    println!("stopped");
                    break 'song;
                }
                _ => {}
            }
        }
    }
    println!("i'm out of inifite loop");
}

fn load_song(path: &str) -> File {
    File::open(path).unwrap()
}

fn empty_sink() -> Sink {
    let device = rodio::default_output_device().unwrap();
    Sink::new(&device)
}

fn play(song: File) -> Sink {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::play_once(&device, BufReader::new(song)).unwrap();
    sink
}

fn pause(sink: &Sink) -> &Sink {
    sink.pause();
    sink
}

fn resume(sink: &Sink) -> &Sink {
    if sink.empty() {
        return sink;
    }
    sink.play();
    sink
}
