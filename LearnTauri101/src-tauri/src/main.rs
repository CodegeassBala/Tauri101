// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod writeToFile;
use writeToFile::{write_to_file,read_file};






fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_file,write_to_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  // match write_to_file("something"){
  //   Ok(val)=>(
  //     println!("{}",val)
  //   ),
  //   Err(err)=>(
  //     println!("{}",err)
  //   )
  // }
}
