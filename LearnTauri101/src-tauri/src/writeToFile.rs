

use std::path::Path;
use std::fs::{File,OpenOptions};
use std::io::{Write,Read};


#[tauri::command]
pub fn write_to_file(content:&str)->Result<String,String>{
    let mut data_file:File;
    let fileExists = Path::new("inputFile.txt").try_exists().map_err(|e| e.to_string())?;
    if !fileExists{
        data_file = File::create("inputFile.txt").map_err(|e| e.to_string())?;
    }
    else {data_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("inputFile.txt").map_err(|e| e.to_string())?;}

    data_file.write(content.as_bytes()).map_err(|e| e.to_string())?;
    return Ok(String::from("Successfully wrote to file"));
}


#[tauri::command]
pub fn read_file() -> Result<String, String> {
    let mut data_file: File;
   
    let fileExists = Path::new("inputFile.txt").try_exists().map_err(|e| e.to_string())?;
   
    if !fileExists {
        data_file = File::create("inputFile.txt").map_err(|e| e.to_string())?;
    } else {
        data_file = File::open("inputFile.txt").map_err(|e| e.to_string())?;
    }
   

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).map_err(|e| e.to_string())?;

    return Ok(file_content)
}