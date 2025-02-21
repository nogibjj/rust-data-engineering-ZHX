/*A cli that generates random fruits */

use calc_cli_with_tests::get_fruits;
use clap::Parser;
use std::fs::File;
use std::io::Write;

/// CLI tool to return random fruits
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The quantity of fruits to return
    #[clap(short, long, default_value = "1")]
    count: u32,
    /// The path to the output file
    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let fruits = get_fruits(args.count);

    println!("fruits: {:?}", fruits);

    if let Some(output_path) = args.output {
        // create file automatically
        if let Ok(mut file) = File::create(output_path.clone()) {
            for fruit in &fruits {
                if let Err(err) = writeln!(file, "{}", fruit) {
                    eprintln!("Failed to write to file: {}", err);
                    return;
                }
            }
            println!("Output written to file: {}", output_path);
        } else {
            eprintln!("Failed to create file: {}", output_path);
        }
    }
}
