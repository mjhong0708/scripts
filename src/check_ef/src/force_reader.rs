use ndarray::{stack, Array, Array1, Array2, Array3, Axis};

pub fn read_forces(poscar: &str, outcar: &str) -> Array3<f64> {
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

    let mask = get_mask(&poscar).into_shape((n_atoms, 1)).unwrap();
    let forces_list: Vec<Array2<f64>> = force_lines_idx
        .iter()
        .map(|i| {
            let block = outcar_lines[i + 2..i + 2 + n_atoms].join("\n");
            // println!("{}", &mask);
            read_force_block(&block) * &mask
        })
        .collect();
    let forces_list = forces_list.iter().map(|x| x.view()).collect::<Vec<_>>();

    // create Array3 from forces_list
    let forces = stack(
        Axis(0),
        &forces_list.iter().map(|x| x.view()).collect::<Vec<_>>(),
    )
    .unwrap();
    forces
}

fn read_force_block(block: &str) -> Array2<f64> {
    // Split each line in block by whitespace
    let mut force_components = vec![];
    block.lines().for_each(|line| {
        force_components.extend(
            line.split_whitespace()
                .skip(3)
                .map(|x| x.parse::<f64>().unwrap()),
        );
    });
    let num_atoms = force_components.len() / 3;

    Array2::from_shape_vec((num_atoms, 3), force_components).unwrap()
}

fn get_mask(poscar: &str) -> Array1<f64> {
    let mut mask = vec![];
    let poscar_lines: Vec<&str> = poscar.split('\n').collect();
    let n_atoms = poscar_lines[6]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .sum();
    if poscar_lines[7].to_lowercase().starts_with('s') {
        for line in poscar_lines.iter().skip(9).take(n_atoms as usize) {
            if line.contains('F') {
                mask.push(0.0);
            } else {
                mask.push(1.0);
            }
        }
    } else {
        for _ in 0..n_atoms {
            mask.push(1.0);
        }
    }
    Array::from_shape_vec((n_atoms as usize,), mask).unwrap()
}
