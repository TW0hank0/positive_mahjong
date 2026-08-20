# positive_mahjong 的版本更改紀錄

這裡會紀錄版本間的更改。

格式如下：

```plaintext
{每個版本：

## [{ 版本 }] - { publish? {發布日期} : "未發布" }

{版本更改}

#### 紀錄

{每日開發：
**{日期}**
{開發/更改 內容}
}
}
```

發佈版本時使用：

```
git tag -a "版本" -m "release pmj: 版本"
```

---

## [未發布]

#### 開發紀錄

**2026-8-17**

- ［V2Better］分離 `room_msg` `game_msg`

**2026-8-20**

- init [pmj_ccore](./TODO.md#pmj_client_core)

## [v0.1.1] - 2026-8-17

修復客戶端錯誤，並改進訊息顯示。

#### 開發紀錄

**2026-8-16**

- 清理無用程式碼
- 修復：cargo-about
- init V2Better

## [v0.1.0] - 2026/8/16

=== 第一版! \^_\^ ===

#### 開發紀錄

**2026-4-12 - 客戶端UI優化**

- 新增：開始此紀錄
- 新增：引用material-symbols
- 新增：zed 專案設定
- 更改：pmj_client 的homepage UI 優化
- 移除：V1Simple 玩法引用

**2026-4-18 - 文件docs**

- 新增：英文版Readme: `READMEs/README_en.md`
- 新增：Readme 字體授權聲明
- 新增：Mdbook Docs
- 移除：協議模板無效欄位

**2026-4-25 - Android-Ks不再寫死**

- 更改：Android KeyStore 使用env動態讀取
- 更改：客戶端Cargo.toml 移除無效Android設定
- 更改：更新README
- 修復：檔案Header

**2026-4-26**

- 修復：CI
- 移除：`pmj_test_connection` (將整合`pmj_client`)
- 更改：玩法獨立

**2026-5-2 - Gitlab**

- 更改：主Repo 移至Gitlab

**2026-5-3**

- 更改：客戶端改用iced框架

**2026-5-16**

- 修復：fix ci

**2026-5-17**

- 移除：unneed slint dep
- 新增：使用taplo格式化toml檔案

**2026-5-23**

- 修復：過時的英文版README
- 修復：Github CI

**2026-5-24**

- 新增：連接動畫（實驗性功能）

**2026-6-6**

- 更改：所有工作區成員移至 `crates` 資料夾
- 更改：原 `pmj_client` 改名 `pmj_client_desktop`
- 移除：現 `pmj_client_desktop` 對Android 的支援
- 新增：跨平臺專用 `pmj_client_cross`

**2026-6-13**

- 新增：cargo-deny設定檔
- 修復：說明文件無法正常使用
- 移除：Gitlab issue 模板中的無用內容
- 修復：gdextension設定

**2026-6-19**

- 新增：［pmj_client_cross］跨平臺客戶端編譯腳本
- 移除：［ci］Github 的儲存庫同步

**2026-7-2**

- 修復：舊的檔案 header 指向 github，現已指向 gitlab
- 新增：開始開發網頁
- 修復：［pmj_server］初訊息 `try_lock failed because the operation would block` 錯誤
- 棄用：Github issue templates

**2026-7-6**

- 更改：［CI］使用新的 `website/` 網頁
- 更改：［CI］使用 `mold` 作為 Linux 系統的 linker
- 移除：［website］不再使用 iframe nav

**2026-7-14**

- 更改：workspace 級 `iced`
- 開發：README 英文版
- 修復：［pmj_server］初訊息未傳送
- 修復：網頁部署

**2026-7-15**

- 新增：［website］KAO 與 KSCD 的提示/說明

**2026-7-24**

- 移除：無用依賴
- 移除：儲存庫的 `auto_generated/`，請至說明文件 <https://tw0hank0.gitlab.io/positive_mahjong/docs/license/>
- 棄用：`.github/workflows/{commit-build.yaml, commit-check.yaml}`，移至 `.github/workflows/commit-ci.yaml`
- 更改：`.github/workflows/docs-build.yaml` 改名 `.github/workflows/pages-deploy.yaml`

**2026-7-28**

- 更改：從 `log` 改為 `tracing`

**2026-7-29**

- 新增：［website］協議聲明

**2026-8-5**

- 移除：［ci］commit-msrv.yaml
- 更改：［pmj_shared］日誌儲存位子

**2026-8-11**

- 新增：［pmj_gamemodes::base && pmj_client_desktop］定期 ping

**2026-8-13**

- 新增：［docs::intro］開發時間序

**2026-8-14**

- 修復：［pmj_sever::gui::base］iced window 設定未套用

**2026-8-16**

- 新增：［pmj_client_desktop］新增讀取超時
