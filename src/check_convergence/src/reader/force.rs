use super::util::*;
use rayon::prelude::*;

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
