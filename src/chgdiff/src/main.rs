use anyhow::Result;
use chgdiff::{Chgcar, GridSpec};
use clap::Parser;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(value_parser)]
    chgcar_file: String,
    #[clap(long = "ref1")]
    reference_1: String,
    #[clap(long = "ref2")]
    reference_2: String,
    #[clap(short = 'o', long = "out", default_value = "chgdiff.vasp")]
    output: String,
}

fn main() -> Result<()> {
    let start_time = Instant::now();
    let args = Args::parse();
    let chgcar = Chgcar::from_file(&args.chgcar_file).unwrap();
    let chgcar_ref_1 = Chgcar::from_file(&args.reference_1).unwrap();
    let chgcar_ref_2 = Chgcar::from_file(&args.reference_2).unwrap();
    println!(
        "Loaded chgcar files ({:.3}s)",
        start_time.elapsed().as_secs_f64()
    );

    // Print some info
    let n_points_target = chgcar.grid_spec.n_points();
    let n_points_ref_1 = chgcar_ref_1.grid_spec.n_points();
    let n_points_ref_2 = chgcar_ref_2.grid_spec.n_points();
    println!("The number of points in the grid: ");
    println!("{}: {}", args.chgcar_file, n_points_target);
    println!("{}: {}", args.reference_1, n_points_ref_1);
    println!("{}: {}", args.reference_2, n_points_ref_2);

    let grid_match =
        chgcar.grid_spec == chgcar_ref_1.grid_spec && chgcar.grid_spec == chgcar_ref_2.grid_spec;
    if !grid_match {
        panic!("The number of grids do not match");
    }

    // Calculate chgdiff
    println!("Writing chgdiff...");
    let poscar = &chgcar.poscar;
    let grid_spec = &chgcar.grid_spec;
    let data_orig = &chgcar.data;
    let data_ref_1 = &chgcar_ref_1.data;
    let data_ref_2 = &chgcar_ref_2.data;
    let chgdiff_data: Vec<f64> = data_orig
        .iter()
        .zip(data_ref_1.iter())
        .zip(data_ref_2.iter())
        .map(|((x, y), z)| x - y - z)
        .collect();
    let chgdiff = Chgcar {
        poscar: poscar.to_string(),
        grid_spec: GridSpec::new(grid_spec.nx, grid_spec.ny, grid_spec.nz),
        data: chgdiff_data,
    };

    chgdiff.write_file(&args.output)?;
    println!("Done");
    Ok(())
}
