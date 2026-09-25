# SPDX-License-Identifier: AGPL-3.0-only
# 版權所有 (C) 2026 TW0hank0
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

import os
import platform
import sys
import zipfile
from typing import Literal

from colorama import Fore

import build_license
import util

INCLUDE_FILES_MATCH_TYPE: Literal["exe_split", "inclue_all_files"] = "exe_split"


def main():
    build_info: list[tuple[str, str | None | list[str]]] = []
    match platform.system():
        case "Linux":
            if platform.machine().lower() in ["amd64", "x86_64"]:
                build_info.extend(
                    [
                        ("x86_64-unknown-linux-gnu", None),
                        (
                            "x86_64-unknown-linux-musl",
                            ["pmj-server-desktop", "pmj-client-desktop"],
                        ),
                    ]
                )
            elif platform.machine().lower() in ["arm64", "aarch64"]:
                build_info.extend(
                    [
                        ("aarch64-unknown-linux-gnu", None),
                        (
                            "aarch64-unknown-linux-musl",
                            ["pmj-server-desktop", "pmj-client-desktop"],
                        ),
                    ]
                )
            else:
                print(
                    f"{Fore.RED}Unsupport machine(arch): {platform.machine()}!{Fore.RESET}"
                )
        case "Windows":
            if platform.machine().lower() in ["amd64", "x86_64"]:
                build_info.append(("x86_64-pc-windows-msvc", None))
            elif platform.machine().lower() in ["arm64", "aarch64"]:
                build_info.append(("aarch64-pc-windows-msvc", None))
            else:
                print(f"{Fore.RED}Unsupport machine(arch)!{Fore.RESET}")
        case _:
            print(f"{Fore.RED}Unsupport system!{Fore.RESET}")
    print(f"target: {build_info}")
    targets = []
    for target, pkg in build_info:
        targets.append(target)
        _ = util.run_cmd(["rustup", "target", "add", target])
        print(f"{Fore.CYAN}Building release for target {target}...{Fore.RESET}", end="")
        cmd = ["cargo", "build", "--release", "--locked", "--target", target]
        if pkg is None:
            cmd.append("--workspace")
            print()
        elif type(pkg) is str:
            cmd.extend(("--package", pkg))
            print(f"{Fore.LIGHTBLACK_EX} (package: {pkg}){Fore.RESET}")
        elif type(pkg) is list:
            for p in pkg:
                cmd.extend(("--package", p))
            print(f"{Fore.LIGHTBLACK_EX} (packages: {' ,'.join(pkg)}){Fore.RESET}")
        else:
            print("Error: pkg not None, str or list!")
        _ = util.run_cmd(
            cmd,
            cwd=util.fix_path(),
            timeout=60 * 75,  # 75分鐘
            stream=True,
        )
    build_license.main()
    zip_desktop(targets)


def zip_desktop(targets: list[str]):
    version = util.get_version()
    include_files: list[str | tuple[str, str]] = [
        util.fix_path("README.md"),
        util.fix_path("LICENSE"),
        util.fix_path(
            "auto_generated",
            "ThirdPartyLicense-Rust.html",
        ),
        util.fix_path(
            "auto_generated",
            "ThirdPartyLicense-Rust.json",
        ),
        util.fix_path(
            "auto_generated",
            "ThirdPartyLicense-Rust.md",
        ),
        (
            util.fix_path(
                "assets",
                "Noto_Sans_TC",
                "OFL.txt",
            ),
            "Noto_Sans_TC_OFL.txt",
        ),
        (
            util.fix_path(
                "assets",
                "material_symbols",
                "LICENSE",
            ),
            "material_symbols_LICENSE",
        ),
    ]
    for target in targets:
        target_path = util.fix_path(
            "target",
            target,
            "release",
        )
        for file in os.listdir(target_path):
            full_file_path = os.path.join(target_path, file)
            if os.path.isfile(full_file_path) is True:
                if INCLUDE_FILES_MATCH_TYPE == "inclue_all_files":
                    include_files.append(full_file_path)
                elif INCLUDE_FILES_MATCH_TYPE == "exe_split":
                    match platform.system():
                        case "Linux":
                            if len(file.split(".")) == 1:
                                include_files.append(full_file_path)
                        case "Windows":
                            if (file.split(".")[1] == "exe") and (
                                len(file.split(".")) > 1
                            ):
                                include_files.append(full_file_path)
                        case _:
                            raise RuntimeError("Not support system!")
                else:
                    print(
                        f"unmatched type: {INCLUDE_FILES_MATCH_TYPE}", file=sys.stderr
                    )
        artifacts_path = util.fix_path("artifacts")
        if os.path.exists(artifacts_path) is False:
            os.mkdir(artifacts_path)
        zip_file_path = os.path.join(
            artifacts_path, f"positive_mahjong-desktop-v{version}-{target}.zip"
        )
        with zipfile.ZipFile(
            zip_file_path,
            mode="w",
            compression=zipfile.ZIP_DEFLATED,
        ) as zipf:
            for file in include_files:
                if type(file) is tuple:
                    zipf.write(file[0], arcname=file[1])
                elif type(file) is str:
                    zipf.write(file, arcname=os.path.basename(file))
                else:
                    print(
                        "incorrect file arg type, not tuple and not str!",
                        file=sys.stderr,
                    )
        print(zip_file_path)


if __name__ == "__main__":
    main()
