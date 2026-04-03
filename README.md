# positive_mahjong

![icon](./assets/icon.png)

**仍在開發中，未完成**

伺服器使用Rust編寫

客戶端GUI使用Slint框架

![MadeWithSlint-logo](./assets/MadeWithSlint-logo-light.png)

### 連結

[Github Repo](https://github.com/TW0hank0/positive_mahjong)

[Codeberg Mirror Repo](https://codeberg.org/TW0hank0/positive_mahjong)

[Keep Android Open Website](https://keepandroidopen.org/)

### 檔案結構

**已過時**

```
project_root
└─ .cargo/ => Cargo設定
└─ .github/
...└─ workflows/ => 工作流(CI)
...└─ ISSUE_TEMPLATE/ => Issue模板
└─ assets/ => 資料
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
└─ secret/ => (**Ignored**) **不能上傳** 的資料 (含Android用Keystore)
...└─ TW0hank0.keystore => Android用
└─ res/ => Android用assets
└─ LICENSE => AGPL-3.0-only
└─ about.toml => cargo-about 設定
```

### 編譯

需求：uv(或官方Python)、rust工具鏈

**電腦**

執行：
```bash
uv run scripts/build_computer.py
```
或
```bash
cargo build --workspace --relase
```

**Android手機**

需求：cargo-apk(`cargo install cargo-apk`)、java、Android sdk+ndk、android-target(`rustup target add aarch64-linux-android`)

執行：
```bash
uv run scripts/build_android.py
```

**WEB-WASM**

未完成 原因：wasm 不支援sync只支援asyn

### 授權/聲明

版權所有 (C) 2026 TW0hank0

本程式基於 GNU Affero General Public License v3 授權

第三方專案授權見：

- [ThirdPartyLicense-Rust.html](./auto_generated/ThirdPartyLicense-Rust.html)
- [ThirdPartyLicense-Rust.md](./auto_generated/ThirdPartyLicense-Rust.md)
- [ThirdPartyLicense-Rust.json](./auto_generated/ThirdPartyLicense-Rust.json)

"The Slint Logo used in this project is licensed under [CC BY-ND 4.0](./assets/CC-BY-ND-4.0.txt)
. Original work by [Slint Developers/Sixtyfps GmbH]. No modifications have been made to the logo file."
(本專案使用的 Slint Logo 依據 [CC BY-ND 4.0](./assets/CC-BY-ND-4.0.txt) 授權。原始作者為 [Slint 開發團隊]。本專案未對該 Logo 檔案進行任何修改。)
