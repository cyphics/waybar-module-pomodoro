use std::error::Error;

use crate::services::timer::CycleType;

fn play_sound(path: &str) -> Result<(), Box<dyn Error>> {
    println!("playing {}", path);
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let file = std::fs::File::open(path)?;
    sink.append(rodio::Decoder::try_from(file)?);

    sink.sleep_until_end();

    Ok(())
}

fn handle_play_sound(path: &str) {
    let result = play_sound(path);
    match result {
        Ok(_) => println!("Played sound {}", path),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn play_notification_sound(cycle_type: &CycleType) {
    match cycle_type {
        CycleType::Work => handle_play_sound("assets/bell.wav"),
        CycleType::ShortBreak => handle_play_sound("assets/bell.wav"),
        CycleType::LongBreak => handle_play_sound("assets/bell.wav"),
    };
}
