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

import json
import os
import shutil

import util


def main():
    print("-" * 10, "cargo-about", "-" * 10)
    autogen_path = util.fix_path("auto_generated")
    if os.path.exists(autogen_path) is False:
        os.mkdir(autogen_path)
    all_commands = [
        [
            "cargo",
            "about",
            "generate",
            "--output-file",
            os.path.join(autogen_path, "ThirdPartyLicense-Rust.html"),
            "--threshold",
            "1.0",
            util.fix_path("templates", "about_html.hbs"),
        ],
        [
            "cargo",
            "about",
            "generate",
            "--threshold",
            "1.0",
            "--output-file",
            os.path.join(autogen_path, "ThirdPartyLicense-Rust.json"),
            util.fix_path("templates", "about_json.hbs"),
        ],
        [
            "cargo",
            "about",
            "generate",
            "--output-file",
            os.path.join(autogen_path, "ThirdPartyLicense-Rust.md"),
            "--threshold",
            "1.0",
            util.fix_path("templates", "about_markdown.hbs"),
        ],
    ]
    for command in all_commands:
        _ = util.run_cmd(command, cwd=util.fix_path())
    print("Indenting json file...", end="")
    json_file_path = util.fix_path(
        "auto_generated",
        "ThirdPartyLicense-Rust.json",
    )
    with open(json_file_path, "r", encoding="utf-8") as f:
        json_data = json.load(f)
    with open(json_file_path, "w", encoding="utf-8") as f:
        json.dump(json_data, f, ensure_ascii=False, sort_keys=True, indent=4)
    print("Finish!")
    print("Copying ThirdPartyLicense.md ...", end="")
    _ = shutil.copyfile(
        util.fix_path(
            "auto_generated",
            "ThirdPartyLicense-Rust.md",
        ),
        util.fix_path(
            "docs",
            "src",
            "license",
            "ThirdPartyLicense-Rust.md",
        ),
    )
    print("Finish!")


if __name__ == "__main__":
    main()
