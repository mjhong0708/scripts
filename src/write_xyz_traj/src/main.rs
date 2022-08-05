use libscripts::io::xyz;
use libscripts::parser::{outcar, poscar};
use rayon::prelude::*;
use std::fs::read_to_string;
use std::io::Write;

fn xyz_from_vasp(poscar: &str, outcar: &str) -> Vec<xyz::XYZBlock> {
    let cell = poscar::read_unit_cell(poscar);
    let unique_elems: Vec<&str> = poscar.lines().nth(5).unwrap().trim().split_whitespace().collect();
    let n_per_elem: Vec<usize> = poscar
        .lines()
        .nth(6)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let elem_vec: Vec<String> = unique_elems
        .iter()
        .zip(n_per_elem.iter())
        .map(|(&x, y)| vec![x; *y])
        .flatten()
        .map(|x| x.to_string())
        .collect();

    let trajectory = outcar::read_trajectory(poscar, outcar);
    trajectory
        .into_par_iter()
        .map(|frame| xyz::XYZBlock {
            cell: Some(cell.clone()),
            elements: elem_vec.clone(),
            positions: frame,
        })
        .collect()
}

fn main() {
    let poscar = read_to_string("POSCAR").expect("POSCAR not found");
    let outcar = read_to_string("OUTCAR").expect("OUTCAR not found");
    let xyz = xyz_from_vasp(&poscar, &outcar);

    // Write each frame in traj.xyz file
    let mut file = std::fs::File::create("traj.xyz").expect("traj.xyz not created");
    for frame in xyz {
        file.write(format!("{}\n", frame).as_bytes()).unwrap();
    }
}
