# 簡介

`positive_mahjong` 使用 rust、tungstenite、iced 製作，支援 Linux 及 Windows 系統。

專案分為客戶端、伺服器、共用資料：

- `pmj_client_desktop`
- `pmj_client_cross`
- `pmj_server`
- `pmj_shared`
- `pmj_gamemodes`

伺服器預設使用 `6060` 端口。

---

`positive_mahjong` 專案自 2026年2月18日 開始開發，曾嘗試使用 `tiny-http` 和 `slint` 實現 V1Simple 玩法，也嘗試用 `iced` 跨平臺 UI，後因開發時間拉長延期，改為 `iced` 與 `tungstenite` 制作 Base 玩法，並於2026/8/16完成 Base 玩法。
