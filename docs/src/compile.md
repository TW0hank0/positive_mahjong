# 編譯

## 電腦

編譯需求：

- Rust toolchain
- uv

第一次編譯需要產生第三方授權聲明：

```bash
uv run scripts/build_license.py
```

使用編譯腳本：

```bash
uv run scripts/build_computer.py
```

## Android 手機

編譯需求：

- Rust toolchain
- uv
- Rustup android target： `aarch64-linux-android`
- Android sdk+ndk (需在Path)
- Keystore
- (ENV) `CARGO_APK_RELEASE_KEYSTORE` ：Keystore檔案路徑
- (ENV) `CARGO_APK_RELEASE_KEYSTORE_PASSWORD` ：Keystore檔案密碼

第一次編譯需要產生第三方授權聲明：

```bash
uv run scripts/build_license.py
```

使用編譯腳本：

```bash
uv run  scripts/build_android.py
```

## wasm-web

**開發中，未完成**

使用編譯腳本：

```bash
python scripts/build_wasm.py
```
