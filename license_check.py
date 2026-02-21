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
    command = [
        "addlicense",
        "-check",
        "-f",
        "addlicense.template",
        "-ignore",
        ".git/**",
        "-ignore",
        "**/.venv/**",
        "-ignore",
        "dist/**",
        "-ignore",
        "pkg/**",
        "-ignore",
        "target/**",
        "-ignore",
        "build/**",
        "-ignore",
        "**/__pycache__/**",
        "-ignore",
        "**/*.lock",
        "-ignore",
        ".python-version",
        "-ignore",
        "**/*.png",
        "-ignore",
        "**/*.kra",
        "-ignore",
        "**/*.ttf",
        "-ignore",
        "assets/",
        "-ignore",
        "**/*.json",
        "-ignore",
        "ThirdPartyLicense-Rust.html",
        "-ignore",
        "ThirdPartyLicense-Python.html",
        "-ignore",
        "ThirdPartyLicense.html",
        "-ignore",
        "src/licenses_rust.rs",
        "-ignore",
        "src/licenses.rs",
        "-ignore",
        "src/licenses_python.rs",
        "-ignore",
        "**/*.icon",
        "-ignore",
        "**/*.sh",
        ".",
    ]
    print(" ".join(command))
    print("-" * 10)
    subprocess.run(
        command,
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
        timeout=180,
    )


main()
