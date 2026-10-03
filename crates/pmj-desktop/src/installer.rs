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

use std::{fs, io, path::PathBuf, sync::mpsc, thread};

use iced::{
    self, Element, Length, Task,
    widget::{Column, Row, button, rule, space, text},
};
use tracing::{debug, error, info, warn};

use pmj_shared::shared::{
    self, FONT_NOTO_SANS_REG_BYTES, ICON_PNG_BYTES, PROJECT_NAME, PROJECT_VERSION,
};

const INST_STORED_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/inst-stored.zip"));
const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");

pub fn icon_init() -> Option<iced::window::Icon> {
    let img = image::load_from_memory_with_format(ICON_PNG_BYTES, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();
    iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok()
}

fn main() {
    let _guard = shared::init_tracing_fmt(String::from("pmj-desktop-installer"));
    let window_settings = iced::window::Settings {
        maximized: false,
        min_size: Some(iced::Size::new(480.0, 280.0)),
        icon: icon_init(),
        position: iced::window::Position::Centered,
        ..Default::default()
    };
    let app_settings = iced::Settings {
        id: Some(format!("{} - pmj-desktop-installer", PROJECT_NAME)),
        default_text_size: iced::Pixels::from(24),
        default_font: FONT_NOTO_SANS_REG,
        vsync: true,
        fonts: vec![std::borrow::Cow::from(FONT_NOTO_SANS_REG_BYTES)],
        ..Default::default()
    };
    let iced_result = iced::application(
        PmjInstaller::default,
        PmjInstaller::update,
        PmjInstaller::view,
    )
    .window(window_settings)
    .settings(app_settings)
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
    Install(Option<mpsc::Receiver<String>>),
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
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            Scenes::Welcome => {
                panic!("call Senes::Welcome.prev()")
            }
            Scenes::InstSummary => Scenes::Welcome,
            Self::Install(_) => Self::InstSummary,
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
    install_state: Option<String>,
}

impl Default for PmjInstaller {
    fn default() -> Self {
        Self {
            theme: iced::Theme::TokyoNight,
            scene: Scenes::Welcome,
            inst_settings: InstSettings::default(),
            installing: false,
            install_state: None,
        }
    }
}

impl PmjInstaller {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        info!("update: message={:?}", message);
        match message {
            Message::StartInstall => {
                self.start_install();
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
                Scenes::Install(ref maybe_rx) => if let Some(rx) = maybe_rx { match rx.try_recv() {
                    Ok(inst_state) => {
                        self.install_state = Some(inst_state);
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                    Err(mpsc::TryRecvError::Disconnected) => {
                        panic!("update: Err(mpsc::TryRecvError::Disconnected)");
                    }
                } },
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
                    .size(18)
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
                    if let Some(ref inst_state) = self.install_state {
                        content.push(text(inst_state.clone()).into());
                    }
                } else {
                    page_title = String::from("準備安裝");
                    content.push(
                        button(text("開始安裝"))
                            .on_press(Message::StartInstall)
                            .into(),
                    );
                }
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
            if btn_exitable {
                buttons.push(
                    button(text("退出").size(20))
                        .on_press(Message::ExitInstaller)
                        .into(),
                );
            }
            if btn_backable {
                buttons.push(
                    button(text("返回").size(20))
                        .on_press(Message::PrevScene)
                        .into(),
                );
            }
            if btn_continueable {
                buttons.push(
                    button(text("繼續").size(20))
                        .on_press(Message::NextScene)
                        .into(),
                );
            }
            layout.push(Row::from_vec(buttons).spacing(20).into());
        }
        Column::from_vec(layout)
            .padding(5)
            .spacing(3)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
    pub fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }
    pub fn title(&self) -> String {
        format!("pmj-desktop-installer - {}", PROJECT_NAME)
    }
    fn start_install(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.scene = Scenes::Install(Some(rx));
        let inst_settings = self.inst_settings.clone();
        thread::spawn(move || {
            let temp_archive_path = dirs::download_dir()
                .unwrap()
                .join("pmj-desktop-installer-tempfile");
            fs::write(temp_archive_path.clone(), INST_STORED_BYTES).ok();
            let file = fs::File::open(temp_archive_path).unwrap();
            let mut archive = zip::ZipArchive::new(file).unwrap();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let outpath = match file.enclosed_name() {
                    Some(path) => path.to_owned(),
                    None => continue,
                };
                tx.send(outpath.display().to_string()).ok();
                let outpath = inst_settings.location.join(outpath);
                if file.name().unwrap().ends_with('/') {
                    // 如果是目錄，直接建立
                    fs::create_dir_all(&outpath).ok();
                } else {
                    // 如果是檔案，確保父目錄存在並寫入內容
                    if let Some(p) = outpath.parent()
                        && !p.exists() {
                            fs::create_dir_all(p).unwrap();
                        }
                    let mut outfile = fs::File::create(&outpath).unwrap();
                    io::copy(&mut file, &mut outfile).unwrap();
                }
            }
        });
    }
}
