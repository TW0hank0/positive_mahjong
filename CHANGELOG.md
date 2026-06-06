# positive_mahjong 的版本更改紀錄

這裡會紀錄版本間的更改。

格式如下：

```plaintext
{每個版本：

## [{ 版本 }] - { publish? {發布日期} : "未發布" }

{版本更改}

#### 紀錄

{每日開發：
**{日期} - {當日開發重點}**
{開發/更改 內容}
}
}
```

---

## [未發布]

=== 第一版! ^\_^ ===

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
