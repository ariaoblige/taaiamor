use std::{thread,time};
use std::fs::File;
use std::io::BufReader;
use colored::Colorize;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let (_strm,audio_stream) = OutputStream::try_default().unwrap();
    let ta_ai_amor_audio     = BufReader::new(File::open("taaiamor.ogg").unwrap());
    let source               = Decoder::new(ta_ai_amor_audio).unwrap();
    
    let _audio = audio_stream.play_raw(source.convert_samples());

    let tempos: [u64;8] = [
        1996,
        1790,
        5590,
        3630,
        3088,
        1135,
        5619,
        14894,
    ];
    let ta_ai_amor: [String; 8] = [
        "Ta aí amor".bright_blue().to_string(),
        "Oiee ".bright_green().to_string()+&emojis::get("😊").unwrap().to_string(),
        "vou sair tá? ".bright_blue().to_string()+&emojis::get("🤞").unwrap().to_string(),
        "Oiie".bright_green().to_string()+&emojis::get("☺️").unwrap().to_string(),
        "Oii, vou sair tá?".bright_blue().to_string(),
        "Oiie ".bright_green().to_string()+&emojis::get("😊").unwrap().to_string(),
        "Amor pq vc só fala oi? ".bright_blue().to_string()+&emojis::get("🤔").unwrap().to_string(),
        "Pq se eu falar tchau você vai embora ".bright_green().to_string()+&emojis::get("😢").unwrap().to_string(),
    ];

    thread::sleep(time::Duration::from_millis(4700));
    
    for num in 0..8 {
        println!("{}",ta_ai_amor[num]);

        thread::sleep(time::Duration::from_millis(tempos[num]));
    }
}
