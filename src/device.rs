use v4l::context;
use v4l::prelude::*;
use v4l::control::Type as ControlType;
use v4l::control::Value as ControlValue;


#[derive(Debug)]
pub struct VideoDevice {
    pub name: String,
    pub path: String,
    pub index: usize,
    pub capabilities: v4l::capability::Capabilities,
    pub controls: Vec<DeviceControls>,
}

#[derive(Debug)]
pub struct Control {
    pub id: u32,
    pub name: String,
    pub min: i64,
    pub max: i64,
    pub step: u64,
    pub default: i64,
    pub value: ControlValue,
    pub control_type: ControlType,
    pub menu_items: Option<Vec<(u32, String)>>,
    // pub flags: Flags,
}

#[derive(Debug)]
pub struct IntegerControl {
    pub id: u32,
    pub name: String,
    pub min: i64,
    pub max: i64,
    pub step: u64,
    pub default: i64,
    pub value: i64,
}

#[derive(Debug)]
pub struct BooleanControl {
    pub id: u32,
    pub name: String,
    pub default: bool,
    pub value: bool,
}

#[derive(Debug)]
pub struct MenuControl {
    pub id: u32,
    pub name: String,
    pub default: usize,
    pub value: Option<usize>,
    // Do we need u32 here? in v4l type
    pub menu_items: Vec<String>,
}


#[derive(Debug)]
pub struct ControlGroup {
    pub id: u32,
    pub name: String,
    pub controls: Vec<DeviceControls>,
}

#[derive(Debug)]
pub enum DeviceControls {
    ControlGroup(ControlGroup),
    Integer(IntegerControl),
    Boolean(BooleanControl),
    Control(Control),
    Menu(MenuControl),
}

pub fn get_devices() -> Vec<VideoDevice> {
    let devices = context::enum_devices()
        .iter()
        .filter(|dev| {
            let path = dev.path().to_str().expect("Failed to get device path");
            !get_capabilities(path)
                .capabilities
                .contains(v4l::capability::Flags::META_CAPTURE)
        })
        .filter_map(|dev| {
        let name = dev.name().unwrap_or(String::from("Unknown"));
        let path = dev.path().to_str();
        let device = path.as_ref().and_then(|p| get_v4l_device_by_path(p).ok());
        let device_controls = device.as_ref().and_then(|d| get_device_controls(d).ok());

        if let (name, Some(path), Some(device_controls)) = (name, path, device_controls) {
            Some(VideoDevice {
                name: name.to_string(),
                path: path.to_string(),
                capabilities: get_capabilities(path),
                index: dev.index(),
                controls: device_controls,
            })
        } else {
            None
        }
        })
        .collect::<Vec<VideoDevice>>();
    devices
}

pub fn get_device_by_path(path: &str) -> Result<VideoDevice, String> {
    let dev = get_devices().into_iter().find(|dev| dev.path == path);
    match dev {
        Some(device) => Ok(device),
        None => Err(format!("Device not found: {}", path)),
    }
}

pub fn get_capabilities(path: &str) -> v4l::capability::Capabilities {
    let dev = Device::with_path(path).unwrap();
    dev.query_caps().expect("Failed to query capabilities")
}

pub fn get_caps_string(dev: &Device) -> String {
    let caps = dev.query_caps().unwrap();
    format!("{:?}", caps)
}

pub fn get_v4l_device_by_path(path: &str) -> Result<Device, String> {
    Device::with_path(path).map_err(|e| format!("{}", e))
}

pub fn get_device_controls(dev: &Device) -> Result<Vec<DeviceControls>, String> {
    let controls = dev.query_controls().map_err(|e| format!("{}", e))?;
    let mut device_controls: Vec<DeviceControls> = Vec::new();

    for ctrl in controls {
        match ctrl.typ {
            ControlType::CtrlClass => {
                device_controls.push(DeviceControls::ControlGroup(ControlGroup {
                    id: ctrl.id,
                    name: ctrl.name,
                    controls: Vec::new(),
                }));
            }
            ControlType::Integer => {
                let ctrl_val = match dev.control(ctrl.id).map_err(|e| format!("{}", e))?.value {
                    ControlValue::Integer(val) => val,
                    _ => 0,
                };
                let current_ctrl = DeviceControls::Integer(IntegerControl {
                    id: ctrl.id,
                    name: ctrl.name.clone(),
                    min: ctrl.minimum,
                    max: ctrl.maximum,
                    step: ctrl.step,
                    default: ctrl.default,
                    value: ctrl_val,
                });
                match device_controls.last_mut() {
                    Some(DeviceControls::ControlGroup(ControlGroup { controls, .. })) => {
                        controls.push(current_ctrl)
                    }
                    _ => device_controls.push(current_ctrl),
                }
            }
            ControlType::Boolean => {
                let ctrl_val = match dev.control(ctrl.id).map_err(|e| format!("{}", e))?.value {
                    ControlValue::Boolean(val) => val,
                    _ => false,
                };
                let current_ctrl = DeviceControls::Boolean(BooleanControl {
                    id: ctrl.id,
                    name: ctrl.name.clone(),
                    default: ctrl.default != 0,
                    value: ctrl_val,
                });
                match device_controls.last_mut() {
                    Some(DeviceControls::ControlGroup(ControlGroup { controls, .. })) => {
                        controls.push(current_ctrl)
                    }
                    _ => device_controls.push(current_ctrl),
                }
            },
            ControlType::Menu => {
                // if let Some(menu_items) = ctrl.items {
                //
                //
                //
                // }
                let current_ctrl = DeviceControls::Menu(MenuControl {
                    id: ctrl.id,
                    name: ctrl.name.clone(),
                    default: ctrl.default as usize,
                    value: Some(0),
                    menu_items: vec![("".to_string())],
                });
                    
                match device_controls.last_mut() {
                    Some(DeviceControls::ControlGroup(ControlGroup { controls, .. })) => {
                        controls.push(current_ctrl)
                    }
                    _ => device_controls.push(current_ctrl),
                }
            },
            ctrl_type => {
                let ctrl_val = dev.control(ctrl.id).map_err(|e| format!("{}", e))?.value;
                let current_ctrl = DeviceControls::Control(Control{
                    id: ctrl.id,
                    name: ctrl.name.clone(),
                    min: ctrl.minimum,
                    max: ctrl.maximum,
                    step: ctrl.step,
                    default: ctrl.default,
                    value: ctrl_val,
                    control_type: ctrl_type,
                    menu_items: Some(vec![(0, "".to_string())]),
                });

                match device_controls.last_mut() {
                    Some(DeviceControls::ControlGroup(ControlGroup { controls, .. })) => {
                        controls.push(current_ctrl)
                    }
                    _ => device_controls.push(current_ctrl),
                }
            }
        }
    }
    Ok(device_controls)
}


pub fn set_control_val(dev: &VideoDevice, control_id: u32, value: ControlValue) -> Result<(), String> {
    let dev = get_v4l_device_by_path(&dev.path).map_err(|e| format!("{}", e))?;
    let control = v4l::Control {
        id: control_id,
        value,
    };
    dev.set_control(control).map_err(|e| format!("{}", e))?;
    Ok(())
}
