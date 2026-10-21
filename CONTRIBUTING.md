# 貢獻說明

說明如何對 positive_mahjong 進行貢獻。

專案儲存庫在 [gitlab.com/TW0hank0/positive_mahjong](https://gitlab.com/TW0hank0/positive_mahjong)。

### 工具安裝

#### Rust

專案使用 Rust 語言開發，需要編譯器及相關工具。

建議使用 [Rustup](https://rustup.rs/) 安裝。

#### uv

專案使用 Python 作為腳本語言，此工具用於下載依賴和執行 scripts。

<https://docs.astral.sh/uv/>。

#### cargo-deny

`cargo install cargo-deny`

#### cargo-about

`cargo install --locked cargo-about --features="cli"`

#### cargo-vet

`cargo install --locked cargo-vet`

#### tombi

`uv tool install tombi`

#### reuse-tool

`uv tool install reuse[charset-normalizer]`

#### mdbook

`cargo install --locked mdbook`

### AI/LLM

禁止 AI/LLM 生成的程式碼，僅允許使用 AI/LLM 作為原型參考，**所有使用 AI/LLM 的內容須披露**。
