pub fn n_atoms_in_poscar(poscar: &str) -> usize {
    poscar
        .lines()
        .nth(6)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .sum()
}

pub fn get_mask(poscar: &str, n_atoms: usize) -> Vec<f64> {
    match poscar.lines().nth(7).unwrap().starts_with('S') {
        true => poscar
            .lines()
            .skip(9)
            .take(n_atoms)
            .map(|line| match line.contains("F") {
                true => 0.0,
                false => 1.0,
            })
            .collect(),
        false => vec![1.0; n_atoms],
    }
}
