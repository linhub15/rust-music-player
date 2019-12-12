use rodio::Sink;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

fn main() {
    let (tx, rx) = mpsc::channel();
    let handler = input_thread(tx);
    song_thread(rx);
    handler.join().unwrap();
}

fn input_thread(tx: Sender<String>) -> JoinHandle<()> {
    std::thread::spawn(move || loop {
        println!("'s' to start. 'p' to pause. 'x' to exit.");
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
        println!("You typed: {}", s);
        let tx1 = mpsc::Sender::clone(&tx);
        tx1.send(s).unwrap();
    })
}

fn song_thread(rx: Receiver<String>) {
    let mut threads: Vec<std::thread::JoinHandle<()>> = Vec::new();
    let sink = empty_sink();

    for s in rx {
        match s.as_ref() {
            "p" => { // pause
                let parked_thread = threads.remove(0);
                parked_thread.thread().unpark();
                parked_thread.join().unwrap();
                sink.pause();
            }
            "s" => { // start
                let thread = std::thread::spawn(move || {
                    let sink = play(load_song("clear-as-water.mp3"));
                    std::thread::park();
                });
                threads.push(thread);
            }
            "r" => { // resume
                sink.play();
                sink.sleep_until_end();
            }
            _ => {}
        }
    }
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
