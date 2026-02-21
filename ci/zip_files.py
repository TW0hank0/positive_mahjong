# SPDX-License-Identifier: AGPL-3.0-only
# 著作權所有 (C) 2026 TW0hank0
#
# 本檔案屬於 positive_toolbox 專案的一部分。
# 專案儲存庫：https://github.com/TW0hank0/positive_toolbox
#
# 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
# 第 3 版（僅此版本）重新發佈及/或修改本程式。
#
# 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
# 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
#
# 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
# 如果沒有，請參見 <https://www.gnu.org/licenses/>。

import tomllib
import zipfile
import os
import platform


def main():
    info_file = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), "Cargo.toml"
    )
    with open(info_file, "rb") as f:
        project_info = tomllib.load(f)
    version = project_info["package"]["version"]
    #
    include_files = []
    target_path = os.path.join(
        os.path.dirname(os.path.dirname(__file__)),
        "target",
        "release",
    )
    for file in os.listdir(target_path):
        full_file_path = os.path.join(target_path, file)
        if os.path.isfile(full_file_path) is True:
            match platform.system():
                case "Linux":
                    if len(file.split(".")) == 1:
                        include_files.append(full_file_path)
                case "Windows":
                    if (file.split(".")[1] == "exe") and (
                        len(file.split(".")) > 1
                    ):
                        include_files.append(full_file_path)
    #
    launcher_path = os.path.abspath(
        os.path.join(
            os.path.dirname(os.path.dirname(__file__)),
            "dist",
            "ptb_launcher",
        )
    )
    match platform.system():
        case "Linux":
            pf = "linux"
        case "Windows":
            pf = "windows"
            launcher_path = launcher_path + ".exe"
        case _:
            pf = "unknown"
    include_files.append(launcher_path)
    zip_file_name = f"positive_toolbox_v{version}_{pf}.zip"
    with zipfile.ZipFile(
        os.path.join(
            os.path.dirname(os.path.dirname(__file__)),
            zip_file_name,
        ),
        mode="w",
        compression=zipfile.ZIP_DEFLATED,
    ) as zipf:
        for file in include_files:
            zipf.write(file, arcname=os.path.basename(file))
    print(zip_file_name)


if __name__ == "__main__":
    main()
