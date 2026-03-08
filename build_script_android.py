#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import sys
import subprocess
import shutil
from typing import List

# === 設定常數 ===
# Rust 目標成員名稱
TARGET_CRATE: str = "pmj_client"

# Android 專案相對路徑 (相對於執行腳本的根目錄)
ANDROID_PROJECT_DIR: str = "android"
JNI_LIBS_DIR: str = os.path.join(
    ANDROID_PROJECT_DIR, "app", "src", "main", "jniLibs"
)

# 支援的 ABI 架構
TARGET_ARCHS: List[str] = ["arm64-v8a", "armeabi-v7a"]

# Cargo 命令參數
CARGO_CMD: str = "cargo"
CARGO_NDK_PLUGIN: str = "ndk"


def check_prerequisites() -> bool:
    """
    檢查必要工具是否已安裝 (rustc, cargo, cargo-ndk)。
    回傳: bool (成功為 True)
    """
    print("[檢查] 驗證開發環境...")

    # 檢查 cargo
    try:
        subprocess.run(
            [CARGO_CMD, "--version"],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
    except FileNotFoundError:
        print("[錯誤] 未發現 'cargo'。請先安裝 Rust。")
        return False
    except subprocess.CalledProcessError:
        print("[錯誤] 'cargo' 執行失敗。")
        return False

    # 檢查 cargo-ndk
    try:
        # 嘗試執行 cargo ndk --version 來確認插件存在
        subprocess.run(
            [CARGO_CMD, CARGO_NDK_PLUGIN, "--version"],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
    except FileNotFoundError:
        print("[錯誤] 未發現 'cargo-ndk' 插件。")
        print("[提示] 請執行: cargo install cargo-ndk")
        return False
    except subprocess.CalledProcessError:
        # 有些舊版本可能沒有 --version，但至少命令要存在
        pass

    print("[成功] 環境檢查通過。")
    return True


def clean_jni_libs() -> None:
    """
    清理舊的 .so 檔案以避免衝突。
    """
    if os.path.exists(JNI_LIBS_DIR):
        print(f"[清理] 移除舊的函式庫目錄：{JNI_LIBS_DIR}")
        # 只刪除架構資料夾，保留目錄結構
        for arch in TARGET_ARCHS:
            arch_path: str = os.path.join(JNI_LIBS_DIR, arch)
            if os.path.exists(arch_path):
                shutil.rmtree(arch_path)
    else:
        os.makedirs(JNI_LIBS_DIR, exist_ok=True)
        print(f"[建立] 建立目標目錄：{JNI_LIBS_DIR}")


def build_rust_library() -> bool:
    """
    執行 cargo ndk 編譯 pmj_client。
    回傳: bool (成功為 True)
    """
    print(
        f"[編譯] 開始編譯 '{TARGET_CRATE}' for Android ({', '.join(TARGET_ARCHS)})..."
    )

    # 建構命令
    # cargo ndk -t <arch> -o <output_dir> build -p <package> --release
    base_cmd: List[str] = [
        CARGO_CMD,
        CARGO_NDK_PLUGIN,
        "-o",
        JNI_LIBS_DIR,
        "build",
        "-p",
        TARGET_CRATE,
        "--release",
    ]

    # 加入目標架構參數
    for arch in TARGET_ARCHS:
        base_cmd.insert(2, "-t")
        base_cmd.insert(3, arch)

    print(f"[指令] {' '.join(base_cmd)}")

    try:
        # 執行編譯，將輸出導向終端以便查看進度
        result: subprocess.CompletedProcess = subprocess.run(
            base_cmd, check=True, text=True
        )
        print("[成功] Rust 函式庫編譯完成。")
        return True
    except subprocess.CalledProcessError as e:
        print(f"[錯誤] 編譯失敗！返回碼：{e.returncode}")
        if e.output:
            print(f"[輸出] {e.output}")
        return False
    except FileNotFoundError:
        print("[錯誤] 無法執行 cargo 命令，請確認路徑。")
        return False


def verify_outputs() -> bool:
    """
    驗證生成的 .so 檔是否存在於正確位置。
    回傳: bool
    """
    print("[驗證] 檢查生成的共享庫...")
    all_found: bool = True

    for arch in TARGET_ARCHS:
        lib_path: str = os.path.join(JNI_LIBS_DIR, arch, "libpmj_client.so")
        if os.path.exists(lib_path):
            file_size: int = os.path.getsize(lib_path)
            print(f"  [OK] {arch}: libpmj_client.so ({file_size} bytes)")
        else:
            print(f"  [失敗] {arch}: 找不到 libpmj_client.so")
            all_found = False

    return all_found


def main() -> None:
    """
    主程式入口點。
    """
    print("=" * 50)
    print("Positive Mahjong (pmj_client) Android 構建腳本")
    print("開發者：TW0hank0 | 包名：io.github.positive_mahjong.tw0hank0")
    print("=" * 50)

    # 1. 檢查當前目錄是否包含 Cargo.toml (Workspace 根目錄)
    if not os.path.exists("Cargo.toml"):
        print(
            "[錯誤] 請在 Rust Workspace 根目錄下執行此腳本 (需包含 Cargo.toml)。"
        )
        sys.exit(1)

    # 2. 檢查環境
    if not check_prerequisites():
        sys.exit(1)

    # 3. 清理舊檔案
    clean_jni_libs()

    # 4. 執行編譯
    if not build_rust_library():
        print("[終止] 因編譯失敗，腳本結束。")
        sys.exit(1)

    # 5. 驗證結果
    if verify_outputs():
        print("=" * 50)
        print("[完成] 所有步驟成功！")
        print(f"[下一步] 請進入 {ANDROID_PROJECT_DIR} 目錄執行 Gradle 構建：")
        print(f"         cd {ANDROID_PROJECT_DIR} && ./gradlew assembleDebug")
        print("=" * 50)
    else:
        print("[警告] 部分檔案生成失敗，請檢查日誌。")
        sys.exit(1)


if __name__ == "__main__":
    main()
