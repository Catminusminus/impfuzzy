use goblin::error;
use goblin::pe::PE;
use ssdeep;
use std::io::Read;

pub fn hash(bytes: &[u8]) -> error::Result<String> {
    let pe = PE::parse(bytes)?;
    let vec: Vec<_> = pe
        .imports
        .iter()
        .map(|import| (String::from(import.dll.trim_end_matches(".dll")) + ".") + &import.name)
        .collect();
    let h = ssdeep::hash(vec.join(",").to_lowercase().as_bytes())
        .ok_or(error::Error::Malformed("ssdeep failed".to_owned()))?;
    Ok(h)
}

pub fn hash_from_file(file_path: impl AsRef<std::path::Path>) -> error::Result<String> {
    let mut vec = Vec::new();
    let mut file = std::fs::File::open(file_path)?;
    file.read_to_end(&mut vec)?;
    hash(&vec)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
