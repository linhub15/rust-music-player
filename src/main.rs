use rodio::Sink;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

fn main() {
    let (tx, rx) = mpsc::channel();
    let main_input = input_thread(tx);
    let receiver = Arc::new(Mutex::new(rx));
    player_thread(Arc::clone(&receiver));

    main_input.join().unwrap();
}

fn input_thread(tx: Sender<char>) -> JoinHandle<()> {
    std::thread::spawn(move || loop {
        let s = get_char();
        let tx1 = mpsc::Sender::clone(&tx);
        tx1.send(s).unwrap();
    })
}

fn player_thread(rx: Arc<Mutex<Receiver<char>>>) {
    let player = std::thread::spawn(move || {
        let c = rx.lock().unwrap().recv().unwrap();
        match c {
            'a' => {
                start_song("clear-as-water.mp3", Arc::clone(&rx));
            }
            _ => {
                println!("here now");
            }
        }
    });
    player.join().unwrap();
}

fn get_char() -> char {
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
    s.pop().unwrap()
}

fn start_song(song_path: &str, rx: Arc<Mutex<Receiver<char>>>) {
    let sink = play(load_song(song_path));
    println!("Playing song...");
    std::thread::spawn(move || 'song: loop {
        match rx.lock().unwrap().try_recv() {
            Ok(value) => match value {
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
                    println!("stopped...");
                    break 'song;
                }
                _ => {}
            },
            Err(TryRecvError::Disconnected) => {}
            Err(TryRecvError::Empty) => {}
        }
    })
    .join()
    .unwrap();
    println!("song loop stopped");
}

fn load_song(path: &str) -> File {
    File::open(path).unwrap()
}

fn play(song: File) -> Sink {
    let device = rodio::default_output_device().unwrap();
    rodio::play_once(&device, BufReader::new(song)).unwrap()
}
