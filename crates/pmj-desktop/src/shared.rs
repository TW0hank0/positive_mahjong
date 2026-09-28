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

use iced::{
    self, Border, border,
    widget::{button, container},
};

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
            Self::BackgroundOutlined => |theme:&iced::Theme|{
                let p = theme.extended_palette();
                container::Style { border: Border {
                    color: p.background.strong.color,
                    width:0.7,
                    radius:border::radius(8),
                },..Default::default() }
            }
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
                        button::Status::Disabled => {Some(iced::Background::Color(
                            p.background.weak.color,
                        ))}
                        _=>{None}
                    },
                    text_color:p.background.base.text,
                    border: Border {
                        width: match status {
                            button::Status::Active | button::Status::Disabled => {1.2}
                            button::Status::Hovered => {1.5}
                            button::Status::Pressed => {0.7}
                        },
                        color: match status {
                            button::Status::Active | button::Status::Disabled =>{p.background.strong.color}
                            button::Status::Hovered => {p.primary.weak.color}
                            button::Status::Pressed => {p.primary.strong.color}
                        },
                        radius: border::radius(match status {
                            button::Status::Active | button::Status::Disabled | button::Status::Hovered => {10}
                            button::Status::Pressed => {6}
                        })
                    },
                    ..Default::default()
                }
            }
        }
    }
}
