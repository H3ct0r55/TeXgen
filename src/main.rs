mod table;
mod config;
mod latex;

use table::parser::load_csv;
use table::Table;
use config::load_config;
use latex::LaTeXTable;

use std::fs::write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the CSV data
    let table = load_csv("examples/test.csv")?;

    // Load the config
    let config = load_config("examples/config.json")?;

    // Wrap into your Table struct

    // Generate LaTeX table
    let latex_table = LaTeXTable::from_table(&table, &config);

    // Print to stdout (optional)
    println!("{}", latex_table.to_string());

    // Ensure output directory exists
    std::fs::create_dir_all("output")?;

    // Write to output file
    write("output/table.tex", latex_table.to_string())?;

    Ok(())
}