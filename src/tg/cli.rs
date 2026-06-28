pub enum Cli {
    CHIPMUNK,
}

pub fn detect_cmd(text: &str) -> Option<Cli> {
    let text = text.strip_prefix('!');
    if let Some(cmd) = text {
        match cmd {
            "chip" => Some(Cli::CHIPMUNK),
            _ => None,
        }
    } else {
        None
    }
}
