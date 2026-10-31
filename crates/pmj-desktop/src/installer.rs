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

use std::{fmt::Display, fs, io, path::PathBuf, sync::mpsc, thread};

use iced::{
    self, Element, Length, Task, widget::{Column, Row, Stack, button, rule, scrollable, space, text},
};
use tracing::{debug, error, info, trace, warn};

use pmj_shared::shared::{
    self, PROJECT_NAME, PROJECT_VERSION,
};
use pmj_desktop::shared::{iced_app_settings, iced_window_settings, FONT_NOTO_SANS_REG, Toast, ToastManager};

const MEMBER_NAME: &str = "pmj-desktop-installer";
const INST_STORED_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/inst-stored.zip"));

fn main() {
    let _guard = shared::init_tracing_fmt(String::from(MEMBER_NAME));
    let iced_result = iced::application(
        PmjInstaller::new,
        PmjInstaller::update,
        PmjInstaller::view,
    )
    .window(iced_window_settings())
    .settings(iced_app_settings(String::from("pmj-desktop-installer")))
    .default_font(FONT_NOTO_SANS_REG)
    .title(PmjInstaller::title)
    .theme(PmjInstaller::theme)
    .run();
    match iced_result {
        Ok(_) => {
            debug!("iced::Result::Ok");
        }
        Err(e) => {
            error!("iced::Result::Err => {}", e);
        }
    }
}

#[derive(Debug)]
enum Scenes {
    Welcome,
    InstSummary,
    Install(Option<mpsc::Receiver<InstState>>),
    Installed,
}

impl Scenes {
    /* pub fn to_num(&self) -> u8 {
        match self {
            Scenes::Welcome => {1},
            Scenes::InstSmmary => {2},
            Self::Install => {3}
        }
    } */
    pub fn next(&self) -> Self {
        match self {
            Self::Welcome => Self::InstSummary,
            Self::InstSummary => Self::Install(None),
            Self::Install(_) => {
                panic!("call Scenes::Install.next()")
            }
            Self::Installed => {
                panic!("call Scenes::Installed.next()")
            }
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            Scenes::Welcome => {
                panic!("call Scenes::Welcome.prev()")
            }
            Scenes::InstSummary => Scenes::Welcome,
            Self::Install(_) => Self::InstSummary,
            Scenes::Installed => {
                panic!("call Scenes::Installed.prev()")
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ExitInstaller,
    NextScene,
    PrevScene,
    StartInstall,
    UpdateInstallState,
    ToastUpdate,
}
#[derive(Debug, Clone)]
struct InstSettings {
    location: PathBuf,
}
impl Default for InstSettings {
    fn default() -> Self {
        Self {
            location: dirs::executable_dir()
                .unwrap_or(dirs::config_local_dir().unwrap())
                .join(PROJECT_NAME),
        }
    }
}
#[derive(Debug)]
struct PmjInstaller {
    theme: iced::Theme,
    scene: Scenes,
    inst_settings: InstSettings,
    installing: bool,
    install_states: Vec<InstState>,
    toast:ToastManager
}

#[derive(Debug)]
enum InstState {
    UnzippingFile(PathBuf),
    InstallFinish
}
impl Display for InstState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::UnzippingFile(path) => {format!("解壓縮檔案：{}", path.display())}
            Self::InstallFinish => {String::from("已安裝")}
        }.as_str())
    }
}

impl Default for PmjInstaller {
    fn default() -> Self {
        Self {
            theme: iced::Theme::TokyoNight,
            scene: Scenes::Welcome,
            inst_settings: InstSettings::default(),
            installing: false,
            install_states: Vec::new(),
            toast:ToastManager::new_and_add(MEMBER_NAME.to_string())
        }
    }
}

impl PmjInstaller {
    pub fn new() -> (Self, iced::Task<Message>) {
        (Self::default(), iced::Task::done(Message::ToastUpdate))
    }
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        trace!("update: message={:?}", message);
        match message {
            Message::ToastUpdate => {
                self.toast.process();
                return Task::done(Message::ToastUpdate);
            }
            Message::StartInstall => {
                self.installing=true;
                self.start_install();
                self.toast.add(Toast::new(format!("開始安裝 {} v{}", PROJECT_NAME, PROJECT_VERSION)));
                return Task::done(Message::UpdateInstallState);
            }
            Message::ExitInstaller => {
                return iced::exit();
            }
            Message::NextScene => {
                self.scene = self.scene.next();
            }
            Message::PrevScene => {
                self.scene = self.scene.prev();
            }
            Message::UpdateInstallState => match self.scene {
                Scenes::Install(ref maybe_rx) => {
                    if let Some(rx) = maybe_rx {
                        match rx.try_recv() {
                            Ok(inst_state) => {
                                if matches!(inst_state, InstState::InstallFinish) {
                                    info!("update: matches!(inst_state, InstState::InstallFinish)");
                                    self.scene = Scenes::Installed;
                                    self.installing=false;
                                } else {
                                    self.install_states.push(inst_state);}
                            }
                            Err(mpsc::TryRecvError::Empty) => {}
                            Err(mpsc::TryRecvError::Disconnected) => {
                                panic!("update: Err(mpsc::TryRecvError::Disconnected)");
                            }
                        }
                    }
                    return Task::done(Message::UpdateInstallState);
                }
                _ => {
                    warn!("update: warn scene");
                }
            },
        }
        Task::none()
    }
    pub fn view(&self) -> Element<'_, Message, iced::Theme, iced::Renderer> {
        let mut layout: Vec<Element<'_, Message, iced::Theme, iced::Renderer>> = Vec::new();
        let btn_exitable = !self.installing;
        let mut btn_continueable = true;
        let mut btn_backable = true;
        let page_title: String;
        let mut content: Vec<Element<'_, Message, iced::Theme, iced::Renderer>> = Vec::new();
        match self.scene {
            Scenes::Welcome => {
                btn_backable = false;
                page_title = format!("{} v{} 安裝程式", PROJECT_NAME, PROJECT_VERSION);
                content.push(
                    text(format!(
                        "這是 {} v{} 的安裝程式，此安裝程式之目的為幫您完成安裝流程。",
                        PROJECT_NAME, PROJECT_VERSION
                    ))
                    .size(20)
                    .into(),
                );
            }
            Scenes::InstSummary => {
                page_title = String::from("安裝總覽");
                content.push(
                    text(format!(
                        "安裝位子：{}",
                        self.inst_settings.location.display()
                    ))
                    .into(),
                );
            }
            Scenes::Install(_) => {
                btn_continueable = false;
                if self.installing {
                    page_title = String::from("安裝中");
                    content.push(scrollable(
                        self.install_states.iter()
                            .fold(Column::new().spacing(1).padding(2), |layout, item| {
                            layout.push(text(item.to_string()))
                        })
                    ).direction(scrollable::Direction::Vertical(scrollable::Scrollbar::new().margin(3)
                    .spacing(5)
                    .width(3)
                    .scroller_width(6))).into());
                } else {
                    page_title = String::from("準備安裝");
                    content.push(
                        button(text("開始安裝"))
                            .on_press(Message::StartInstall)
                            .into(),
                    );
                }
            }
            Scenes::Installed => {
                btn_backable=false;
                btn_continueable=false;
                page_title = String::from("安裝完成")
            }
        }
        layout.push(
            Column::new()
                .push(text(page_title).size(32))
                .push(rule::horizontal(1))
                .into(),
        );
        layout.push(space().height(10).into());
        layout.push(Column::from_vec(content).spacing(3).into());
        // 按鈕
        {
            layout.push(space().height(Length::Fill).into());
            layout.push(rule::horizontal(1).into());
            layout.push(space().height(3).into());
            let mut buttons = Vec::new();
                buttons.push(
                    button(text("退出").size(22))
                        .on_press_maybe(if btn_exitable{Some( Message::ExitInstaller)} else {None})
                        .into(),
                );
                buttons.push(
                    button(text("返回").size(22))
                        .on_press_maybe(if btn_backable {Some(Message::PrevScene)} else {None})
                        .into(),
                );
                buttons.push(
                    button(text("繼續").size(22))
                        .on_press_maybe(if btn_continueable {Some(Message::NextScene)} else {None})
                        .into(),
                );
            layout.push(Row::from_vec(buttons).spacing(20).into());
        }
        Stack::new().push(Column::from_vec(layout)
            .padding(5)
            .spacing(3)
            .height(Length::Fill)
            .width(Length::Fill)).push(self.toast.view())
            .into()
    }
    pub fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }
    pub fn title(&self) -> String {
        format!("{} - {}", MEMBER_NAME, PROJECT_NAME)
    }
    fn start_install(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.scene = Scenes::Install(Some(rx));
        let inst_settings = self.inst_settings.clone();
        thread::spawn(move || {
            let mut archive = zip::ZipArchive::new(io::Cursor::new(INST_STORED_BYTES)).unwrap();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let outpath = match file.enclosed_name() {
                    Some(path) => path.to_owned(),
                    None => continue,
                };
                tx.send(InstState::UnzippingFile(outpath.clone())).ok();
                let outpath = inst_settings.location.join(outpath);
                if file.name().unwrap().ends_with('/') {
                    fs::create_dir_all(&outpath).ok();
                } else {
                    if let Some(p) = outpath.parent()
                        && !p.exists()
                    {
                        fs::create_dir_all(p).unwrap();
                    }
                    let mut outfile = fs::File::create(&outpath).unwrap();
                    io::copy(&mut file, &mut outfile).unwrap();
                }
            }
            tx.send(InstState::InstallFinish).ok();
        });
    }
}
