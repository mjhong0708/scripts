// use crate::structure::Structure;
// use nalgebra::{Const, DMatrix, Dynamic, Matrix3, OMatrix};
// use regex::Regex;
// use std::fs::File;
// use std::io::Read;

// impl Structure {
//     pub fn from_poscar(filename: &str) {
//         let mut file = File::open(filename).unwrap();
//         let mut contents = String::new();
//         file.read_to_string(&mut contents).unwrap();

//         let mut chunks = {
//             let re = Regex::new(r"\n\s*\n").unwrap();
//             re.split(&contents)
//                 .map(|x| x.to_string())
//                 .collect::<Vec<String>>()
//         };
//         if chunks.len() == 0 {
//             panic!("Empty POSCAR");
//         }
//         if chunks[0] == "" {
//             chunks[0] = format!("\n{}", chunks[0]);
//         }

//         let mut lines = chunks[0].lines();
//         let _comment = lines.next().unwrap();
//         let scale = lines.next().unwrap().parse::<f64>().unwrap();

//         let cell = {
//             let mut lattice_vectors = Vec::new();
//             for _ in 0..3 {
//                 lattice_vectors.extend(
//                     lines
//                         .next()
//                         .unwrap()
//                         .split_whitespace()
//                         .map(|x| scale * x.parse::<f64>().unwrap())
//                         .collect::<Vec<f64>>(),
//                 );
//             }
//             Matrix3::from_vec(lattice_vectors)
//         };
//         let unique_elems = parse_str_to_vec::<String>(lines.next().unwrap());
//         let num_elems = parse_str_to_vec::<usize>(lines.next().unwrap());
//         let n_atoms = num_elems.iter().sum::<usize>() as u32;
//         let chemical_symbols = {
//             let mut _chemical_symbols = Vec::new();
//             for i in 0..unique_elems.len() {
//                 _chemical_symbols.extend(vec![unique_elems[i].clone(); num_elems[i]]);
//             }
//             _chemical_symbols
//         };
//         let coords: OMatrix<f64, Dynamic, Const<3>> = {
//             let mut _coords = vec![];
//             for _ in 0..n_atoms {
//                 _coords.extend(parse_str_to_vec::<f64>(lines.next().unwrap()));
//             }
//             DMatrix::from_vec(_coords)
//         };
//         let charge = 0.0;
//         let structure = Structure {
//             cell,
//             coords,
//             elements: chemical_symbols,
//             charge,
//         };

//         structure
//     }
// }

// fn parse_str_to_vec<T>(s: &str) -> Vec<T>
// where
//     T: std::str::FromStr,
//     <T as std::str::FromStr>::Err: std::fmt::Debug,
// {
//     s.split_whitespace()
//         .map(|x| x.parse::<T>().unwrap())
//         .collect::<Vec<T>>()
// }
