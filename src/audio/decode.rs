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

// fn decode_opus_packets(packets: &[Vec<u8>]) -> anyhow::Result<Vec<f32>> {
//
// }
