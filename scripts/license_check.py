# SPDX-License-Identifier: AGPL-3.0-only
# 著作權所有 (C) 2026 TW0hank0
#
# 本檔案屬於 positive_mahjong 專案的一部分。
# 專案儲存庫：https://github.com/TW0hank0/positive_mahjong
#
# 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
# 第 3 版（僅此版本）重新發佈及/或修改本程式。
#
# 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
# 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
#
# 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
# 如果沒有，請參見 <https://www.gnu.org/licenses/>。

import subprocess
import sys


def main():
    ignore_dir = [
        "**/.git/**",
        "**/.venv/**",
        "dist/**",
        "pkg/**",
        "target/**",
        "build/**",
        "**/__pycache__/**",
        "**/*.lock",
        "/.python-version",
        "**/*.png",
        "**/*.kra",
        "**/*.ttf",
        "**/*.otf",
        "assets/**",
        "**/*.json",
        "ThirdPartyLicense-Rust.*",
        "ThirdPartyLicense-Python.*",
        "auto_generated/**",
        "**/*.icon",
        "**/*.ico",
        "**/*.sh",
        "**/*.bat",
    ]
    ignored = []
    for dir in ignore_dir:
        ignored.extend(["-ignore", dir])
    command = [
        "addlicense",
        "-check",
        "-f",
        "templates/addlicense.template",
    ]
    command.extend(ignored.copy())
    command.append(".")
    print("Run Command:", " ".join(command))
    print("-" * 10)
    process = subprocess.run(
        command,
        # check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
        timeout=60,
    )
    print("-" * 10)
    if process.returncode != 0:
        print("Something Wrong!")
        fix_command = [
            "addlicense",
            "-f",
            "templates/addlicense.template",
        ]
        fix_command.extend(ignored.copy())
        fix_command.append(".")
        print("Fix Command:", " ".join(fix_command))
    else:
        print("Check Finish.")


if __name__ == "__main__":
    main()
