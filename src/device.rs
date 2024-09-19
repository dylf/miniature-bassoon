use std::collections::HashMap;
use std::convert::AsRef;

use v4l::context;
use v4l::prelude::*;
use v4l::control::Type as ControlType;
use v4l::control::Value as ControlValue;

use crate::storage::SaveData;


#[derive(Debug)]
pub struct VideoDevice {
    pub name: String,
    pub path: String,
    pub index: usize,
    pub capabilities: v4l::capability::Capabilities,
    pub controls: Vec<DeviceControls>,
}

fn disabled_flags() -> v4l::control::Flags {
    v4l::control::Flags::READ_ONLY |
    v4l::control::Flags::DISABLED |
    v4l::control::Flags::INACTIVE
}


pub trait ControlData {
    fn is_disabled(&self) -> bool;
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
    pub flags: v4l::control::Flags,
}

impl ControlData for Control {
    fn is_disabled(&self) -> bool {
        self.flags.intersects(disabled_flags())
    }
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
    pub flags: v4l::control::Flags,
}

impl ControlData for IntegerControl {
    fn is_disabled(&self) -> bool {
        self.flags.intersects(disabled_flags())
    }
}

#[derive(Debug)]
pub struct BooleanControl {
    pub id: u32,
    pub name: String,
    pub default: bool,
    pub value: bool,
    pub flags: v4l::control::Flags,
}

impl ControlData for BooleanControl {
    fn is_disabled(&self) -> bool {
        self.flags.intersects(disabled_flags())
    }
}

#[derive(Debug)]
pub struct MenuControl {
    pub id: u32,
    pub name: String,
    pub default: usize,
    pub value: Option<usize>,
    pub menu_items: Vec<MenuItem>,
    pub flags: v4l::control::Flags,
}

impl ControlData for MenuControl {
    fn is_disabled(&self) -> bool {
        self.flags.intersects(disabled_flags())
    }
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: u32,
    label: String,
}

impl AsRef<str> for MenuItem {
    fn as_ref(&self) -> &str {
        &self.label
    }
}

#[derive(Debug)]
pub struct ButtonControl {
    pub id: u32,
    pub name: String,
    pub default: usize,
    pub value: Option<bool>,
    pub flags: v4l::control::Flags,
}

impl ControlData for ButtonControl {
    fn is_disabled(&self) -> bool {
        self.flags.intersects(disabled_flags())
    }
}

#[derive(Debug)]
pub struct ControlGroup {
    pub id: u32,
    pub name: String,
    pub controls: Vec<DeviceControls>,
    pub flags: v4l::control::Flags,
}

#[derive(Debug)]
pub enum DeviceControls {
    ControlGroup(ControlGroup),
    Integer(IntegerControl),
    Boolean(BooleanControl),
    Control(Control),
    Menu(MenuControl),
    Button(ButtonControl),
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
                    flags: ctrl.flags,
                }));
            }
            ControlType::Integer => {
                let ctrl_val = match dev.control(ctrl.id).map_err(|e| format!("{}", e))?.value {
                    ControlValue::Integer(val) => val,
                    _ => {
                        println!("Could not get control value for integer");
                        ctrl.default
                    },
                };
                let current_ctrl = DeviceControls::Integer(IntegerControl {
                    id: ctrl.id,
                    name: ctrl.name.clone(),
                    min: ctrl.minimum,
                    max: ctrl.maximum,
                    step: ctrl.step,
                    default: ctrl.default,
                    value: ctrl_val,
                    flags: ctrl.flags,
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
                    _ => {
                        println!("Could not get control value for boolean");
                        ctrl.default != 0
                    }
                };
                let current_ctrl = DeviceControls::Boolean(BooleanControl {
                    id: ctrl.id,
                    name: ctrl.name.clone(),
                    default: ctrl.default != 0,
                    value: ctrl_val,
                    flags: ctrl.flags,
                });
                match device_controls.last_mut() {
                    Some(DeviceControls::ControlGroup(ControlGroup { controls, .. })) => {
                        controls.push(current_ctrl)
                    }
                    _ => device_controls.push(current_ctrl),
                }
            },
            ControlType::Menu => {
                let ctrl_val = match dev.control(ctrl.id).map_err(|e| format!("{}", e))?.value {
                    ControlValue::Integer(val) => val,
                    _ => {
                        println!("Could not get control value for menu");
                        ctrl.default
                    },
                };
                let menu_items: Vec<MenuItem> = match &ctrl.items {
                    Some(items) => {
                    items.iter().map(|item| {
                        let label = match &item.1 {
                            v4l::control::MenuItem::Value(0) => "Off".to_string(),
                            v4l::control::MenuItem::Value(1) => "On".to_string(),
                            v4l::control::MenuItem::Name(name) => name.to_string(),
                            v4l::control::MenuItem::Value(val) => val.to_string(),
                        };
                        MenuItem {
                            id: item.0,
                            label,
                        }
                    }).collect::<Vec<MenuItem>>()
                    },
                    None => vec![],
                };
                let current_ctrl = DeviceControls::Menu(MenuControl {
                    id: ctrl.id,
                    name: ctrl.name.clone(),
                    default: ctrl.default as usize,
                    value: Some(ctrl_val as usize),
                    menu_items,
                    flags: ctrl.flags,
                });
                    
                match device_controls.last_mut() {
                    Some(DeviceControls::ControlGroup(ControlGroup { controls, .. })) => {
                        controls.push(current_ctrl)
                    }
                    _ => device_controls.push(current_ctrl),
                }
            },
            ControlType::Button => {
                let current_ctrl = DeviceControls::Button(ButtonControl {
                    id: ctrl.id,
                    name: ctrl.name.clone(),
                    default: ctrl.default as usize,
                    value: None,
                    flags: ctrl.flags,
                });
                    
                match device_controls.last_mut() {
                    Some(DeviceControls::ControlGroup(ControlGroup { controls, .. })) => {
                        controls.push(current_ctrl)
                    }
                    _ => device_controls.push(current_ctrl),
                }
            },
            ctrl_type => {
                println!("Unsupported control - type: {:?} , name: {:?}, id: {:?}", ctrl_type, ctrl.name, ctrl.id);
                // let ctrl_val = dev.control(ctrl.id).(|e| format!("{}", e))?.value;
                let ctrl_val = match dev.control(ctrl.id) {
                    Ok(ctrl) => {
                        ctrl.value
                    },
                    _ => {
                        println!("Could not get control value");
                        ControlValue::None
                    },
                };
                    
                    
                println!("Unsupported control val: {:?}", ctrl_val);
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
                    flags: ctrl.flags,
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

pub fn get_device_save_data(device: &VideoDevice) -> Result<SaveData, String> {
    let dev = get_v4l_device_by_path(&device.path).map_err(|e| format!("{}", e))?;

    let controls = dev.query_controls().map_err(|e| format!("{}", e))?;

    let mut control_values = HashMap::new();
    controls.iter().for_each(|ctrl| {
        if let Ok(ctrl) = dev.control(ctrl.id).map_err(|e| format!("{}", e)) {
                match ctrl.value {
                    ControlValue::Integer(val) => {
                        control_values.insert(ctrl.id, val as u32);
                    } 
                    ControlValue::Boolean(val) => {
                        control_values.insert(ctrl.id, val as u32);
                    }
                    _ => ()
                }
        };
    });
    Ok(
        SaveData {
            controls: control_values,
        }
    )
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
