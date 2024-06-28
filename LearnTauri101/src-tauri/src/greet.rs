#[tauri::command]
pub fn greet(name: &str) -> String {
    println!("{}",name);
   return format!("Hello, {}!", name);
}