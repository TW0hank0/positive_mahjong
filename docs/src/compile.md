# 編譯

## 電腦

第一次編譯的準備：

- Android sdk+ndk (需在Path)
- Keystore
  > PATH: `CARGO_APK_RELEASE_KEYSTORE` 和 `CARGO_APK_RELEASE_KEYSTORE_PASSWORD`

```
python scripts/build_license.py
```

使用編譯腳本：

```bash
python scripts/build_computer.py
```

## Android 手機

```bash
python scripts/build_android.py
```

## wasm-web

**開發中，未完成**

```bash
python scripts/build_wasm.py
```
