use cosmic::widget;
use v4l::context;
use v4l::prelude::*;
use v4l::control::Type as ControlType;
use v4l::control::Value as ControlValue;
use cosmic::{theme, Element};

#[derive(Debug)]
pub struct VideoDevice {
    pub name: String,
    pub path: String,
    index: usize,
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

pub trait RenderControl {
    fn render_ctrl<'a, T: 'a>(&'a self) -> Element<'a, T>;
}

impl RenderControl for Control  {
    fn render_ctrl<'a, T: 'a>(&'a self) -> Element<'a, T> {
        let spacing = theme::active().cosmic().spacing;
        // let min = self.min as f32;
        // let max = self.max as f32;
        // let default = self.default as f32;
        // let step = self.step as f32;
        // let element: Element<Message> = match ctrl.control_type {
        //     v4l::control::Type::Menu => {
        //         let menu_items = ctrl.menu_items.as_ref().unwrap();
        //         let selected = menu_items.iter().position(|(i, _)| *i == (ctrl.default as u32)).unwrap_or(0);
        //         let options = menu_items.iter().map(|(_, v)| v.clone()).collect::<Vec<String>>();
        //         widget::dropdown::dropdown(
        //             options,
        //             Some(selected),
        //             |val| Message::Slider(val as f32)
        //         ).into()
        //     }
        //     _ => widget::slider::Slider::new(min..=max, default, |val| Message::Slider(val)).step(step).into()
        // };
        let text = widget::text::text(self.name.clone());
        widget::column().spacing(spacing.space_xxs).push(text).into()
    }
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

pub fn get_caps_string(path: &str) -> String {
    let dev = Device::with_path(path).unwrap();
    let caps = dev.query_caps().unwrap();
    format!("{:?}", caps)
}

pub fn get_device_controls(path: &str) -> Result<Vec<DeviceControls>, String> {
    let dev = Device::with_path(path).map_err(|e| format!("{}", e))?;
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


    
