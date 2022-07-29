use qst::Job;

use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use std::str::FromStr;

fn main() {
    let jobinfo = {
        let out = std::process::Command::new("sh")
            .args(["-c", "scontrol show job"])
            .output()
            .unwrap()
            .stdout;
        String::from_utf8(out).unwrap()
    };

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(110)
        .set_header(vec![
            Cell::new("Job ID").add_attribute(Attribute::Bold),
            Cell::new("Job name").add_attribute(Attribute::Bold),
            Cell::new("State").add_attribute(Attribute::Bold),
            Cell::new("Partition").add_attribute(Attribute::Bold),
            Cell::new("Num.\nnodes").add_attribute(Attribute::Bold),
            Cell::new("Num.\ntasks").add_attribute(Attribute::Bold),
            Cell::new("Elapsed\ntime").add_attribute(Attribute::Bold),
        ]);

    for l in jobinfo.split("\n\n").filter(|&x| !x.is_empty()) {
        let job = Job::from_str(l).unwrap();
        if job.username == std::env::var("USER").unwrap() {
            let color = match job.state.as_str() {
                "RUNNING" => Color::Grey,
                "COMPLETED" => Color::Green,
                _ => Color::Reset,
            };

            let row = vec![
                Cell::new(job.id).fg(color),
                Cell::new(job.jobname).fg(color),
                Cell::new(job.state).fg(color),
                Cell::new(job.partition).fg(color),
                Cell::new(job.numnodes).fg(color),
                Cell::new(job.numtasks).fg(color),
                Cell::new(job.runtime).fg(color),
            ];
            table.add_row(row);
        }
    }

    println!("{}", table);
}
