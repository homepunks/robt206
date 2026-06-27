use opus::{Channels, Decoder};
use std::io::Cursor;

fn demux_ogg(audio_raw: &[u8]) -> anyhow::Result<Vec<Vec<u8>>> {
    let cursor = Cursor::new(audio_raw);
    let mut reader = ogg::PacketReader::new(cursor);

    let mut packets = Vec::new();
    while let Some(packet) = reader.read_packet()? {
        packets.push(packet.data);
    }

    Ok(packets)
}

fn decode_opus_packets(packets: &[Vec<u8>]) -> anyhow::Result<Vec<f32>> {
    let mut decoder = Decoder::new(48_000, Channels::Mono)?;
    let mut pcm = Vec::new();
    let mut frame = [0f32; 5760]; /* 48k * .12 */

    for p in packets {
        if p.starts_with(b"OpusHead") || p.starts_with(b"OpusTags") {
            continue;
        }

        let n = decoder.decode_float(p, &mut frame, false)?;
        pcm.extend_from_slice(&frame[..n]);
    }

    Ok(pcm)
}

pub fn decode_voice(audio_raw: &[u8]) -> anyhow::Result<Vec<f32>> {
    let packets = demux_ogg(audio_raw)?;
    let pcm = decode_opus_packets(&packets)?;
    Ok(pcm)
}
