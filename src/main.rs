use std::fs::File;
use std::io::BufReader;
use std::{thread, time};

fn main() {
    let song: File = File::open("clear-as-water.mp3").unwrap();
    let song_2: File = File::open("kaze-no-toorimichi.mp3").unwrap();

    let sink = play(song);

    let duration = time::Duration::from_millis(3000);
    thread::sleep(duration);

    let sink = pause(&sink);

    let duration = time::Duration::from_millis(3000);
    thread::sleep(duration);

    let sink = resume(&sink);
    pause(&sink);

    let sink = play(song_2);
    sink.sleep_until_end();
}

fn play(song: File) -> rodio::Sink {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let decoder = rodio::Decoder::new(BufReader::new(song)).unwrap();
    sink.append(decoder);
    sink
}

fn pause(sink: &rodio::Sink) -> &rodio::Sink {
    sink.pause();
    sink
}

fn resume(sink: &rodio::Sink) -> &rodio::Sink {
    sink.play();
    sink
}
