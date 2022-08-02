use rayon::prelude::*;

fn n_atoms_in_poscar(poscar: &str) -> usize {
    poscar
        .lines()
        .nth(6)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .sum()
}

fn get_mask(poscar: &str, n_atoms: usize) -> Vec<f64> {
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

fn read_force_block(block: &str, mask: &[f64]) -> Vec<Vec<f64>> {
    block
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.split_ascii_whitespace()
                .skip(3)
                .map(|x| mask[i] * x.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn read_energies(oszicar: &str) -> Vec<f64> {
    let re_e0 = regex::Regex::new(r"E0=\s+[\d+]?\+?-?.\d+E[\+-]?\d+").unwrap();
    let res = oszicar
        .lines()
        .filter(|&line| line.contains("E0"))
        .map(|line| {
            re_e0
                .captures(line)
                .and_then(|cap| cap.get(0))
                .and_then(|m| m.as_str().split("=").last())
                .and_then(|s| s.trim().parse::<f64>().ok())
        });
    res.map(|x| x.unwrap()).collect()
}
pub fn read_forces(poscar: &str, outcar: &str) -> Vec<Vec<Vec<f64>>> {
    let n_atoms: usize = n_atoms_in_poscar(poscar);
    let mask = get_mask(&poscar, n_atoms);
    let outcar_lines = outcar.lines().collect::<Vec<_>>();

    outcar_lines
        .par_iter()
        .enumerate()
        .filter(|(_, line)| line.contains("TOTAL-FORCE"))
        .map(|(i, _)| i)
        .map(|i| {
            let block = &outcar_lines[i + 2..i + 2 + n_atoms].join("\n");
            read_force_block(&block, &mask)
        })
        .collect()
}

pub fn calculate_max_force(forces: &[Vec<f64>]) -> f64 {
    forces
        .iter()
        .map(|f| f[0].powi(2) + f[1].powi(2) + f[2].powi(2))
        .max_by(|&x, &y| x.total_cmp(&y))
        .unwrap()
        .sqrt()
}
