use std::fs::File;
use std::io::Write;
use xdg::BaseDirectories;
use std::collections::HashMap;

use crate::device::VideoDevice;

#[derive(Debug)]
pub struct SaveData {
    pub controls: HashMap<u32, u32>,
    // TODO: Profile name.
    // Capture stuff - resolution, format, etc.
}

pub fn get_save_filename(device: &VideoDevice) -> String {
    let device_name = device.name.replace(' ', "_");
    let usb_bus = device.capabilities.bus.clone();
    format!("{}-{}.cfg", device_name, usb_bus)
}

pub async fn save_device_state(filename: String, save_data: SaveData) -> std::io::Result<()> {
    let xdg_dirs = BaseDirectories::with_prefix("cosmic-cam").unwrap();
    let data_path = xdg_dirs.place_data_file(filename).unwrap();
    
    // TODO: Serialize the save data.
    let mut file = File::create(data_path)?;
    for (key, value) in save_data.controls.iter() {
        file.write_fmt(format_args!("{}={}\n", key, value))?;
    }
    Ok(())
}
