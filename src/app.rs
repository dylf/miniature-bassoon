// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::content::{self, Content};
use crate::device::*;
use crate::settings;
use crate::storage::{get_save_filename, save_device_state};
use crate::fl;
use cosmic::app::{context_drawer, message, Task, Core};
use cosmic::iced::Alignment;
use cosmic::widget::{self, icon, menu, nav_bar};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Element};

const REPOSITORY: &str = "https://github.com/dylf/miniature-bassoon";

pub struct App {
    core: Core,
    context_page: ContextPage,
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    nav: nav_bar::Model,
    content: Content,
    selected_device: Option<VideoDevice>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Content(content::Message),
    LaunchUrl(String),
    ToggleContextPage(ContextPage),
    Setting(settings::Message),
}

pub enum Page {
    VideoDeviceForm(String),
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
    Settings
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("menu-about"),
            Self::Settings => fl!("menu-settings"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
    ToggleSettings,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
            MenuAction::ToggleSettings => Message::ToggleContextPage(ContextPage::Settings),
        }
    }
}

impl Application for App {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "dev.dylf.MiniatureBassoon";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut nav = nav_bar::Model::default();

        get_devices().iter().for_each(|device| {
            let name = device.name.clone();
            nav.insert()
                .text(name)
                .data::<Page>(Page::VideoDeviceForm(device.path.clone()))
                .icon(icon::from_name("camera-web-symbolic"))
                .activate();
        });


        let mut app = App {
            core,
            context_page: ContextPage::default(),
            key_binds: HashMap::new(),
            nav,
            content: Content::new(),
            selected_device: None,
        };
        app.set_device_from_nav();

        let command = app.update_titles();

        (app, command)
    }

    fn on_app_exit(&mut self) -> Option<Self::Message> {
        println!("Exiting...");
        None
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![
                    menu::Item::Button(fl!("menu-settings"), None, MenuAction::ToggleSettings),
                    menu::Item::Divider,
                    menu::Item::Button(fl!("menu-about"), None, MenuAction::About),
                ]
            )
        )])
        .item_width(menu::ItemWidth::Uniform(320))
        .item_height(menu::ItemHeight::Dynamic(40))
        .spacing(4.0);

        vec![menu_bar.into()]
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.selected_device {
            Some(dev) => {
                self.content.view(dev).map(Message::Content)
            }
            _ => {
                cosmic::widget::button::standard("Main")
                    .on_press(Message::Content(content::Message::Save)).into()
            }
        }
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::LaunchUrl(url) => {
                let _result = open::that_detached(url);
            }

            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }
            }
            Message::Content(message) => {
                let dev = &self.selected_device;
                if let Some(dev) = dev {
                    let content_command = self.content.update(dev, message);
                    self.set_device_from_nav();
                    let dev = self.selected_device.as_ref().unwrap();
                    if let Some(content::Task::Save) = content_command {
                        let save_data = get_device_save_data(dev);
                        if let Ok(save_data) = save_data {
                            let filename = get_save_filename(dev);
                            return Task::perform
                                (save_device_state(filename, save_data),
                                    |_| message::none() );
                        } else {
                            return Task::none();
                        }
                    };
                }
            }
            Message::Setting(message) => {
                let setting_command = self.update_settings(message);
                if let Some(settings::Task::Save) = setting_command {
                    return Task::none();
                }
            }
        }
        Task::none()
    }

    fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => context_drawer::context_drawer(
                self.about(),
                Message::ToggleContextPage(ContextPage::About),
            ).title(fl!("menu-about")),
            ContextPage::Settings => context_drawer::context_drawer(
                self.settings(),
                Message::ToggleContextPage(ContextPage::Settings),
            ).title(fl!("menu-settings")),
        })
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        self.nav.activate(id);
        self.set_device_from_nav();
        self.update_titles()
    }
}

impl App {

    fn set_device_from_nav(&mut self) {
        match self.nav.data(self.nav.active()) {
            Some(Page::VideoDeviceForm(dev_path)) => {
                let dev = get_device_by_path(dev_path).unwrap();
                self.selected_device = Some(dev);
            }
            _ => {
                println!("Something terrible has occured!");
            }
        }
    }
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../res/icons/hicolor/128x128/apps/com.example.CosmicAppTemplate.svg")
                [..],
        ));

        let title = widget::text::title3(fl!("app-title"));

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::LaunchUrl(REPOSITORY.to_string()))
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    pub fn update_titles(&mut self) -> Task<Message> {
        let mut window_title = fl!("app-title");
        let mut header_title = String::new();

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
            header_title.push_str(page);
        }

        self.set_header_title(header_title);
        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
}
