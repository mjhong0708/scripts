pub fn read_energies(oszicar: &str) -> Vec<f64> {
    let re_e0 = regex::Regex::new(r"E0=\s+[\d+]?\+?-?.\d+E[\+-]?\d+").unwrap();
    let result_lines = oszicar.lines().filter(|line| line.contains("E0"));
    let mut energies = Vec::new();
    for line in result_lines {
        let line = line.to_string();
        if let Some(capture) = re_e0.captures(&line) {
            let energy_chunk = capture
                .get(0)
                .unwrap()
                .as_str()
                .trim()
                .parse::<String>()
                .unwrap();
            let energy = energy_chunk.split('=').collect::<Vec<&str>>()[1]
                .trim()
                .parse::<f64>()
                .unwrap();
            energies.push(energy);
        }
    }
    energies
}
