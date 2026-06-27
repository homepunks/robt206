use ogg::{PacketWriteEndInfo, PacketWriter};
use opus::{Application, Channels, Encoder};

fn encode_opus_packets(pcm: &[f32]) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut encoder = Encoder::new(48_000, Channels::Mono, Application::Voip)?;
    let mut packets = Vec::new();
    let mut out = [0u8; 4000];

    for chunk in pcm.chunks(960) {
        /* 20ms @ 48kHz */
        let mut frame = [0f32; 960];
        frame[..chunk.len()].copy_from_slice(chunk);

        let n = encoder.encode_float(&frame, &mut out)?;
        packets.push(out[..n].to_vec());
    }

    Ok(packets)
}

fn mux_ogg(packets: &[Vec<u8>]) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    {
        let mut writer = PacketWriter::new(&mut buf);
        let serial: u32 = 0xC0FFEE;

        writer.write_packet(opus_head(), serial, PacketWriteEndInfo::EndPage, 0)?;
        writer.write_packet(opus_tags(), serial, PacketWriteEndInfo::EndPage, 0)?;

        let last = packets.len() - 1;
        for (i, p) in packets.iter().enumerate() {
            let granule = (i as u64 + 1) * 960;
            let info = if i == last {
                PacketWriteEndInfo::EndStream
            } else {
                PacketWriteEndInfo::NormalPacket
            };
            writer.write_packet(p.clone(), serial, info, granule)?;
        }
    }

    Ok(buf)
}

pub fn encode_voice(pcm: &[f32]) -> anyhow::Result<Vec<u8>> {
    let packets = encode_opus_packets(pcm)?;
    let oga = mux_ogg(&packets)?;
    Ok(oga)
}

fn opus_head() -> Vec<u8> {
    let mut h = Vec::new();
    h.extend_from_slice(b"OpusHead");
    h.push(1);
    h.push(1);
    h.extend_from_slice(&0u16.to_le_bytes());
    h.extend_from_slice(&48_000u32.to_le_bytes());
    h.extend_from_slice(&0i16.to_le_bytes());
    h.push(0);
    h
}

fn opus_tags() -> Vec<u8> {
    let mut t = Vec::new();
    let vendor = b"robt206";
    t.extend_from_slice(b"OpusTags");
    t.extend_from_slice(&(vendor.len() as u32).to_le_bytes());
    t.extend_from_slice(vendor);
    t.extend_from_slice(&0u32.to_le_bytes());
    t
}
