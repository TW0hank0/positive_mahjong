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

## [0.1.0] - 未發布

\>\> !第一版! <<

#### 紀錄

**2026-4-12 - 客戶端UI優化**

- 開始此紀錄
- 新增引用：material-symbols
- 新增zed 專案設定
- pmj_client：homepage UI 優化
- 移除V1Simple 玩法引用

**2026-4-18 - 文件docs**

- 新增英文版Readme: `READMEs/README_en.md`
- 新增readme 字體授權聲明
- 新增mdbook
- 協議模板移除無效欄位

**2026-4-25 - Android-Ks不再寫死**

- Android KeyStore 使用env動態讀取
- 客戶端Cargo.toml 移除無效Android設定
- 更新README
- 修復檔案Header

**2026-4-26**

- 修復CI
