use std::error::Error;
use csv::StringRecord;

#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
}

pub fn load_csv(path: &str) -> Result<Table, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    let mut rows = Vec::new();
    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        println!("Parsed row: {:?}", row);
        rows.push(row);
    }
    
    Ok(Table { rows })
}