use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use rodio::Source;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.wav")?;
    let source = rodio::Decoder::try_from(file)?;

    // Shared flag to enable/disable overdrive
    let overdrive_enabled = Arc::new(AtomicBool::new(true));
    let overdrive_enabled_clone = overdrive_enabled.clone();

    // Apply overdrive and alternate the effect during playback
    let overdriven =
        source
            .overdrive(10.0, 1.0)
            .periodic_access(Duration::from_millis(250), move |src| {
                let enable = overdrive_enabled_clone.load(Ordering::Relaxed);
                src.set_gain(if enable { 10.0 } else { 1.0 });
                src.set_color(if enable { 1.0 } else { 0.0 });
            });

    sink.append(overdriven);

    println!("Playing music.wav with alternating overdrive effect...");
    // Alternate the overdrive effect every two seconds for 10 cycles
    for _ in 0..10 {
        thread::sleep(Duration::from_secs(2));
        let prev = overdrive_enabled.load(Ordering::Relaxed);
        overdrive_enabled.store(!prev, Ordering::Relaxed);
        println!("Overdrive {}", if !prev { "ON" } else { "OFF" });
    }

    // Wait for playback to finish
    sink.sleep_until_end();

    Ok(())
}
