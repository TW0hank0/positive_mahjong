// SPDX-License-Identifier: AGPL-3.0-only
// 版權所有 (C) 2026 TW0hank0
//
// 本檔案屬於 positive_mahjong 專案的一部分。
// 專案儲存庫：https://gitlab.com/TW0hank0/positive_mahjong
//
// 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
// 第 3 版（僅此版本）重新發佈及/或修改本程式。
//
// 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
// 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
//
// 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
// 如果沒有，請參見 <https://www.gnu.org/licenses/>。

use std::time;
use tracing::info;
use iced::{
    self, Background, Border, Length, alignment, border, widget::{Column, Row, button, container, scrollable,  text},
};

use pmj_shared::shared::{FONT_NOTO_SANS_REG_BYTES, PROJECT_NAME, ICON_PNG_BYTES};

pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");

#[derive(Debug)]
pub enum ContainerStyles {
    PrimaryOutlined,
    BackgroundOutlined,
}

impl ContainerStyles {
    pub fn style(&self) -> impl Fn(&iced::Theme) -> container::Style {
        match self {
            Self::PrimaryOutlined => |theme: &iced::Theme| {
                let p = theme.extended_palette();
                container::Style {
                    border: Border {
                        color: p.primary.base.color,
                        width: 0.7,
                        radius: border::radius(8),
                    },
                    ..Default::default()
                }
            },
            Self::BackgroundOutlined => |theme: &iced::Theme| {
                let p = theme.extended_palette();
                container::Style {
                    border: Border {
                        color: p.background.strong.color,
                        width: 0.7,
                        radius: border::radius(8),
                    },
                    ..Default::default()
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum ButtonStyles {
    PrimaryRounded,
    PrimaryOutlined,
}

impl ButtonStyles {
    pub fn style(&self) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
        match self {
            Self::PrimaryRounded => |theme: &iced::Theme, status: button::Status| {
                let p = theme.extended_palette();
                button::Style {
                    background: Some(iced::Background::Color(p.primary.base.color)),
                    text_color: p.primary.base.text,
                    border: Border {
                        width: 2.0,
                        radius: match status {
                            button::Status::Active | button::Status::Disabled => border::radius(8),
                            button::Status::Hovered => border::radius(12),
                            button::Status::Pressed => border::radius(16),
                        },
                        color: match status {
                            button::Status::Active | button::Status::Disabled => {
                                iced::Color::TRANSPARENT
                            }
                            button::Status::Hovered | button::Status::Pressed => {
                                p.primary.strong.color
                            }
                        },
                    },
                    shadow: iced::Shadow {
                        color: iced::Color::TRANSPARENT,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            },
            Self::PrimaryOutlined => |theme: &iced::Theme, status: button::Status| {
                let p = theme.extended_palette();
                button::Style {
                    background: match status {
                        button::Status::Disabled => {
                            Some(iced::Background::Color(p.background.weak.color))
                        }
                        _ => None,
                    },
                    text_color: p.background.base.text,
                    border: Border {
                        width: match status {
                            button::Status::Active | button::Status::Disabled => 1.2,
                            button::Status::Hovered => 1.5,
                            button::Status::Pressed => 0.7,
                        },
                        color: match status {
                            button::Status::Active | button::Status::Disabled => {
                                p.background.strong.color
                            }
                            button::Status::Hovered => p.primary.weak.color,
                            button::Status::Pressed => p.primary.strong.color,
                        },
                        radius: border::radius(match status {
                            button::Status::Active
                            | button::Status::Disabled
                            | button::Status::Hovered => 10,
                            button::Status::Pressed => 6,
                        }),
                    },
                    ..Default::default()
                }
            },
        }
    }
}

pub fn iced_window_settings() -> iced::window::Settings {
    let img = image::load_from_memory_with_format(ICON_PNG_BYTES, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();
    iced::window::Settings {
        maximized: true,
        min_size: Some(iced::Size::new(720.0, 480.0)),
        icon: iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok(),
        position: iced::window::Position::Centered,
        ..Default::default()
    }
}
pub fn iced_app_settins(member_name: String) -> iced::Settings {
    iced::Settings {
        id: Some(format!("{} - {}", member_name, PROJECT_NAME)),
        default_text_size: iced::Pixels::from(24),
        default_font: FONT_NOTO_SANS_REG,
        vsync: true,
        fonts: vec![std::borrow::Cow::from(FONT_NOTO_SANS_REG_BYTES)],
        ..Default::default()
    }
}

#[derive(Debug)]
pub struct ToastManager {
    new_toasts: Vec<(Toast, time::Instant)>,
    toasts: Vec<Toast>,
}

impl ToastManager {
    pub fn new() -> Self {
        Self {
            new_toasts:Vec::new(),
            toasts:Vec::new()
        }
    }
    pub fn is_new_empty(&self)->bool{
        self.new_toasts.is_empty()
    }
    pub fn add(&mut self, toast: Toast){
        info!("Add toast: {:?}", toast);
        self.new_toasts.push((toast, time::Instant::now()));
    }
    pub fn process(&mut self) {
        let mut indexes = Vec::new();
        let mut index = 0;
        for (_toast, instant) in self.new_toasts.iter() {
            if instant.elapsed() > time::Duration::from_secs(10) {
                indexes.push(index);
            }
            index += 1;
        }
        for i in indexes {
            let (t, _) = self.new_toasts.remove(i);
            self.toasts.push(t);
        }
    }
    pub fn view<'a, Message:'a>(&self) -> scrollable::Scrollable<'a, Message> {
        let mut layout = Column::new().width(Length::Fill).height(Length::Fill).align_x(alignment::Horizontal::Right);
        if !self.new_toasts.is_empty(){
        for (toast, instant) in self.new_toasts.clone() {
            let mut tlayout = Column::new();
            match toast.title {
                Some(title) => {tlayout=tlayout.push(text(title).size(28))}
                None => {}
            }
            tlayout=tlayout.push(text(toast.msg).size(22));
            tlayout=tlayout.push(iced::widget::progress_bar(0.0..=10.0, instant.elapsed().as_secs_f32()));
            let tlevel = toast.level;
            layout=layout.push(container(tlayout).style(move|theme:&iced::Theme| {
                let p=theme.extended_palette();
                container::Style{
                    background: Some(Background::Color(match tlevel {
                        ToastLevel::Error => {p.danger.base.color}
                        ToastLevel::Info => {
                            p.primary.base.color
                        }
                        ToastLevel::Sucessful => {p.success.base.color}
                        ToastLevel::Warning => {p.warning.base.color}
                    })
                    ),..Default::default()}}))
        }}
        scrollable(Row::new().push(layout).width(Length::Fill).height(Length::Fill).align_y(alignment::Vertical::Bottom)).width(Length::Fill).height(Length::Fill)
    }
}

#[derive(Debug, Clone)]
pub struct Toast {
    level: ToastLevel,
    msg: String,
    title: Option<String>
}
impl Default for Toast {
    fn default() -> Self {
        Self { level: ToastLevel::Info, msg: String::new(), title: None }
    }
}
impl Toast {
    pub fn new(msg: String) -> Self {
        Self { msg,..Default::default() }
    }
    pub fn set_sucessful(mut self) -> Self {
        self.level=ToastLevel::Sucessful;
        self
    }
    pub fn set_info(mut self)->Self{
        self.level=ToastLevel::Info;
        self
    }
    pub fn set_warning(mut self)->Self{
        self.level=ToastLevel::Warning;
        self
    }
    pub fn set_error(mut self) ->Self{
        self.level=ToastLevel::Error;
        self
    }
    pub fn set_title(mut self, t: String) ->Self{
        self.title=Some(t);
        self
    }
}

#[derive(Debug, Clone)]
pub enum ToastLevel {
    Sucessful,
    Info,
    Warning,
    Error,
}
