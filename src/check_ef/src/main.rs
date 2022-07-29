use anyhow::Result;
use check_ef::{read_energies, read_forces, Matrix};
use clap::Parser;
use paris::Logger;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[clap(
    about = "Check convergence of vasp geometry optimization",
    version,
    author
)]
struct Options {
    /// The directory where the calculation is performed.
    #[clap(short, long, default_value = ".")]
    dir: String,
    /// If turned on, forces are not checked.
    #[clap(short = 'f', long, action)]
    no_force: bool,
    /// If turned on, results are written to "convergence.dat".
    #[clap(short = 'w', long, action)]
    write_results: bool,
}

fn main() -> Result<()> {
    let opts = Options::parse();
    let mut log = Logger::new();
    let mut header = format!("{:<15}", "Step");

    let dir = Path::new(&opts.dir);

    let poscar = read_file(&dir.join("POSCAR")).expect("POSCAR not found");
    let oszicar = read_file(&dir.join("OSZICAR")).expect("OSZICAR not found");

    // Read forces
    let max_forces = if !opts.no_force {
        let outcar = read_file(&dir.join("OUTCAR")).expect("OUTCAR not found");
        let forces_list = read_forces(&poscar, &outcar);

        // Calculate max forces
        let mask = get_mask(&poscar); // fixed atoms are 0.0
        let max_forces: Vec<f64> = forces_list
            .iter()
            .map(|forces| calculate_max_force(forces, &mask))
            .collect();
        write!(header, "{:<15}", "F_max (eV/A)")?;
        // header.push_str(&format!("{:<15}", "F_max (eV/A)"));
        Some(max_forces)
    } else {
        None
    };
    write!(header, "{:<15}", "E_0 (eV)")?;
    write!(header, "{:<15}", "E - E0 (eV)")?;
    write!(header, "{:<15}", "dE (eV)")?;

    // Read energies from oszicar
    let energies = read_energies(&oszicar);
    if energies.is_empty() {
        log.error("No SCF loop found.");
        std::process::exit(1);
    }

    // Calculate relative energies
    let rel_energies: &Vec<f64> = &energies.iter().map(|e| e - energies[0]).collect();

    let delta_energies: Vec<f64> = {
        let mut _padded_e = vec![0.0];
        _padded_e.extend(&energies);
        _padded_e[..].windows(2).map(|x| x[1] - x[0]).collect()
    };

    // Pretty print values
    println!("{}", header);
    println!("{}", vec!["-"; header.len()].join(""));
    let mut lines = vec![];
    for i in 0..energies.len() {
        let mut line = format!("{:<15}", i + 1);
        if let Some(ref max_forces) = max_forces {
            write!(line, "{:<15.6}", max_forces[i])?;
        }
        write!(line, "{:<15.6}", energies[i])?;
        write!(line, "{:<15.6}", rel_energies[i])?;
        write!(line, "{:<15.6}", delta_energies[i])?;
        println!("{}", line);
        lines.push(line);
    }
    if opts.write_results {
        let mut file = File::create(&dir.join("convergence.dat"))?;
        file.write_all(header.as_bytes())?;
        file.write_all(b"\n")?;
        file.write_all(lines.join("\n").as_bytes())?;
    }
    Ok(())
}

fn get_mask(poscar: &str) -> Vec<f64> {
    let mut mask = vec![];
    let poscar_lines: Vec<&str> = poscar.split('\n').collect();
    let n_atoms = poscar_lines[6]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .sum();
    if poscar_lines[7].starts_with('s') {
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
    mask
}

fn calculate_max_force(forces: &Matrix<f64>, mask: &[f64]) -> f64 {
    let magnitudes = forces
        .iter()
        .zip(mask.iter())
        .map(|(f, &c)| c * (f[0].powi(2) + f[1].powi(2) + f[2].powi(2)).sqrt());

    magnitudes.fold(0.0, |acc, x| acc.max(x))
}

fn read_file(filename: &PathBuf) -> Result<String> {
    let mut buf = String::new();
    let mut f = File::open(filename)?;
    f.read_to_string(&mut buf)?;
    Ok(buf)
}
