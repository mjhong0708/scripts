use clap::Parser;
use pos2pot::poscar::extract_elements;
use pos2pot::potcar::{get_potcars_from_map, get_recommended_potcars, prompt_potcars};
use std::cmp::Eq;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use paris::Logger;

/// Converts POSCAR to POTCAR.
/// Generated POTCAR will be written to the same directory as POSCAR.
/// If the POTCAR file already exists, it will be overwritten.
#[derive(Parser, Debug)]
#[clap(author,version,about,long_about = None)]
struct Args {
    /// The path to the POSCAR file. Defaults to "POSCAR".
    #[clap(short, long, default_value = "POSCAR")]
    filename: String,
    /// Manual mode or not. If manual mode is enabled, the user has to provide POTCARs to use,
    /// or will be prompted to select the POTCARs.
    /// Defaults to false, which means the program will automatically select the recommended POTCARs.
    #[clap(short, long)]
    manual: bool,
    /// The elements and corresponding POTCARs to be used. If not given, the program will
    /// prompt the user to select the POTCARs.
    #[clap(short, long, parse(try_from_str = parse_to_hashmap), required=false)]
    potcars: Option<HashMap<String, String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut log = Logger::new();
    let args = Args::parse();
    // File paths
    let poscar_dir = Path::new(&args.filename)
        .parent()
        .expect("Failed to get parent directory of POSCAR file.")
        .to_str()
        .expect("Not a valid path.");

    let potcar_path = match env::var("POTCAR_PATH_PREFIX") {
        Ok(p) => p,
        Err(_) => {
            panic!("The environment variable POTCAR_PATH_PREFIX is not set.");
        }
    };

    let potcar_destination = Path::new(poscar_dir).join("POTCAR");

    let elements = extract_elements(&args.filename);
    log.info(format!(
        "Found {} in {}",
        elements.join(", "),
        args.filename
    ));

    let potcar_names = match args.manual {
        false => get_recommended_potcars(&elements),
        true => match args.potcars {
            Some(p) => get_potcars_from_map(&elements, &p),
            None => prompt_potcars(&elements),
        },
    };
    log.info(format!("Using POTCARs: {}", potcar_names.join(", ")));

    let potcar_paths = potcar_names
        .into_iter()
        .map(|p| potcar_path.to_string() + "/" + &p + "/POTCAR");

    // Delete the POTCAR file if it exists
    if Path::new(&potcar_destination).exists() {
        fs::remove_file(&potcar_destination).expect("Failed to delete POTCAR file.");
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&potcar_destination)
        .unwrap();

    potcar_paths.for_each(|path| {
        let potcar_str = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read POTCAR file {}", &path));
        write!(file, "{}", potcar_str).expect("Failed to write to POTCAR file.");
    });
    log.info(format!(
        "<blue>All Done! Wrote POTCAR to {:?}</>",
        &potcar_destination
    ));
    Ok(())
}

fn parse_to_hashmap<T, U>(s: &str) -> Result<HashMap<T, U>, Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr + Hash + Eq,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let args: Vec<String> = s.split(',').map(|s| s.to_string()).collect();
    let mut map: HashMap<T, U> = HashMap::new();
    for arg in args {
        let key_val: Vec<&str> = arg.split('=').collect();
        if key_val.len() != 2 {
            panic!("Invalid argument: {}", arg);
        }
        map.insert(key_val[0].parse()?, key_val[1].parse()?);
    }
    Ok(map)
}
