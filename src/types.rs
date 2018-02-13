use std::path::PathBuf;

pub struct PlaybackContext {
    pub path: PathBuf,
}

pub struct BuildContext {
    pub path: PathBuf,
    pub rewrite: bool,
}
