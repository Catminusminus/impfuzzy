use goblin::pe::PE;
use ssdeep;
use std::io::Read;
#[derive(Debug)]
pub enum Error {
    Parse(goblin::error::Error),
    SsdeepHashing,
    IO(std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

impl From<goblin::error::Error> for Error {
    fn from(err: goblin::error::Error) -> Error {
        Error::Parse(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

fn trim_end_matches_with_strings<'a>(string: &'a str, array: &[&str]) -> &'a str {
    for suffix in array {
        if string.ends_with(suffix) {
            return string.trim_end_matches(suffix);
        }
    }
    string
}

pub fn hash(bytes: &[u8]) -> Result<String> {
    let pe = PE::parse(bytes)?;
    let vec: Vec<_> = pe
        .imports
        .iter()
        .map(|import| {
            (trim_end_matches_with_strings(import.dll, &[".ocx", ".sys", ".dll"]).to_owned() + ".")
                + &import.name
        })
        .collect();
    let h = ssdeep::hash(vec.join(",").to_lowercase().as_bytes()).ok_or(Error::SsdeepHashing)?;
    Ok(h)
}

pub fn hash_from_file(file_path: impl AsRef<std::path::Path>) -> Result<String> {
    let mut vec = Vec::new();
    let mut file = std::fs::File::open(file_path)?;
    file.read_to_end(&mut vec)?;
    hash(&vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trim_succeeded() {
        assert_eq!(
            trim_end_matches_with_strings("a.dll", &[".dll", ".sys"]),
            "a"
        );
    }
    #[test]
    fn trim_failed() {
        assert_eq!(
            trim_end_matches_with_strings("a.dlll", &[".dll", ".sys"]),
            "a.dlll"
        );
    }
}
