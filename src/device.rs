use v4l::context;

#[derive(Debug)]
pub struct VideoDevice {
    pub name: String,
    path: String,
    index: usize,
}


pub fn get_devices() -> Vec<VideoDevice> {
    let devices = context::enum_devices()
        .iter()
        .map(|dev| {
            let name = dev.name().unwrap();
            let path = dev.path().to_str().unwrap();
            VideoDevice {
                name: name.to_string(),
                path: path.to_string(),
                index: dev.index(),
            }
        })
        .collect::<Vec<VideoDevice>>();
    devices
}

