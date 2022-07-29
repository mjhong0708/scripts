use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::*;
use home::home_dir;
use std::process::{exit, Command};
use std::{env, fs};

const OWNERS_URL: &str = "https://gist.githubusercontent.com/mjhong0708/ac46c65b08111d691aab2177cbbe468b/raw/364da33a6e19aae0290c44dc04219e1aec3bc02d/node_owners.csv";

fn main() {
    let user_home = match home_dir() {
        Some(x) => x,
        None => {
            panic!("Could not find home directory");
        }
    };
    let node_owners_file = user_home.join(".config/node_owners.csv");
    let update: Option<String> = env::args().nth(1);

    match update {
        Some(update) => {
            if update == String::from("update") {
                let _ = Command::new("curl")
                    .args(&["-s", OWNERS_URL, "-o", node_owners_file.to_str().unwrap()])
                    .status();
                println!("Successfully updated node owners file");
                exit(0);
            } else {
                panic!("Invalid argument\nUsage: ndstat [update]");
            }
        }
        None => {
            if !node_owners_file.exists() {
                let _ = Command::new("curl")
                    .args(&["-s", OWNERS_URL, "-o", node_owners_file.to_str().unwrap()])
                    .status();
            }
        }
    }

    let mut table = Table::new();
    let node_owners = fs::read_to_string(node_owners_file).unwrap();
    let node_owners: Vec<&str> = node_owners
        .lines()
        .skip(1)
        .map(|s| s.split(',').last().unwrap())
        .collect();

    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(90)
        .set_header(vec![
            Cell::new("Node").add_attribute(Attribute::Bold),
            Cell::new("Type").add_attribute(Attribute::Bold),
            Cell::new("Status").add_attribute(Attribute::Bold),
            Cell::new("Using\ncores").add_attribute(Attribute::Bold),
            Cell::new("Total\ncores").add_attribute(Attribute::Bold),
            Cell::new("User").add_attribute(Attribute::Bold),
            Cell::new("Owner").add_attribute(Attribute::Bold),
        ]);

    let pestat_output = {
        let out = Command::new("sh")
            .args(["-c", "pestat -c | awk '{print $1,$2,$3,$4,$5,$10}'"])
            .output()
            .unwrap()
            .stdout;
        String::from_utf8(out).unwrap()
    };

    for (line, owner) in pestat_output.lines().skip(3).zip(node_owners.iter()) {
        let mut row: Vec<&str> = line.split_whitespace().collect();
        let mut color = Color::Reset;
        if row.len() == 5 {
            row.push("None");
            color = Color::Green;
        }
        if row[5] == std::env::var("USER").unwrap() {
            color = Color::Blue;
        }
        if row[2] == "down*" {
            color = Color::Red;
        }
        row.push(*owner);

        let cells = row.iter().map(|&s| Cell::new(s).fg(color));

        table.add_row(cells);
    }
    println!("{}", table);
}
