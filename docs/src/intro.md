# 簡介

`positive_mahjong` 使用Rust和Iced製作，支援Linux、Android及Windows系統。

專案包含客戶端及伺服器，Workspace分為四個Packages：

- `pmj_client`
- `pmj_server`
- `pmj_shared`
- `pmj_gamemodes`

客戶端連線時請注意，格式為：「ws://{伺服器Ip}:{端口號(預設為6060)}」。
