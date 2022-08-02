use clap::Parser;

#[derive(Parser)]
#[clap(about = "Check convergence of vasp geometry optimization", version, author)]
pub struct Options {
    /// The directory where the calculation is performed.
    #[clap(short, long, default_value = ".")]
    pub dir: String,
    /// If turned on, forces are not checked.
    #[clap(short = 'f', long, action)]
    pub no_force: bool,
    /// If turned on, results are written to "convergence.dat".
    #[clap(short = 'w', long, action)]
    pub write_results: bool,
}
