use std::fs;

fn main() -> anyhow::Result<()> {
    let in_path = "cache/voice_779812434.oga";
    let bytes = fs::read(in_path)?;
    let pcm = robt206::audio::decode::decode_voice(&bytes)?;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 48_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("cache/out.wav", spec)?;
    for &s in &pcm {
        writer.write_sample((s * 32767.0) as i16)?;
    }
    writer.finalize()?;
    Ok(())
}
