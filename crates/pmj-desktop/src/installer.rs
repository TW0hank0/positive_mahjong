use std::{io, path::PathBuf, fs};

use zip;
use iced::{self, Element, Length, Task, widget::{Column, Row, button, rule, space, text}};
use tracing::{trace, debug, info, warn, error};

use pmj_shared::shared::{self, ICON_PNG_BYTES, FONT_NOTO_SANS_REG_BYTES, PROJECT_NAME, PROJECT_VERSION};

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
        PmjInstaller::new,
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

#[derive(Debug,Clone)]
enum Scenes {
    Welcome,
    InstSmmary,
    Install,
}

impl Scenes {
    pub fn to_num(&self) -> u8 {
        match self {
            Scenes::Welcome => {1},
            Scenes::InstSmmary => {2},
            Self::Install => {3}
        }
    }
    pub fn next(&self) ->Option<Self> {
        match self {
            Scenes::Welcome => {Some(Self::InstSmmary)}
            Scenes::InstSmmary=> {Some(Self::Install)},
            Self::Install =>{None}
        }
    }
    pub fn prev(&self) -> Option<Self> {
        match self {
            Scenes::Welcome => {None},
            Scenes::InstSmmary => {Some(Scenes::Welcome)},
            Self::Install => {Some(Self::InstSmmary)}
        }
    }
}

#[derive(Debug, Clone)]
enum UIMessage {
    ExitInstaller,
    NextScene,
    PrevScene,
    StartInstall,
}
#[derive(Debug)]
struct InstSettings {
    location: PathBuf,
}
impl Default for InstSettings {
    fn default() -> Self {
        Self { location: dirs::executable_dir().unwrap_or(dirs::config_local_dir().unwrap()).join(PROJECT_NAME) }
    }
}
#[derive(Debug)]
struct PmjInstaller {
    theme: iced::Theme,
    scene: Scenes,
    inst_settings: InstSettings
}
impl PmjInstaller {
    pub fn new() -> Self {
        Self { theme: iced::Theme::TokyoNight,scene:Scenes::Welcome, inst_settings:InstSettings::default() }
    }
    pub fn update(&mut self, message: UIMessage) -> iced::Task<UIMessage> {
        info!("update: message={:?}", message);
        match message {
            UIMessage::StartInstall => {
                let temp_archive_path = dirs::download_dir().unwrap().join("pmj-desktop-installer-tempfile");
                fs::write(temp_archive_path.clone(), INST_STORED_BYTES).ok();
                let file = fs::File::open(temp_archive_path).unwrap();
                let mut archive = zip::ZipArchive::new(file).unwrap();
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();
                    let outpath = match file.enclosed_name() {
                            Some(path) => path.to_owned(),
                            None => continue,
                        };
                    let outpath = std::path::Path::new(&self.inst_settings.location).join(outpath);
                    if file.name().unwrap().ends_with('/') {
                            // 如果是目錄，直接建立
                            fs::create_dir_all(&outpath);
                        } else {
                            // 如果是檔案，確保父目錄存在並寫入內容
                            if let Some(p) = outpath.parent() {
                                if !p.exists() {
                                    fs::create_dir_all(p).unwrap();
                                }
                            }
                            let mut outfile = fs::File::create(&outpath).unwrap();
                            io::copy(&mut file, &mut outfile).unwrap();
                        }}
            }
            UIMessage::ExitInstaller => {
                return iced::exit();
            }
            UIMessage::NextScene => {
                match self.scene.next() {
                    Some(n) => {self.scene=n}
                    None=>{}
                }
            }
            UIMessage::PrevScene => {
                match self.scene.prev() {
                    Some(p) => {self.scene=p}
                    None=>{}
                }
            }
        }
        Task::none()
    }
    pub fn view(&self) -> Element<'_, UIMessage, iced::Theme, iced::Renderer> {
        let mut layout: Vec<Element<'_, UIMessage, iced::Theme, iced::Renderer>> = Vec::new();
        match self.scene {
            Scenes::Welcome => {
                layout.push(text(format!("{} v{} 安裝程式", PROJECT_NAME, PROJECT_VERSION)).size(32).into());
                layout.push(rule::horizontal(1).into());
                layout.push(space().height(10).into());
                layout.push(text(format!("這是 {} v{} 的安裝程式，此安裝程式之目的為幫您完成安裝流程。", PROJECT_NAME, PROJECT_VERSION))
                    .size(18).into());
            }
            Scenes::InstSmmary => {
                layout.push(text("安裝總覽").size(32).into());
                layout.push(rule::horizontal(1).into());
                layout.push(text(format!("安裝位子：{}", self.inst_settings.location.display())).into());
            }
            Scenes::Install => {
                layout.push(text("安裝").size(32).into());
            }
        }
        // 按鈕
        {
            layout.push(space().height(Length::Fill).into());
            layout.push(rule::horizontal(1).into());
            layout.push(space().height(3).into());
            let mut buttons = Vec::new();
        buttons.push(button(text("退出").size(20)).on_press(UIMessage::ExitInstaller).into());
        buttons.push(space().width(20).into());
        buttons.push(button(text("返回").size(20)).on_press(UIMessage::PrevScene).into());
        buttons.push(space().width(20).into());
        buttons.push(button(text("繼續").size(20)).on_press(UIMessage::NextScene).into());
        layout.push(Row::from_vec(buttons).into());}
        Column::from_vec(layout).padding(7).spacing(3).height(Length::Fill).width(Length::Fill).into()
    }
    pub fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }
    pub fn title(&self) -> String {
        format!("pmj-desktop-installer - {}", PROJECT_NAME)
    }
}
