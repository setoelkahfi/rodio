use rodio::Source;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.wav")?;
    let source = rodio::Decoder::try_from(file)?.overdrive(5.0, 0.8);

    sink.append(source);

    println!("Playing music.wav with overdrive until finished...");
    sink.sleep_until_end();
    println!("Done.");

    Ok(())
}
