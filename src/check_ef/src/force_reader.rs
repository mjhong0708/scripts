use crate::types::Matrix;

pub fn read_forces(poscar: &str, outcar: &str) -> Vec<Matrix<f64>> {
    let poscar_lines = poscar.lines().collect::<Vec<_>>();
    let outcar_lines = outcar.lines().collect::<Vec<_>>();
    let n_atoms: usize = poscar_lines[6]
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .sum();

    let force_lines_idx: Vec<usize> = outcar
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("TOTAL-FORCE"))
        .map(|(i, _)| i)
        .collect();

    let mut forces_list = vec![];
    for i in force_lines_idx {
        let block = outcar_lines[i + 2..i + 2 + n_atoms].join("\n");
        let forces = read_force_block(&block);
        forces_list.push(forces);
    }
    forces_list
}

fn read_force_block(block: &str) -> Matrix<f64> {
    let forces = block.lines().map(|line| {
        line.split_whitespace()
            .skip(3)
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>()
    });
    forces.collect()
}
