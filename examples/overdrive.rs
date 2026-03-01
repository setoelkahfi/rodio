use rodio::source::{SineWave, Source};
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the default output stream and get the mixer
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let mixer = stream_handle.mixer();

    // Create a sine wave source and apply overdrive
    let overdriven = SineWave::new(440.0)
        .amplify(0.2)
        .overdrive(5.0, 0.8)
        .take_duration(Duration::from_secs(3));

    // Play the overdriven sound
    mixer.add(overdriven);

    println!("Playing overdriven sine wave for 3 seconds...");
    thread::sleep(Duration::from_secs(3));
    println!("Done.");

    Ok(())
}
