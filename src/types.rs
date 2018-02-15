use std::path::PathBuf;

pub struct PlaybackContext {
    pub chars_per_second: usize,
    pub path: PathBuf,
}

pub struct BuildContext {
    pub path: PathBuf,
    pub rewrite: bool,
}
