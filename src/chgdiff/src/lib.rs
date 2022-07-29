use anyhow::Result;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{BufWriter, Read, Write};

#[derive(Debug)]
pub struct GridSpec {
    pub nx: u32,
    pub ny: u32,
    pub nz: u32,
}
impl GridSpec {
    pub fn new(nx: u32, ny: u32, nz: u32) -> Self {
        GridSpec { nx, ny, nz }
    }
    pub fn n_points(&self) -> u32 {
        self.nx * self.ny * self.nz
    }
}
impl PartialEq for GridSpec {
    fn eq(&self, other: &Self) -> bool {
        self.nx == other.nx && self.ny == other.ny && self.nz == other.nz
    }
}

pub struct Chgcar {
    pub poscar: String,
    pub grid_spec: GridSpec,
    pub data: Vec<f64>, // 1D format, does not respect grid shape
}

impl Chgcar {
    pub fn from_file(filename: &str) -> Result<Chgcar, String> {
        let mut file = File::open(filename).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| e.to_string())?;

        let lines = contents.lines().collect::<Vec<&str>>();
        let n_atoms: u32 = lines[6]
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .sum();

        let poscar_endpoint = (8 + n_atoms - 1) as usize;
        let poscar = lines[..=poscar_endpoint].join("\n");

        let grid_spec = {
            let _line = lines[poscar_endpoint + 2];
            let ngrids = _line
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            GridSpec {
                nx: ngrids[0],
                ny: ngrids[1],
                nz: ngrids[2],
            }
        };

        let mut count = 0;
        let mut data: Vec<f64> = Vec::new();
        for line in lines[poscar_endpoint + 3..].iter() {
            let _line = line.trim();
            if _line.is_empty() {
                continue;
            }
            let _data = _line
                .split_ascii_whitespace()
                .map(|x| x.parse::<f64>().unwrap())
                .collect::<Vec<f64>>();
            count += _data.len() as u32;
            data.extend(_data);
            if count == grid_spec.n_points() {
                break;
            }
        }
        Ok(Chgcar {
            poscar,
            grid_spec,
            data,
        })
    }

    pub fn write_file(&self, filename: &str) -> Result<()> {
        let num_points = self.grid_spec.n_points();
        let num_rows = (num_points / 5) as usize;
        let num_remainder = (num_points % 5) as usize;

        let f = File::create(filename)?;
        let mut writer = BufWriter::new(f);
        let contents = {
            let mut contents = self.poscar.clone();
            writeln!(contents, "\n")?;
            writeln!(
                contents,
                "{} {} {}",
                self.grid_spec.nx, self.grid_spec.ny, self.grid_spec.nz
            )?;

            for i in 0..num_rows {
                let start = i * 5;
                let end = start + 5;
                let _data = &self.data[start..end];
                writeln!(
                    contents,
                    "{:.11E} {:.11E} {:.11E} {:.11E} {:.11E}",
                    _data[0], _data[1], _data[2], _data[3], _data[4]
                )?;
            }
            if num_remainder > 0 {
                let _data = &self.data[num_rows * 5..];
                for d in _data.iter().take(num_remainder) {
                    write!(contents, "{:.11E} ", d)?;
                }
                contents.push('\n');
            }
            contents
        };

        writer.write_all(contents.as_bytes()).unwrap();
        Ok(())
    }
}
