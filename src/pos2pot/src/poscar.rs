use std::fs::File;
use std::io::Read;

pub fn extract_elements(poscar_filepath: &str) -> Vec<String> {
    let mut poscar_file = File::open(poscar_filepath)
        .expect("Failed to open POSCAR file. No file found, or not a regular file.");
    let mut poscar = String::new();
    // Read the file contents into a string
    poscar_file
        .read_to_string(&mut poscar)
        .expect("Failed to read POSCAR file");
    // Extract the elements from the POSCAR file
    poscar
        .lines()
        .nth(5)
        .expect("Failed to read elements line.")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_elements() {
        let elements = extract_elements("tests/POSCAR");
        assert_eq!(elements, vec!["Pt", "Cu"]);
    }
}
