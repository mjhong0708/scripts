use std::fs::File;
use std::io::Write;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 5 {
        println!("Usage: {} <grid_scheme> <nx> <ny> <nz>", args[0]);
        std::process::exit(1);
    }

    let grid_scheme = {
        let g = &args[1];
        if g.to_lowercase().starts_with('g') {
            "Gamma"
        } else {
            "Monkhorst-Pack"
        }
    };
    let nx = args[2].parse::<u32>().unwrap();
    let ny = args[3].parse::<u32>().unwrap();
    let nz = args[4].parse::<u32>().unwrap();

    let file_path = "KPOINTS";
    let mut file = File::create(file_path).unwrap();

    writeln!(file, "Autometic mesh").unwrap();
    writeln!(file, " 0").unwrap();
    writeln!(file, "{}", grid_scheme).unwrap();
    writeln!(file, " {}  {}  {}", nx, ny, nz).unwrap();
    writeln!(file, " 0  0  0").unwrap();
}
