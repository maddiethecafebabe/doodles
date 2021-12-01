use std::env;
use std::fs;
use std::io;
use std::ops;
use std::path::Path;

pub struct Input(String);

impl Input {
    pub fn read_from<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        fs::read_to_string(path).map(Self)
    }

    pub fn from_str<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }

    pub fn from_env_args() -> io::Result<Self> {
        Self::read_from(env::args().nth(1).expect("no filename given"))
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn into_lines(&self) -> Vec<String> {
        self.0
            .split("\n")
            .into_iter()
            .map(|s| s.to_owned())
            .collect()
    }

    pub fn into_num_vec(&self) -> Vec<usize> {
        self.0
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(str::parse)
            .map(Result::unwrap)
            .collect()
    }
}

impl ops::Deref for Input {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
