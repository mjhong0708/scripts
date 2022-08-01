use ndarray::prelude::*;
use ndarray::stack;

fn n_atoms_in_poscar(poscar: &str) -> usize {
    poscar
        .lines()
        .nth(6)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .sum()
}

fn get_mask(poscar: &str, n_atoms: usize) -> Array1<f64> {
    let vals: Vec<f64> = match poscar.lines().nth(7).unwrap().starts_with('S') {
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
    };

    Array::from_vec(vals)
}

fn read_force_block(block: &str, n_atoms: usize, mask: &Array1<f64>) -> Array2<f64> {
    let force_components = block
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(3)
                .map(|x| x.parse::<f64>().unwrap())
        })
        .flatten();
    let forces = Array::from_iter(force_components)
        .into_shape((n_atoms, 3))
        .unwrap();
    forces * mask.view().into_shape((n_atoms, 1)).unwrap()
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
pub fn read_forces(poscar: &str, outcar: &str) -> Array3<f64> {
    let n_atoms: usize = n_atoms_in_poscar(poscar);
    let mask = get_mask(&poscar, n_atoms);
    let forces_list: Vec<Array2<f64>> = outcar
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("TOTAL-FORCE"))
        .map(|(i, _)| i)
        .map(|i| {
            let block = outcar
                .lines()
                .skip(i + 2)
                .take(n_atoms)
                .collect::<Vec<&str>>()
                .join("\n");
            read_force_block(&block, n_atoms, &mask)
        })
        .collect();

    // create Array3 from forces_list
    let forces = stack(
        Axis(0),
        &forces_list.iter().map(|x| x.view()).collect::<Vec<_>>(),
    )
    .unwrap();
    forces
}
