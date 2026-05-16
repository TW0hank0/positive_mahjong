use std::{
    self,
    net::{IpAddr, TcpStream},
    sync,
};

use iced_core::{Border, Padding, border::Radius};
use iced_wgpu::Renderer;
use iced_widget::{
    Column, Container, Grid, Row, button, container, grid, pick_list, scrollable, text, text_input,
};
use iced_winit::{
    core::{Color, Element, Length, Pixels, Theme, alignment},
    runtime,
};
//use iced_winit::runtime::Task;
//use iced_winit::winit::event_loop::EventLoopProxy;

use tungstenite::{Message, connect};

use tungstenite::WebSocket;

use pmj_shared::shared::{self, FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES, FONT_NOTO_SANS_REG_BYTES};

pub const FONT_NOTO_SANS_REG: iced_core::font::Font =
    iced_core::font::Font::with_name("Noto Sans TC");
pub const MATERIAL_SYMBOLS_OUTLINED: iced_core::font::Font =
    iced_core::font::Font::with_name("Material Symbols Outlined");

fn font_init() {
    let _ = runtime::font::load(FONT_NOTO_SANS_REG_BYTES);
    let _ = runtime::font::load(FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES);
}

#[derive(Debug)]
pub struct Client {
    current_scene: ClientScenes,
    status_home: HomeStatus,
    status_play_base: PlayBaseStatus,
    ws: Option<sync::Arc<sync::RwLock<WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>>>>,
    player_id: Option<u8>,
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
pub enum UIMessage {
    Home(HomeMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeMessage {
    InputServerIpChanged(String),
    VSoftKeyBoardInput(String),
    ConnectServer,
}

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Client {
    pub fn new() -> Self {
        font_init();
        Self {
            current_scene: ClientScenes::Home,
            status_home: HomeStatus {
                server_ip: String::new(),
            },
            status_play_base: PlayBaseStatus { server_ip: None },
            ws: None,
            player_id: None,
        }
    }
    pub fn update(&mut self, message: UIMessage) {
        match message {
            UIMessage::Home(home_message) => match home_message {
                HomeMessage::InputServerIpChanged(server_ip) => {
                    self.status_home.server_ip = server_ip;
                }
                HomeMessage::VSoftKeyBoardInput(key) => {
                    if key == String::from("backspace") || key == String::from("\u{e14a}") {
                        self.status_home.server_ip.pop();
                    } else {
                        self.status_home.server_ip =
                            format!("{}{}", self.status_home.server_ip, key);
                    }
                }
                HomeMessage::ConnectServer => {
                    self.home_connect_server();
                }
            },
        };
    }

    pub fn view(&self) -> Element<'_, UIMessage, Theme, Renderer> {
        let mut layout = Column::new()
            .align_x(alignment::Horizontal::Left)
            .padding(10);
        //
        match self.current_scene {
            ClientScenes::Home => {
                let mut title_bar = Row::new().align_y(alignment::Vertical::Center);
                title_bar = title_bar
                    .push(text(format!("{}", shared::PROJECT_NAME,)).size(Pixels::from(26)));
                title_bar = title_bar
                    .push(text(format!("v{}", shared::PROJECT_VERSION)).size(Pixels::from(22)));
                layout = layout.push(title_bar).spacing(16);
                let mut server_ip_input_bar = Row::new();
                server_ip_input_bar = server_ip_input_bar
                    .push(
                        text_input("輸入伺服器地址...", &self.status_home.server_ip)
                            .on_input(|content| {
                                UIMessage::Home(HomeMessage::InputServerIpChanged(content))
                            })
                            .size(Pixels::from(24))
                            .style(|theme: &Theme, status: text_input::Status| {
                                let style = text_input::default(theme, status);
                                style
                                    .border
                                    .rounded(Radius::new(Pixels::from(12)))
                                    .width(Pixels::from(7));
                                style
                            })
                            .line_height(text::LineHeight::Relative(1.5)),
                    )
                    .spacing(15);
                server_ip_input_bar = server_ip_input_bar
                    .push(button("connect").on_press(UIMessage::Home(HomeMessage::ConnectServer)));
                layout = layout.push(server_ip_input_bar).spacing(35);
                let mut vsoft_keyboard = Grid::new()
                    .height(grid::Sizing::EvenlyDistribute(Length::Shrink))
                    .columns(8);
                for key in 0..=9 {
                    vsoft_keyboard = vsoft_keyboard
                        .push(self.home_create_vsoft_key(format!("{}", key)))
                        .spacing(10);
                }
                for key in ALPHABET {
                    vsoft_keyboard = vsoft_keyboard
                        .push(self.home_create_vsoft_key(format!("{}", key)))
                        .spacing(10);
                }
                vsoft_keyboard = vsoft_keyboard
                    .push(self.home_create_vsoft_key(format!("{}", "backspace")))
                    .spacing(10);
                layout = layout.push(vsoft_keyboard).spacing(10);
            }
            ClientScenes::PlayBase => {}
        }
        //
        return scrollable(layout).into();
    }

    fn home_create_vsoft_key<'a>(
        &self,
        key: String,
    ) -> Container<'a, UIMessage, iced_core::Theme, Renderer> {
        container(
            button(
                if key == String::from("backspace") || key == String::from("\u{e14a}") {
                    text(format!("\u{e14a}")).font(MATERIAL_SYMBOLS_OUTLINED)
                } else {
                    text(format!("{}", key)).font(FONT_NOTO_SANS_REG)
                }
                .size(Pixels::from(28))
                .height(Length::Fill)
                .width(Length::Fill)
                .align_x(text::Alignment::Center)
                .align_y(alignment::Vertical::Center),
            )
            .height(Length::Shrink)
            .width(Length::Shrink)
            .on_press(UIMessage::Home(HomeMessage::VSoftKeyBoardInput(format!(
                "{}",
                key
            ))))
            .style(|theme: &Theme, status: button::Status| {
                let ex_palette = theme.extended_palette();
                let mut style = button::Style::default();
                match status {
                    button::Status::Active => {
                        style.with_background(ex_palette.primary.base.color);
                    }
                    button::Status::Disabled => {
                        style.with_background(ex_palette.primary.base.color);
                    }
                    button::Status::Hovered => {
                        style.with_background(ex_palette.primary.weak.color);
                    }
                    button::Status::Pressed => {
                        style.with_background(ex_palette.primary.strong.color);
                    }
                }
                style.border = Border::default().width(0);
                style
            })
            .padding(Padding::from(5)),
        )
        .padding(Padding::new(7.0))
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(|theme: &Theme| {
            let ex_palette = theme.extended_palette();
            container::Style::default()
                .border(
                    Border::default()
                        .rounded(Radius::new(Pixels::from(12)))
                        .color(ex_palette.primary.strong.color)
                        .width(Pixels::from(7)),
                )
                .background(ex_palette.primary.weak.color)
        })
    }

    pub fn title(&self) -> String {
        String::from("pmj_client")
    }

    fn home_connect_server(&mut self) {
        match connect(self.status_home.server_ip.clone()) {
            Ok((row_ws, _resp)) => {
                let ws: sync::Arc<
                    sync::RwLock<WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>>,
                > = sync::Arc::new(sync::RwLock::new(row_ws));
                let req_text = serde_json::to_string(&shared::ClientConnectRequestType {
                    app_name: String::from("positive_mahjong"),
                    client: String::from("pmj_client"),
                })
                .unwrap();
                let _ = ws.write().unwrap().send(Message::Text(req_text.into()));
                let raw_msg = ws.write().unwrap().read().unwrap();
                match raw_msg {
                    Message::Text(text) => {
                        let msg: shared::ServerFirstConnectType =
                            serde_json::from_str(&text).unwrap();
                        if msg.player_id.is_some() {
                            self.ws = Some(ws.clone());
                            self.player_id = msg.player_id;
                        } else {
                            eprintln!("error: msg.player_id is None");
                        }
                    }
                    _ => { /* 忽略 */ }
                }
            }
            Err(e) => {
                eprintln!("ws connect err: {}", e);
            }
        }
    }
}
