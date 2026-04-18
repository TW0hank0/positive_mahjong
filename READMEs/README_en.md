# positive_mahjong

![icon](./assets/icon.png)

**繁體中文** | [English](READMEs/README_en.md)

**Please use the Chinese version as the main one.**

### Features

**Still in deving. Not finish.**

written in rust

Server's gui framework is `iced`, client's gui framework is `slint`.

![MadeWithSlint-logo](./assets/MadeWithSlint-logo-light.png)

### Links

[Github Repo](https://github.com/TW0hank0/positive_mahjong)

[Codeberg Mirror Repo](https://codeberg.org/TW0hank0/positive_mahjong)

[Keep Android Open Website](https://keepandroidopen.org/)

### File struct

```
project_root
└─ .cargo/ => Cargo settings
└─ .github/
...└─ workflows/ => workflow (CI)
...└─ ISSUE_TEMPLATE/ => Issue templates
└─ assets/ => project assets
└─ auto_generate/ => 使用腳本產生的檔案
└─ ci/ => 工作流 (workflow - CI)
└─ pmj_client/ => 客戶端 (rust)
...└─ ui/ => Slint UI檔案
...└─ src/ => Rust 客戶端程式碼
└─ pmj_server/ => 伺服器 (rust)
└─ pmj_shared/ => 共用資料
...└─ src/
......└─ shared.rs => 通用資料 (玩法通用資料)
......└─ gamemodes_shared/ => 玩法資料
└─ pmj_test_connection/ => 測試連線
└─ scripts/ => 腳本 (含編譯腳本)
└─ templates/ => 模板
...└─ about_html.hbs => cargo-about的html格式生成模板
...└─ about_json.hbs => cargo-about的json格式生成模板
...└─ about_markdown.hbs => cargo-about的markdown格式生成模板
...└─ addlicense.template => addlicense的檔案Headler模板
└─ secret/ => (**Ignored**) **不能上傳** 的資料
...└─ TW0hank0.keystore => Android用 Keystore
└─ res/ => Android用 assets
└─ LICENSE => AGPL-3.0-only
└─ about.toml => cargo-about 設定
```

### Compile

Need rust toolchain.

**Computer**

Run：

```bash
uv run scripts/build_computer.py
```

**Android phone**

Need cargo-apk(`cargo install cargo-apk`), java, Android sdk+ndk and android-target rust-std(`rustup target add aarch64-linux-android`)

Run：

```bash
uv run scripts/build_android.py
```

**WEB-WASM**

Not support now.

### 授權與聲明

版權所有 (C) 2026 TW0hank0

本程式基於 GNU Affero General Public License v3 授權

第三方專案授權見：

- [ThirdPartyLicense-Rust.html](./auto_generated/ThirdPartyLicense-Rust.html)
- [ThirdPartyLicense-Rust.md](./auto_generated/ThirdPartyLicense-Rust.md)
- [ThirdPartyLicense-Rust.json](./auto_generated/ThirdPartyLicense-Rust.json)

**Slint Logo**

檔案路徑：`assets/MadeWithSlint-logo-light.png`

本專案使用的 `Slint Logo` 依據 [CC BY-ND 4.0](./assets/CC-BY-ND-4.0.txt) 授權。作者為 [Slint 開發團隊]。本專案未對該 Logo 檔案進行任何修改。

**Material Design 3 component set for Slint**

資料夾路徑：`pmj_client/material/`

本專案使用的 `Material Design 3 component set for Slint` 依據 [MIT License](pmj_client/material/LICENSE.md) 授權。作者為 [Slint 開發團隊]。

**Noto Sans TC**

資料夾路徑：`assets/Noto_Sans_TC/`

本專案使用的 `Noto Sans TC` 依據 [SIL OPEN FONT LICENSE Version 1.1](assets/Noto_Sans_TC/OFL.txt) 授權。作者為 [Google 與 Adobe]。

**Material Symbols**

資料夾路徑：`assets/material_symbols`

本專案使用的 `Material Symbols` 依據 [Apache License Version 2.0](assets/material_symbols/LICENSE) 授權。作者為 [Google]。
