use std::path::PathBuf;

pub struct PlaybackContext {
    pub globs: Vec<String>,
    pub chars_per_second: usize,
    pub path: PathBuf,
}

pub struct BuildContext {
    pub globs: Vec<String>,
    pub path: PathBuf,
    pub rewrite: bool,
}
