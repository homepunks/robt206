pub enum Cli {
    CHIPMUNK,
    REVERSE,
    ROBOT,
}

pub fn detect_cmd(text: &str) -> Option<Cli> {
    let text = text.strip_prefix('!');
    if let Some(cmd) = text {
        match cmd {
            "chip"  => Some(Cli::CHIPMUNK),
            "rev"   => Some(Cli::REVERSE),
            "botik" => Some(Cli::ROBOT),
            _ => None,
        }
    } else {
        None
    }
}
