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
import shutil
import sys
import zipfile
from typing import Literal
from warnings import deprecated

from colorama import Fore

import build_license
import util

util.init()

INCLUDE_FILES_MATCH_TYPE: Literal["exe_split", "inclue_all_files"] = "exe_split"
RUN_CMD_STREAM: bool = False
EXEC_EXT = ".exe" if os.name == "nt" else ""

build_env = os.environ.copy()
build_env["CARGO_BUILD_JOBS"] = "1"


class BuildTargetInfo:
    def __init__(self, target: str, packages: list[str] | str | None = None):
        self.target = target
        self.packages = packages

    def rustup_target(self):
        util.run_cmd(["rustup", "target", "add", self.target])

    def check(self):
        cmd = [
            "cargo",
            "-vv",
            "check",
            "--release",
            "--target",
            self.target,
        ]
        if self.packages is None:
            cmd.append("--workspace")
        elif type(self.packages) is str:
            cmd.extend(("--package", self.packages))
        else:
            for p in self.packages:
                cmd.extend(("--package", p))
        _ = util.run_cmd(
            cmd,
            stream=RUN_CMD_STREAM,
            timeout=60 * 15,
            env=build_env,
        )

    def build(self):
        cmd = [
            "cargo",
            "-vv",
            "build",
            "--release",
            "--locked",
            "--target",
            self.target,
        ]
        if self.packages is None:
            cmd.append("--workspace")
        elif type(self.packages) is str:
            cmd.extend(("--package", self.packages))
        else:
            for p in self.packages:
                cmd.extend(("--package", p))
        _ = util.run_cmd(cmd, env=build_env, stream=RUN_CMD_STREAM, timeout=60 * 45)

    def build_desktop_installer(self):
        commit_info = util.CommitInfo.get_from_git()
        if commit_info.is_release is False:
            version = util.get_version() + "+" + commit_info.short_sha
        else:
            version = util.get_version()
        _ = util.run_cmd(
            [
                "cargo",
                "-vv",
                "build",
                "--bin",
                "pmj-desktop-installer",
                "--target",
                self.target,
            ],
            timeout=60 * 15,  # 15 分鐘
            stream=RUN_CMD_STREAM,
            env=build_env,
        )
        target_path = util.fix_path(
            "target",
            self.target,
            "release",
        )
        desktop_installer_path = os.path.join(
            target_path, f"pmj-desktop-installer{EXEC_EXT}"
        )
        if (os.path.exists(desktop_installer_path) is True) and (
            os.path.isfile(desktop_installer_path) is True
        ):
            print(
                "Copied to: "
                + str(
                    shutil.copy2(
                        desktop_installer_path,
                        util.fix_path(
                            "artifacts",
                        ),
                    )
                )
            )
            new_desktop_installer_name = (
                f"pmj-desktop-installer-v{version}-{self.target}{EXEC_EXT}"
            )
            os.rename(
                desktop_installer_path,
                util.fix_path("artifacts", new_desktop_installer_name),
            )
            print(
                f"Renamed to {util.fix_path('artifacts', new_desktop_installer_name)}"
            )

    def create_zip(self):
        commit_info = util.CommitInfo.get_from_git()
        if commit_info.is_release is False:
            version = util.get_version() + "+" + commit_info.short_sha
        else:
            version = util.get_version()
        target_path = util.fix_path(
            "target",
            self.target,
            "release",
        )
        include_files = [
            util.fix_path("README.md"),
            util.fix_path("LICENSE.txt"),
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
            util.fix_path("LICENSES", "LicenseRef-Noto-Sans-TC.txt"),
            (
                util.fix_path(
                    "assets",
                    "material_symbols",
                    "LICENSE",
                ),
                "material_symbols_LICENSE",
            ),
        ]
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
                            if (len(file.split(".")) > 1) and (
                                file.split(".")[1] == "exe"
                            ):
                                include_files.append(full_file_path)
                        case _:
                            raise RuntimeError("Not support system!")
                else:
                    print(
                        f"unmatched type: {INCLUDE_FILES_MATCH_TYPE}",
                        file=sys.stderr,
                    )
        artifacts_path = util.fix_path("artifacts")
        if os.path.exists(artifacts_path) is False:
            os.mkdir(artifacts_path)
        zip_file_path = os.path.join(
            artifacts_path, f"positive_mahjong-desktop-v{version}-{self.target}.zip"
        )
        with zipfile.ZipFile(
            zip_file_path,
            mode="w",
            compression=zipfile.ZIP_DEFLATED,
        ) as zipf:
            for file in include_files:
                if type(file) is tuple and len(file) > 1:
                    zipf.write(file[0], arcname=file[1])
                elif (type(file) is str) or (type(file) is tuple and len(file) == 1):
                    zipf.write(file, arcname=os.path.basename(file))
                else:
                    print(
                        "incorrect file arg type, not tuple and not str!",
                        file=sys.stderr,
                    )
        print(f"{zip_file_path=}")


def main():
    build_target_infos = []
    match platform.system():
        case "Linux":
            if platform.machine().lower() in ["amd64", "x86_64"]:
                build_target_infos.append(BuildTargetInfo("x86_64-unknown-linux-gnu"))
                build_target_infos.append(
                    BuildTargetInfo(
                        "x86_64-unknown-linux-musl",
                        ["pmj-server-desktop", "pmj-client-desktop"],
                    )
                )
            elif platform.machine().lower() in ["arm64", "aarch64"]:
                build_target_infos.append(BuildTargetInfo("aarch64-unknown-linux-gnu"))
                build_target_infos.append(
                    BuildTargetInfo(
                        "aarch64-unknown-linux-musl",
                        ["pmj-server-desktop", "pmj-client-desktop"],
                    )
                )
            else:
                raise RuntimeError(
                    f"{Fore.RED}Unsupport machine arch: {platform.machine()}!{Fore.RESET}"
                )
        case "Windows":
            if platform.machine().lower() in ["amd64", "x86_64"]:
                build_target_infos.append(BuildTargetInfo("x86_64-pc-windows-msvc"))
            elif platform.machine().lower() in ["arm64", "aarch64"]:
                build_target_infos.append(BuildTargetInfo("aarch64-pc-windows-msvc"))
            else:
                raise RuntimeError(
                    f"{Fore.RED}Unsupport machine arch: {platform.machine()}!{Fore.RESET}"
                )
        case _:
            raise RuntimeError(f"{Fore.RED}Unsupport system!{Fore.RESET}")
    print(f"{build_target_infos=}")
    build_license.main()
    for build_info in build_target_infos:
        build_info.rustup_target()
        build_info.check()
        build_info.build()
        build_info.create_zip()
        build_info.build_desktop_installer()


@deprecated("replace with BuildTargetInfo.create_zip()")
def zip_desktop(targets: list[str]):
    commit_info = util.CommitInfo.get_from_git()
    if commit_info.is_release is False:
        version = util.get_version() + "+" + commit_info.short_sha
    else:
        version = util.get_version()
    include_files: list[str | tuple[str, str]] = [
        util.fix_path("README.md"),
        util.fix_path("LICENSE.txt"),
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
