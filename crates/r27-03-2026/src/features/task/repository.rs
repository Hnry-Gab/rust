use std::fs::File;
use std::io::{BufReader, Write};
use std::error::Error;

use super::model::Data;

pub fn load(path: &str) -> Result<Data, Box<dyn Error>> {
    let file: File = File::open(path)?;
    let reader = BufReader::new(file);

    let data: Data = serde_json::from_reader(reader)?;
    Ok(data)
}

pub fn save(path: &str, data: &Data) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    
    let json_string = serde_json::to_string_pretty(data)?;
    
    file.write_all(json_string.as_bytes())?;
    Ok(())
}