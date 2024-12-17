use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: i32,

    #[arg(short, long)]
    day: i32,
}

fn main() {
    let args = Args::parse();

    let day_file = format!("src/y{}/day{:02}.rs", args.year, args.day);
    let mod_file = format!("src/y{}/mod.rs", args.year);
    let main_file = "src/main.rs";

    let day_template = format!(
        r#"use crate::utils::AdventDay;

pub struct Day{:02} {{
    input: String,
}}

impl AdventDay for Day{:02} {{
    fn new(input: String) -> Self {{
        Self {{ input }}
    }}

    fn part_one(&self) -> String {{
        // Solve part 1 here
        "".to_string()
    }}

    fn part_two(&self) -> String {{
        // Solve part 2 here
        "".to_string()
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const DATA: &str = "";

    #[test]
    fn part_one() {{
        let day{:02} = Day{:02}::new(DATA.to_string());
        assert_eq!(day{:02}.part_one(), "");
    }}

    #[test]
    #[ignore = "not implemented"]
    fn part_two() {{
        let day{:02} = Day{:02}::new(DATA.to_string());
        assert_eq!(day{:02}.part_two(), "");
    }}
}}
"#,
        args.day, args.day, args.day, args.day, args.day, args.day, args.day, args.day
    );

    if Path::new(&day_file).exists() {
        eprintln!("Day file {} already exists!", day_file);
        return;
    }

    fs::create_dir_all(format!("src/y{}", args.year)).expect("Failed to create year directory");
    fs::write(&day_file, day_template).expect("Failed to write day file");
    println!("Created {}", day_file);

    let mod_entry = format!("pub mod day{:02};", args.day);
    let mut mod_content = fs::read_to_string(&mod_file).unwrap_or_else(|_| String::new());
    if !mod_content.contains(&mod_entry) {
        mod_content.push_str(&format!("{}\n", mod_entry));
        fs::write(&mod_file, mod_content).expect("Failed to update mod.rs");
        println!("Updated {}", mod_file);
    }

    let new_match_arm = format!(
        r#"        ({}, {}) => y{}::day{:02}::Day{:02}::new(input).run(),"#,
        args.year, args.day, args.year, args.day, args.day
    );

    let mut main_content = fs::read_to_string(main_file).expect("Failed to read main.rs");
    let insertion_marker =
        "        _ => println!(\"No implementation for year {} day {}\", year, day),";
    if let Some(pos) = main_content.find(insertion_marker) {
        main_content.insert_str(pos, &format!("{}\n", new_match_arm));
        fs::write(main_file, main_content).expect("Failed to update main.rs");
        println!("Updated {}", main_file);
    } else {
        eprintln!("Could not find insertion marker in main.rs!");
    }

    println!(
        "Day {:02} setup complete! File created: {}",
        args.day, day_file
    );
}
