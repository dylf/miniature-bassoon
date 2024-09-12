use std::fs::File;
use std::io::Write;
use xdg::BaseDirectories;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SaveData {
    pub device: String,
    pub data: HashMap<u32, u32>,
}

pub async fn save_value(data: SaveData) -> std::io::Result<()> {
    println!("Saving data: {:?}", data);
    let xdg_dirs = BaseDirectories::with_prefix("cosmic-cam").unwrap();
    let data_path = xdg_dirs.place_data_file("last").unwrap();

    let mut file = File::create(data_path)?;
    file.write_fmt(format_args!("device={:?}\n", data.device))?;
    for (key, value) in data.data.iter() {
        file.write_fmt(format_args!("{}={}\n", key, value))?;
    }
    Ok(())
}
