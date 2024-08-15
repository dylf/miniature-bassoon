use std::fs::File;
use std::io::Write;
use xdg::BaseDirectories;

pub async fn save_value(data: String) -> std::io::Result<()> {
    println!("Saving data: {}", data);
    let xdg_dirs = BaseDirectories::with_prefix("cosmic-cam").unwrap();
    let data_path = xdg_dirs.place_data_file("last").unwrap();

    let mut file = File::create(data_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
