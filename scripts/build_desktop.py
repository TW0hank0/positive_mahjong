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

import os
import platform
import sys
import zipfile
from typing import Literal

import util

INCLUDE_FILES_MATCH_TYPE: Literal["exe_split", "inclue_all_files"] = "exe_split"


def main():
    print("-" * 10, "cargo build", "-" * 10)
    _ = util.run_cmd(
        [
            "cargo",
            "build",
            "--release",
            "--locked",
        ],
        cwd=util.fix_path(),
    )
    zip_desktop()


def zip_desktop():
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
    target_path = util.fix_path(
        "target",
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
                        if (file.split(".")[1] == "exe") and (len(file.split(".")) > 1):
                            include_files.append(full_file_path)
                    case _:
                        raise RuntimeError("Not support system!")
            else:
                print(f"unmatched type: {INCLUDE_FILES_MATCH_TYPE}", file=sys.stderr)
    pf = platform.system().lower()
    zip_file_name = util.fix_path(f"positive_mahjong-desktop-v{version}-{pf}.zip")
    with zipfile.ZipFile(
        zip_file_name,
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
                    "incorrect file arg type, not tuple and not str!", file=sys.stderr
                )
    print(zip_file_name)


if __name__ == "__main__":
    main()
