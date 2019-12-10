use std::fs::File;
use std::io::BufReader;

const FILE: &str = "clear-as-water.mp3";

fn main() {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);

    let file = File::open(FILE).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}