use clap::Parser;
use std::fs::File;
use std::io::Write;

const SHEBANG: &str = "#!/bin/bash\n";

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short = 'v', long)]
    vasp_version: String,
    #[clap(short = 't', long)]
    vasp_type: String,
    #[clap(short = 'J', long)]
    job_name: String,
    #[clap(short = 'N', long)]
    nnodes: u32,
    #[clap(short = 'n', long)]
    ntasks_per_node: u32,
    #[clap(short = 'p', long)]
    node_partition: String,
    #[clap(long)]
    nodelist: Option<String>,
}

fn main() -> std::io::Result<()> {
    write_job_script()
}

fn write_job_script() -> std::io::Result<()> {
    let args = Args::parse();
    let filename = "job-script-vasp.sh";
    // create file and write shebang
    let mut file = File::create(filename)?;
    file.write_all(SHEBANG.as_bytes())?;
    // write job name
    file.write_all(format!("#SBATCH -J {}\n", args.job_name).as_bytes())?;
    // write node partition
    file.write_all(format!("#SBATCH -p {}\n", args.node_partition).as_bytes())?;
    // write number of nodes
    file.write_all(format!("#SBATCH -N {}\n", args.nnodes).as_bytes())?;
    // write number of tasks per node
    file.write_all(format!("#SBATCH --ntasks-per-node {}\n", args.ntasks_per_node).as_bytes())?;
    // io
    file.write_all("#SBATCH -o stdout_%j.log\n".as_bytes())?;
    file.write_all("#SBATCH -e stderr_%j.log\n".as_bytes())?;
    // write nodelist
    if let Some(nodelist) = args.nodelist {
        file.write_all(format!("#SBATCH --nodelist={}\n\n\n", nodelist).as_bytes())?;
    }

    // load modules
    file.write_all("module purge\n".as_bytes())?;
    file.write_all("module add compiler/2022.1.0\n".as_bytes())?;
    file.write_all("module add mkl/2022.1.0\n".as_bytes())?;
    file.write_all("module add mpi/2021.6.0\n\n".as_bytes())?;

    let version_var = args.vasp_version.replace('.', "_");
    let vasp_dir = std::env::var(format!("VASP_{}_DIR", version_var));
    match vasp_dir {
        Ok(vasp_dir) => {
            let vasp_bin = format!(
                "{}/vasp.{}.{}.x",
                vasp_dir, args.vasp_version, args.vasp_type,
            );

            file.write_all(
                format!("mpirun -np $SLURM_NTASKS {} > stdout.log", vasp_bin).as_bytes(),
            )?;
        }
        Err(e) => {
            eprintln!(
                "It seems that you did not set the environment variable VASP_{}_DIR",
                version_var
            );
            panic!("{}", e);
        }
    }

    Ok(())
}
