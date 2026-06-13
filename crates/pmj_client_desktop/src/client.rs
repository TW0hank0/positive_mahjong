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
    sync, thread,
};

use iced::{
    self, Border, Color, Element, Length, Pixels, alignment, task,
    widget::{
        self, Column, Grid, Row, button, container, grid, scrollable, space, stack, text,
        text_input,
    },
};

use tungstenite::{Message, WebSocket, connect};

use pmj_shared::shared::{self, FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES, FONT_NOTO_SANS_REG_BYTES};

use crate::{circular, easing};

pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");
pub const MATERIAL_SYMBOLS_OUTLINED: iced::font::Font =
    iced::font::Font::with_name("Material Symbols Outlined");

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
    theme: iced::theme::Theme,
    process_threads: Vec<thread::JoinHandle<ThreadResult>>,
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
    msgs: Vec<String>,
}

#[derive(Debug)]
pub struct PlayBaseStatus {
    server_ip: Option<IpAddr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UIMessage {
    Home(HomeMessage),
    FetchThreadsStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeMessage {
    InputServerIpChanged(String),
    VSoftKeyBoardInput(String),
    ConnectServer,
    SendFirstMsg,
    ReadFirstMsgResp,
}

pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadResult {
    pub is_error: bool,
    pub process_type: ThreadProcessTypes,
    pub result_read_first_msg_resp: Option<ThreadProcessResultReadFirstMsgResp>,
}

impl Default for ThreadResult {
    fn default() -> Self {
        Self {
            is_error: true,
            process_type: ThreadProcessTypes::DoNotThing,
            result_read_first_msg_resp: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadProcessResultReadFirstMsgResp {
    pub player_id: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreadProcessTypes {
    ReadFirstMsgResp,
    DoNotThing,
}

impl Client {
    pub fn new() -> Self {
        let _ = iced::font::load(FONT_NOTO_SANS_REG_BYTES);
        let _ = iced::font::load(FONT_MATERIAL_SYMBOLS_OUTLINED_BYTES);
        Self {
            current_scene: ClientScenes::Home,
            status_home: HomeStatus {
                server_ip: String::new(),
                try_connecting_server: false,
                msgs: Vec::new(),
            },
            status_play_base: PlayBaseStatus { server_ip: None },
            ws: None,
            player_id: None,
            theme: iced::theme::Theme::TokyoNight,
            process_threads: Vec::new(),
        }
    }
    pub fn update(&mut self, message: UIMessage) -> task::Task<UIMessage> {
        match message {
            UIMessage::FetchThreadsStatus => {
                let mut rp_index = 0;
                loop {
                    if rp_index > self.process_threads.len() {
                        break;
                    } else {
                        let rpthread = self.process_threads.get(rp_index).unwrap();
                        if rpthread.is_finished() {
                            let pthread = self.process_threads.remove(rp_index);
                            match pthread.join() {
                                Ok(thread_result) => {
                                    if thread_result.is_error {
                                        self.status_home
                                            .msgs
                                            .push(String::from("process_thread ran into error!"));
                                        match thread_result.process_type {
                                            ThreadProcessTypes::DoNotThing => {}
                                            ThreadProcessTypes::ReadFirstMsgResp => {
                                                return task::Task::done(UIMessage::Home(
                                                    HomeMessage::ReadFirstMsgResp,
                                                ));
                                            }
                                        }
                                        continue;
                                    } else {
                                        match thread_result.process_type {
                                            ThreadProcessTypes::ReadFirstMsgResp => {
                                                self.player_id = Some(
                                                    thread_result
                                                        .result_read_first_msg_resp
                                                        .unwrap()
                                                        .player_id,
                                                );
                                            }
                                            ThreadProcessTypes::DoNotThing => {}
                                        }
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("thread: {:?}", e);
                                    self.status_home.msgs.push(format!("thread: {:?}", e));
                                }
                            }
                        } else {
                            rp_index += 1;
                        }
                    }
                }
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
                    if self.status_home.server_ip.is_empty() {
                        self.status_home
                            .msgs
                            .push(String::from("未輸入伺服器地址！"));
                    } else if self.status_home.try_connecting_server {
                        self.status_home
                            .msgs
                            .push(String::from("已有正在嘗試連接的伺服器！"));
                    } else {
                        self.status_home.try_connecting_server = true;
                        let value = self.home_connect_server();
                        return value;
                    }
                }
                HomeMessage::SendFirstMsg => {
                    let value = self.home_send_first_msg();
                    return value;
                }
                HomeMessage::ReadFirstMsgResp => {
                    self.process_threads.push(self.home_read_first_msg_resp());
                    return task::Task::done(UIMessage::FetchThreadsStatus);
                }
            },
        };
        return task::Task::none();
    }

    pub fn view(&self) -> Element<'_, UIMessage, iced::Theme, iced::Renderer> {
        let mut layout: Column<'_, UIMessage, iced::Theme, iced::Renderer> = Column::new()
            .align_x(alignment::Horizontal::Left)
            .padding(10);
        //
        match self.current_scene {
            ClientScenes::Home => {
                let mut layout_home = Column::new();
                // 標題欄
                {
                    let mut title_bar = Row::new().align_y(alignment::Vertical::Center);
                    title_bar = title_bar.push(
                        text(format!("{}", shared::PROJECT_NAME))
                            .height(Length::Shrink)
                            .size(Pixels::from(26)),
                    );
                    title_bar = title_bar.spacing(25);
                    title_bar = title_bar.push(
                        text(format!("v{}", shared::PROJECT_VERSION))
                            .height(Length::Shrink)
                            .size(Pixels::from(22)),
                    );
                    layout_home = layout_home.push(title_bar);
                }
                layout_home = layout_home.push(space().height(5));
                // 伺服器地址輸入處理
                {
                    let mut server_ip_input_bar = Row::new();
                    server_ip_input_bar = server_ip_input_bar
                        .push(
                            text_input("輸入伺服器地址...", &self.status_home.server_ip)
                                .on_input(|content| {
                                    UIMessage::Home(HomeMessage::InputServerIpChanged(content))
                                })
                                .size(Pixels::from(24))
                                .style(|theme: &iced::theme::Theme, status: text_input::Status| {
                                    let style = text_input::default(theme, status);
                                    style
                                        .border
                                        .rounded(iced::border::Radius::new(Pixels::from(12)))
                                        .width(Pixels::from(10));
                                    style
                                })
                                .line_height(text::LineHeight::Relative(1.5)),
                        )
                        .spacing(15);
                    server_ip_input_bar = server_ip_input_bar.push(
                        button(text("連線").size(24))
                            .on_press(UIMessage::Home(HomeMessage::ConnectServer))
                            .style(|theme: &iced::Theme, status: button::Status| {
                                let ex_palette = theme.extended_palette();
                                let mut style = button::Style::default();
                                match status {
                                    button::Status::Active => {
                                        style =
                                            style.with_background(ex_palette.primary.base.color);
                                        style.text_color = ex_palette.primary.base.text;
                                    }
                                    button::Status::Disabled => {
                                        style =
                                            style.with_background(ex_palette.background.weak.color);
                                        style.text_color = ex_palette.background.weak.text;
                                    }
                                    button::Status::Hovered => {
                                        style =
                                            style.with_background(ex_palette.primary.weak.color);
                                        style.text_color = ex_palette.primary.weak.text;
                                    }
                                    button::Status::Pressed => {
                                        style =
                                            style.with_background(ex_palette.primary.strong.color);
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
                    layout_home = layout_home.push(server_ip_input_bar).spacing(35);
                }
                // 虛擬鍵盤
                {
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
                    layout_home = layout_home.push(vsoft_keyboard);
                }
                layout_home = layout_home.push(space().height(20));
                // 訊息顯示
                {
                    let mut msg_area = Column::new();
                    msg_area = msg_area.spacing(5);
                    let mut msg_number: u64 = 1;
                    for msg in self.status_home.msgs.iter() {
                        let ex_palette = self.theme.extended_palette();
                        let mut msg_row = Row::new();
                        msg_row = msg_row.spacing(3);
                        msg_row = msg_row.push(text(msg_number.to_string()).size(20).style(
                            move |_theme| text::Style {
                                color: Some(ex_palette.secondary.strong.text),
                            },
                        ));
                        msg_row = msg_row.spacing(3);
                        msg_row =
                            msg_row.push(text(msg).size(20).style(move |_theme| text::Style {
                                color: Some(ex_palette.secondary.base.text),
                            }));
                        msg_row = msg_row.spacing(3);
                        msg_area = msg_area.push(msg_row).spacing(1);
                        msg_number += 1;
                    }
                    layout_home = layout_home.push(container(msg_area).style(
                        |theme: &iced::theme::Theme| {
                            let ex_palette = theme.extended_palette();
                            let mut style = container::Style::default();
                            style.background =
                                Some(iced::Background::Color(ex_palette.secondary.base.color));
                            style.border = Border {
                                color: ex_palette.secondary.strong.color,
                                width: 2.5,
                                radius: iced::border::Radius::new(Pixels::from(8)),
                            };
                            style
                        },
                    ));
                }
                layout_home = layout_home.push(space().height(5));
                //
                if self.status_home.try_connecting_server {
                    let mut content_column = Column::new().padding(5).spacing(2);
                    content_column = content_column.push(
                        circular::Circular::new()
                            .easing(&easing::STANDARD)
                            .size(54.0),
                    );
                    content_column = content_column
                        .push(text("連線中...").size(28).style(
                            move |theme: &iced::theme::Theme| {
                                let ex_palette = theme.extended_palette();
                                text::Style {
                                    color: Some(ex_palette.secondary.base.text),
                                }
                            },
                        ))
                        .spacing(2);
                    let content = container(container(content_column).style(
                        move |theme: &iced::theme::Theme| {
                            let ex_palette = theme.extended_palette();
                            let mut style = container::Style::default();
                            style = style.background(iced::Background::Color(
                                ex_palette.secondary.weak.color,
                            ));
                            style.border(
                                Border::default()
                                    .color(ex_palette.secondary.strong.color)
                                    .rounded(12)
                                    .width(3),
                            );
                            style
                        },
                    ))
                    .center(Length::Fill)
                    .align_x(alignment::Alignment::Center)
                    .align_y(alignment::Alignment::Center)
                    .style(move |_theme| {
                        let mut style = container::Style::default();
                        style = style.background(iced::Background::Color(Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.4,
                        }));
                        style = style.border(Border::default().width(0));
                        style
                    });
                    layout_home =
                        widget::column([
                            stack([scrollable(layout_home).into(), content.into()]).into()
                        ]);
                    layout = layout.push(layout_home);
                } else {
                    layout = layout.push(scrollable(layout_home));
                }
            }
            ClientScenes::PlayBase => {
                let mut layout_play_base = Column::new();
                {
                    let mut info_bar = Row::new();
                    info_bar = info_bar.push(text(format!(
                        "伺服器地址：{}",
                        self.status_play_base.server_ip.unwrap().to_string(),
                    )));
                    layout_play_base = layout_play_base.push(info_bar)
                }
                layout = layout.push(layout_play_base);
                /* TODO: PlayScene */
            }
        }
        return layout.into();
    }

    fn home_create_vsoft_key<'a>(
        &self,
        key: String,
    ) -> button::Button<'a, UIMessage, iced::theme::Theme, iced::Renderer> {
        button(
            if key == String::from("backspace") || key == String::from("\u{e14a}") {
                text(format!("\u{e14a}")).font(MATERIAL_SYMBOLS_OUTLINED)
            } else {
                text(format!("{}", key))
            }
            .size(Pixels::from(28))
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(text::Alignment::Center)
            .align_y(alignment::Vertical::Center)
            .style(|theme: &iced::theme::Theme| {
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
        .style(|theme: &iced::theme::Theme, status: button::Status| {
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
        String::from("pmj_client_desktop")
    }

    pub fn theme(&self) -> iced::theme::Theme {
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
                return task::Task::done(UIMessage::Home(HomeMessage::SendFirstMsg));
            }
            Err(e) => {
                eprintln!("{}", e);
                self.status_home.msgs.push(e.to_string());
                self.status_home.try_connecting_server = false;
            }
        }
        return task::Task::none();
    }
    fn home_send_first_msg(&mut self) -> task::Task<UIMessage> {
        //TODO: log::info!("正在嘗試傳送初連接訊息");
        println!("正在嘗試傳送初連接訊息");
        let req_text = serde_json::to_string(&shared::ClientConnectRequestType {
            app_name: String::from("positive_mahjong"),
            client: String::from("pmj_client"),
        })
        .unwrap();
        match self.ws.clone() {
            Some(ws) => match ws.try_write() {
                Ok(mut guard) => match guard.send(Message::Text(req_text.into())) {
                    Ok(_) => {
                        println!("已傳送初連結訊息，等待伺服器回應...");
                    }
                    Err(e) => {
                        eprintln!("error: {}", e);
                        self.status_home.try_connecting_server = false;
                        return task::Task::none();
                    }
                },
                Err(e) => {
                    eprintln!("First msg: get guard error: {}", e);
                    return task::Task::done(UIMessage::Home(HomeMessage::SendFirstMsg));
                }
            },
            None => {
                return task::Task::done(UIMessage::Home(HomeMessage::ConnectServer));
            }
        }
        return task::Task::none();
    }

    fn home_read_first_msg_resp(&self) -> thread::JoinHandle<ThreadResult> {
        let ws = self.ws.clone().unwrap();
        thread::spawn(move || match ws.try_write() {
            Ok(mut guard) => {
                if !guard.can_read() {
                    eprintln!("guard.can_read() => false!");
                    ThreadResult {
                        is_error: true,
                        process_type: ThreadProcessTypes::ReadFirstMsgResp,
                        result_read_first_msg_resp: None,
                    }
                } else {
                    match guard.read() {
                        Ok(raw_msg) => {
                            match raw_msg {
                                Message::Text(text) => {
                                    let msg: shared::ServerFirstConnectType =
                                        serde_json::from_str(&text).unwrap();
                                    if msg.player_id.is_some() {
                                        println!("成功取得玩家識別碼：{}", msg.player_id.unwrap());
                                        ThreadResult {
                                            is_error: false,
                                            process_type: ThreadProcessTypes::ReadFirstMsgResp,
                                            result_read_first_msg_resp: Some(
                                                ThreadProcessResultReadFirstMsgResp {
                                                    player_id: msg.player_id.unwrap(),
                                                },
                                            ),
                                        }
                                    } else {
                                        eprintln!("error: msg.player_id is None");
                                        ThreadResult {
                                            is_error: true,
                                            process_type: ThreadProcessTypes::ReadFirstMsgResp,
                                            result_read_first_msg_resp: None,
                                        }
                                    }
                                }
                                _ => {
                                    ThreadResult {
                                        is_error: true,
                                        process_type: ThreadProcessTypes::ReadFirstMsgResp,
                                        result_read_first_msg_resp: None,
                                    }
                                    /* TODO:BIN-MsgPack */
                                }
                            }
                        }
                        Err(_e) => ThreadResult {
                            is_error: true,
                            process_type: ThreadProcessTypes::ReadFirstMsgResp,
                            result_read_first_msg_resp: None,
                        },
                    }
                }
            }
            Err(_e) => ThreadResult {
                is_error: true,
                process_type: ThreadProcessTypes::ReadFirstMsgResp,
                result_read_first_msg_resp: None,
            },
        })
        /*match self.ws.clone().unwrap().try_write() {
            Ok(mut guard) => {
                if !guard.can_read() {
                    eprintln!("error: guard.can_read() => flase");
                }
                match guard.read() {
                    Ok(raw_msg) => {
                        match raw_msg {
                            Message::Text(text) => {
                                let msg: shared::ServerFirstConnectType =
                                    serde_json::from_str(&text).unwrap();
                                if msg.player_id.is_some() {
                                    self.player_id = msg.player_id;
                                    println!("成功取得玩家識別碼：{}", self.player_id.unwrap());
                                } else {
                                    eprintln!("error: msg.player_id is None");
                                    //TODO
                                }
                            }
                            _ => { /* TODO:BIN-MsgPack */ }
                        }
                    }
                    Err(e) => {
                        eprintln!("error: {}", e);
                        self.status_home.try_connecting_server = false;
                        return task::Task::none();
                    }
                }
            }
            Err(e) => {
                eprintln!("(try-again) error: {}", e);
                return task::Task::done(UIMessage::Home(HomeMessage::ReadFirstMsgResp));
            }
        }*/
    }
}
