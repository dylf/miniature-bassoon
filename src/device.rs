use v4l::context;
use v4l::prelude::*;
use v4l::control::Type as ControlType;
use v4l::control::Value as ControlValue;

#[derive(Debug)]
pub struct VideoDevice {
    pub name: String,
    pub path: String,
    pub index: usize,
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
}

#[derive(Debug)]
pub struct ControlGroup {
    pub id: u32,
    pub name: String,
    pub controls: Vec<Control>,
}

#[derive(Debug)]
pub enum DeviceControls {
    ControlGroup(ControlGroup),
    Control(Control),
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
        .map(|dev| {
            let name = dev.name().expect("Failed to get device name");
            let path = dev.path().to_str().expect("Failed to get device path");
            VideoDevice {
                name: name.to_string(),
                path: path.to_string(),
                index: dev.index(),
            }
        })
        .collect::<Vec<VideoDevice>>();
    devices
}

pub fn get_capabilities(path: &str) -> v4l::capability::Capabilities {
    let dev = Device::with_path(path).unwrap();
    dev.query_caps().expect("Failed to query capabilities")
}

pub fn get_caps_string(dev: &Device) -> String {
    let caps = dev.query_caps().unwrap();
    format!("{:?}", caps)
}

pub fn get_device_by_path(path: &str) -> Result<Device, String> {
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
            ctrl_type => {
                let ctrl_val = dev.control(ctrl.id).map_err(|e| format!("{}", e))?.value;
                let current_ctrl = Control {
                    id: ctrl.id,
                    name: ctrl.name.clone(),
                    min: ctrl.minimum,
                    max: ctrl.maximum,
                    step: ctrl.step,
                    default: ctrl.default,
                    value: ctrl_val,
                    control_type: ctrl_type,
                    menu_items: Some(vec![(0, "".to_string())]),
                };

                match device_controls.last_mut() {
                    Some(DeviceControls::ControlGroup(ControlGroup { controls, .. })) => {
                        controls.push(current_ctrl)
                    }
                    _ => device_controls.push(DeviceControls::Control(current_ctrl)),
                }
            }
        }
    }
    Ok(device_controls)
}

pub fn set_control_val(dev: &Device, control_id: u32, value: ControlValue) -> Result<(), String> {
    let control = v4l::Control {
        id: control_id,
        value,
    };
    dev.set_control(control).map_err(|e| format!("{}", e))?;
    Ok(())
}
