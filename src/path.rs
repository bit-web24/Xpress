use std::env::current_dir;

pub struct Path {
    path: std::path::PathBuf,
}

impl Path {
    pub fn new<S: Into<std::path::PathBuf>>(path: S) -> Self {
        Self {
            path: current_dir().unwrap().join(path.into()),
        }
    }

    pub fn to_str(&self) -> &str {
        self.path.to_str().unwrap_or("")
    }

    pub fn join<S: Into<std::path::PathBuf>>(self, path: S) -> Self {
        Self {
            path: self.path.join(path.into()),
        }
    }
}
