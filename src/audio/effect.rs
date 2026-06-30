pub fn chipmunk(pcm: &[f32], speed: f32) -> Vec<f32> {
    let out_len = (pcm.len() as f32 / speed) as usize;
    let mut out = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let pos = i as f32 * speed;
        let idx = pos.floor() as usize;
        let frac = pos - idx as f32;

        if idx + 1 >= pcm.len() {
            break;
        }

        let sample = pcm[idx] * (1f32 - frac) + pcm[idx + 1] * frac;
        out.push(sample);
    }

    out
}

pub fn reverse(pcm: &[f32]) -> Vec<f32> {
    pcm.iter().rev().copied().collect()
}
