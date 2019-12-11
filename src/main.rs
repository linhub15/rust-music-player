use rodio::Sink;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};

fn main() {
    let mut s = String::new();
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    loop {
        println!("'s' to start. 'p' to pause. 'x' to exit.");
        let _ = stdout().flush();
        s.clear();
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
        match s.as_ref() {
            "x" => break,
            "s" => {
                std::thread::spawn(move || {
                    let sink = play(load_song("clear-as-water.mp3"));
                    sink.sleep_until_end();
                }).;
            }
            "p" => {
                // This doesn't work because I can't access sink from thread
                // maybe this can help
                // https://stackoverflow.com/questions/26199926/how-to-terminate-or-suspend-a-rust-thread-from-another-thread
                println!("{:?}", sink.len());
                pause(&sink);
            }
            _ => println!("{} is invalid", s),
        }
    }
}

fn load_song(path: &str) -> File {
    File::open(path).unwrap()
}

fn play(song: File) -> Sink {
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    let decoder = rodio::Decoder::new(BufReader::new(song)).unwrap();
    sink.append(decoder);
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
