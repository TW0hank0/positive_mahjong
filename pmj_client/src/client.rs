use std::{self, net::IpAddr};

use iced_wgpu::Renderer;
use iced_widget::{Column, Row, pick_list, scrollable, text, text_input};
use iced_winit::core::{Color, Element, Length, Pixels, Theme, alignment};
//use iced_winit::runtime::Task;
//use iced_winit::winit::event_loop::EventLoopProxy;

use pmj_shared::shared;

#[derive(Debug)]
pub struct Client {
    current_scene: ClientScenes,
    status_home: HomeStatus,
    status_play_base: PlayBaseStatus,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClientScenes {
    Home,
    PlayBase,
}

#[derive(Debug)]
pub struct HomeStatus {
    server_ip: String,
}

#[derive(Debug)]
pub struct PlayBaseStatus {
    server_ip: Option<IpAddr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Home(HomeMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeMessage {
    InputServerIpChanged(String),
}

impl Client {
    pub fn new() -> Self {
        Self {
            current_scene: ClientScenes::Home,
            status_home: HomeStatus {
                server_ip: String::new(),
            },
            status_play_base: PlayBaseStatus { server_ip: None },
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Home(home_message) => match home_message {
                HomeMessage::InputServerIpChanged(server_ip) => {
                    self.status_home.server_ip = server_ip;
                }
            },
        };
    }

    pub fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        let mut layout = Column::new().align_x(alignment::Horizontal::Left);
        //
        match self.current_scene {
            ClientScenes::Home => {
                let mut title_bar = Row::new().align_y(alignment::Vertical::Center);
                title_bar = title_bar.push(
                    text(format!(
                        "{} v{}",
                        shared::PROJECT_NAME,
                        shared::PROJECT_VERSION
                    ))
                    .size(Pixels::from(28)),
                );
                layout = layout.push(title_bar).spacing(10);
                let mut server_ip_input_bar = Row::new();
                server_ip_input_bar = server_ip_input_bar.push(
                    text_input("輸入伺服器地址...", &self.status_home.server_ip)
                        .on_input(|content| {
                            Message::Home(HomeMessage::InputServerIpChanged(content))
                        })
                        .size(Pixels::from(22)),
                );
                layout = layout.push(server_ip_input_bar);
            }
            ClientScenes::PlayBase => {}
        }
        //
        return scrollable(layout).into();
    }

    pub fn title(&self) -> String {
        String::from("pmj_client")
    }
}
