// SPDX-License-Identifier: AGPL-3.0-only
// 著作權所有 (C) 2026 TW0hank0
//
// 本檔案屬於 positive_mahjong 專案的一部分。
// 專案儲存庫：https://github.com/TW0hank0/positive_mahjong
//
// 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
// 第 3 版（僅此版本）重新發佈及/或修改本程式。
//
// 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
// 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
//
// 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
// 如果沒有，請參見 <https://www.gnu.org/licenses/>。

use std::{
    self,
    net::{IpAddr, TcpStream},
    sync,
};

use iced_core::{Border, Padding, border::Radius, theme};
use iced_wgpu::Renderer;
use iced_widget::{
    self, Column, Container, Grid, Row, button, container, grid, iced, pick_list, scrollable,
    stack, text, text_input,
};
use iced_winit::{
    core::{Color, Element, Length, Pixels, Theme, alignment},
    runtime::{self, task},
};
//use iced_winit::winit::event_loop::EventLoopProxy;

use tungstenite::WebSocket;
use tungstenite::{Message, connect};

use pmj_shared::shared::{self, FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES, FONT_NOTO_SANS_REG_BYTES};

use pmj_shared::{circular, easing};

pub const FONT_NOTO_SANS_REG: iced_core::font::Font =
    iced_core::font::Font::with_name("Noto Sans TC");
pub const MATERIAL_SYMBOLS_OUTLINED: iced_core::font::Font =
    iced_core::font::Font::with_name("Material Symbols Outlined");

/* fn font_init() {
    let _ = runtime::font::load(FONT_NOTO_SANS_REG_BYTES);
    let _ = runtime::font::load(FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES);
} */

#[derive(Debug)]
pub struct Client {
    current_scene: ClientScenes,
    status_home: HomeStatus,
    status_play_base: PlayBaseStatus,
    ws: Option<sync::Arc<sync::RwLock<WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>>>>,
    player_id: Option<u8>,
    theme: theme::Theme,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClientScenes {
    Home,
    PlayBase,
}

#[derive(Debug)]
pub struct HomeStatus {
    server_ip: String,
    try_connecting_server: bool,
}

#[derive(Debug)]
pub struct PlayBaseStatus {
    server_ip: Option<IpAddr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UIMessage {
    NotThing,
    LoadFont,
    Home(HomeMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeMessage {
    InputServerIpChanged(String),
    VSoftKeyBoardInput(String),
    ConnectServer,
    FirstMsg,
}

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Client {
    pub fn new() -> Self {
        let _ = runtime::font::load(FONT_NOTO_SANS_REG_BYTES);
        let _ = runtime::font::load(FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES);
        Self {
            current_scene: ClientScenes::Home,
            status_home: HomeStatus {
                server_ip: String::new(),
                try_connecting_server: false,
            },
            status_play_base: PlayBaseStatus { server_ip: None },
            ws: None,
            player_id: None,
            theme: Theme::TokyoNight,
        }
    }
    pub fn update(&mut self, message: UIMessage) -> task::Task<UIMessage> {
        match message {
            UIMessage::NotThing => {}
            UIMessage::LoadFont => {
                return runtime::font::load(FONT_NOTO_SANS_REG_BYTES)
                    .chain(runtime::font::load(FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES))
                    .map(|result| match result {
                        Ok(_) => {
                            return UIMessage::NotThing;
                        }
                        Err(e) => {
                            eprintln!("Failed to load font: {:?}", e);
                            return UIMessage::LoadFont;
                        }
                    });
            }
            UIMessage::Home(home_message) => match home_message {
                HomeMessage::InputServerIpChanged(server_ip) => {
                    self.status_home.server_ip = server_ip;
                }
                HomeMessage::VSoftKeyBoardInput(key) => {
                    if key == String::from("backspace") || key == String::from("\u{e14a}") {
                        self.status_home.server_ip.pop();
                    } else {
                        self.status_home.server_ip.push_str(&key);
                    }
                }
                HomeMessage::ConnectServer => {
                    return self.home_connect_server();
                }
                HomeMessage::FirstMsg => {
                    return self.home_first_msg();
                }
            },
        };
        return task::Task::none();
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
                    .push(text(format!("{}", shared::PROJECT_NAME)).size(Pixels::from(26)));
                title_bar = title_bar.spacing(25);
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
                                    .width(Pixels::from(10));
                                style
                            })
                            .line_height(text::LineHeight::Relative(1.5)),
                    )
                    .spacing(15);
                server_ip_input_bar = server_ip_input_bar.push(
                    button(text("連線").size(24))
                        .on_press(UIMessage::Home(HomeMessage::ConnectServer))
                        .style(|theme: &Theme, status: button::Status| {
                            let ex_palette = theme.extended_palette();
                            let mut style = button::Style::default();
                            match status {
                                button::Status::Active => {
                                    style = style.with_background(ex_palette.primary.base.color);
                                    style.text_color = ex_palette.primary.base.text;
                                }
                                button::Status::Disabled => {
                                    style = style.with_background(ex_palette.background.weak.color);
                                    style.text_color = ex_palette.background.weak.text;
                                }
                                button::Status::Hovered => {
                                    style = style.with_background(ex_palette.primary.weak.color);
                                    style.text_color = ex_palette.primary.weak.text;
                                }
                                button::Status::Pressed => {
                                    style = style.with_background(ex_palette.primary.strong.color);
                                    style.text_color = ex_palette.primary.strong.text;
                                }
                            }
                            style.border = Border::default()
                                .width(5)
                                .color(ex_palette.primary.strong.color)
                                .rounded(10);
                            style
                        }),
                );
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
                        .push(self.home_create_vsoft_key(format!("{}", key).to_lowercase()))
                        .spacing(10);
                }
                for key in [":", "[", "]", ".", "/", "backspace"] {
                    vsoft_keyboard = vsoft_keyboard
                        .push(self.home_create_vsoft_key(format!("{}", key)))
                        .spacing(10);
                }
                layout = layout.push(vsoft_keyboard).spacing(10);
                //
                /*let connecting_tip = container(
                    container(text("載入中").style(|theme: &Theme| {
                        let ex_palette = theme.extended_palette();
                        text::Style {
                            color: Some(ex_palette.primary.base.text),
                        }
                    }))
                    .style(|theme: &Theme| {
                        let ex_palette = theme.extended_palette();
                        let mut style = container::Style::default();
                        style.background =
                            Some(iced_core::Background::Color(ex_palette.primary.weak.color));
                        style.border = Border::default()
                            .color(ex_palette.primary.strong.color)
                            .rounded(12)
                            .width(4);
                        style
                    }),
                )
                .style(|_theme: &theme::Theme| {
                    container::Style::default()
                        .background(iced_core::Background::Color(Color::TRANSPARENT))
                })
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill);*/
                if self.status_home.try_connecting_server {
                    let mut connecting_tip = Column::new().align_x(alignment::Horizontal::Center);
                    let mut tip_row = Row::new().align_y(alignment::Vertical::Center);
                    tip_row = tip_row.push(
                        circular::Circular::new()
                            .easing(&easing::STANDARD)
                            .cycle_duration(std::time::Duration::from_millis(850)),
                    );
                    tip_row = tip_row.push(text("連線中...").size(18));
                    connecting_tip = connecting_tip.push(tip_row);

                    layout = iced_widget::column!(stack([layout.into(), connecting_tip.into()]));
                }
            }
            ClientScenes::PlayBase => { /* TODO: PlayScene */ }
        }
        //
        return scrollable(layout).into();
    }

    fn home_create_vsoft_key<'a>(
        &self,
        key: String,
    ) -> button::Button<'a, UIMessage, iced_core::Theme, Renderer> {
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
            .align_y(alignment::Vertical::Center)
            .style(|theme: &theme::Theme| {
                let ex_palette = theme.extended_palette();
                let mut style = text::Style::default();
                style.color = Some(ex_palette.primary.base.text);
                style
            }),
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
                    style = style.with_background(ex_palette.primary.base.color);
                    style.text_color = ex_palette.primary.base.text;
                }
                button::Status::Disabled => {
                    style = style.with_background(ex_palette.background.weak.color);
                    style.text_color = ex_palette.primary.base.text;
                    //style.text_color = ex_palette.background.weak.text;
                }
                button::Status::Hovered => {
                    style = style.with_background(ex_palette.primary.weak.color);
                    style.text_color = ex_palette.primary.base.text;
                    //style.text_color = ex_palette.primary.weak.text;
                }
                button::Status::Pressed => {
                    style = style.with_background(ex_palette.primary.strong.color);
                    style.text_color = ex_palette.primary.base.text;
                    //style.text_color = ex_palette.primary.strong.text;
                }
            }
            style.border = Border::default()
                .width(5)
                .color(ex_palette.primary.strong.color)
                .rounded(10);
            style
        })
    }

    pub fn title(&self) -> String {
        String::from("pmj_client")
    }

    pub fn theme(&self) -> theme::Theme {
        self.theme.clone()
    }

    fn home_connect_server(&mut self) -> task::Task<UIMessage> {
        match connect(self.status_home.server_ip.clone()) {
            Ok((row_ws, _resp)) => {
                let ws: sync::Arc<
                    sync::RwLock<WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>>,
                > = sync::Arc::new(sync::RwLock::new(row_ws));
                self.ws = Some(ws.clone());
                println!("Websocket 連線成功。");
            }
            Err(e) => {
                eprintln!("ws connect err: {}", e);
            }
        }
        return task::Task::none();
    }
    fn home_first_msg(&mut self) -> task::Task<UIMessage> {
        //TODO: log::info!("正在嘗試傳送初連接訊息");
        println!("正在嘗試傳送初連接訊息");
        let req_text = serde_json::to_string(&shared::ClientConnectRequestType {
            app_name: String::from("positive_mahjong"),
            client: String::from("pmj_client"),
        })
        .unwrap();
        match self.ws.clone() {
            Some(ws) => match ws.try_write() {
                Ok(mut guard) => {
                    match guard.send(Message::Text(req_text.into())) {
                        Ok(_) => {
                            match guard.read() {
                                Ok(raw_msg) => {
                                    match raw_msg {
                                        Message::Text(text) => {
                                            let msg: shared::ServerFirstConnectType =
                                                serde_json::from_str(&text).unwrap();
                                            if msg.player_id.is_some() {
                                                self.player_id = msg.player_id;
                                                println!(
                                                    "成功取得玩家識別碼：{}",
                                                    self.player_id.unwrap()
                                                );
                                            } else {
                                                eprintln!("error: msg.player_id is None");
                                                //TODO
                                            }
                                        }
                                        _ => { /* TODO:BIN-MsgPack */ }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("error:{}", e);
                                    return task::Task::done(UIMessage::Home(
                                        HomeMessage::FirstMsg,
                                    ));
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("error:{}", e);
                            return task::Task::done(UIMessage::Home(HomeMessage::FirstMsg));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("First msg: get guard error: {}", e);
                    return task::Task::done(UIMessage::Home(HomeMessage::FirstMsg));
                }
            },
            None => {
                return task::Task::done(UIMessage::Home(HomeMessage::ConnectServer));
            }
        }
        return task::Task::none();
    }
}
