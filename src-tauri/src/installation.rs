use std::error::Error;

pub mod process;
pub mod setup;

pub fn get_details(arg: Vec<String>) -> Result<String, Box<dyn Error>> {
    println!("{:?}", arg); //results are like this: ["D:\\_u\\do\\ventoy-1.0.99-windows.zip", "ventoy-1.0.99/VentoyVlnk.exe"]

    Ok(serde_json::to_string("")?)
}
