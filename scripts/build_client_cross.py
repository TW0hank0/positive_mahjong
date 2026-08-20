# SPDX-License-Identifier: AGPL-3.0-only
# 著作權所有 (C) 2026 TW0hank0
#
# 本檔案屬於 positive_mahjong 專案的一部分。
# 專案儲存庫：https://gitlab.com/TW0hank0/positive_mahjong
#
# 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
# 第 3 版（僅此版本）重新發佈及/或修改本程式。
#
# 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
# 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
#
# 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
# 如果沒有，請參見 <https://www.gnu.org/licenses/>。

import platform
import sys

import util


def main():
    # [ [ export_name, rust_target_name ] ]
    platforms: list[tuple[str, str, str]] = [
        (
            "Android-arm64",
            "aarch64-linux-android",
            "build/pmj_client_cross/pmj_client_cross-android-arm64.apk",
        ),
        (
            "Android-x86_64",
            "x86_64-linux-android",
            "build/pmj_client_cross/pmj_client_cross-android-x86_64.apk",
        ),
        ("Web", "wasm32-unknown-unknown", "build/pmj_client_cross/web/"),
    ]
    pf_sys = platform.system()
    match pf_sys:
        case "Windows":
            platforms.extend(
                [
                    (
                        "Windows_Desktop-x86_64",
                        "x86_64-pc-windows-msvc",
                        "build/pmj_client_cross/windows-x86_64/",
                    ),
                    (
                        "Windows_Desktop-arm64",
                        "aarch64-pc-windows-msvc",
                        "build/pmj_client_cross/windows-arm64/",
                    ),
                ]
            )
        case "Linux":
            platforms.extend(
                [
                    (
                        "Linux-x86_64",
                        "x86_64-unknown-linux-musl",
                        "build/pmj_client_cross/linux-x86_64",
                    ),
                    (
                        "Linux-arm64",
                        "aarch64-unknown-linux-musl",
                        "build/pmj_client_cross/linux-arm64",
                    ),
                ]
            )
        case _:
            print(f"platform not in [Linux, Windows], is {pf_sys}, not support!")
            sys.exit(1)
    for ex_preset_name, target_name, ex_dir_path in platforms:
        print("=> Installing rust target ...")
        print("--- info")
        print("target: " + target_name)
        print("end-of info ---")
        target_inst_cmd = ["rustup", "target", "add", target_name]
        util.run_cmd(target_inst_cmd)
        print("=> Compiling pmj_client_cross_lib ...")
        compile_cmd = [
            "cargo",
            "build",
            "--release",
            f"--target={target_name}",
        ]
        util.run_cmd(compile_cmd)
        print("=> Exporting godot ...")
        print("--- info")
        print("export_name: " + ex_preset_name)
        print("export_to: " + ex_dir_path)
        print("end-of info ---")
        export_command = [
            "godot",
            "--headless",
            "--verbose",
            "--export-release",
            ex_preset_name,
            ex_dir_path,
        ]
        util.run_cmd(export_command)


if __name__ == "__main__":
    main()
