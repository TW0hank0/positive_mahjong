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
import subprocess
import sys

from colorama import Back, Fore, Style

import util


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
        # auto generated license info
        "**/ThirdPartyLicense-Rust.*",
        "**/ThirdPartyLicense-Python.*",
        "auto_generated/**",
        "**/*.icon",
        "**/*.ico",
        "docs/book/**",
        # for ci
        "**/rust-init.sh",
        # website
        "website/docs/**",
        "website_build/**",
        # `old_pmj_client` 包含第三方程式碼
        # [android-iced-example](https://github.com/ibaryshnikov/android-iced-example)
        "crates/old_pmj_client/src/android/**",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj_client_desktop/src/easing.rs",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj_client_desktop/src/circular.rs",
        "crates/old_slint_client/**",
        "supply-chain/**",
        "scripts/**/__init__.py",
    ]
    ignored: list[str] = []
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
    work_cwd = os.getcwd()
    project_root = os.path.relpath(os.path.dirname(os.path.dirname(__file__)))
    # print(Fore.CYAN + "--- Run command" + Fore.RESET)
    # print(
    #     Back.LIGHTBLACK_EX
    #     + " && ".join([f"cd {project_root}", " ".join(command), f"cd {work_cwd}"])
    #     + Back.RESET
    # )
    # print("-" * 10)
    returncode = util.run_cmd(command, cwd=os.path.dirname(os.path.dirname(__file__)))
    # process = subprocess.run(
    #     command,
    #     stdout=sys.stdout,
    #     stdin=sys.stdin,
    #     stderr=sys.stderr,
    #     timeout=180,
    #     cwd=os.path.dirname(os.path.dirname(__file__)),
    # )
    if returncode != 0:
        print(Style.DIM + ("-" * 10) + Style.NORMAL)
        print(f"{Fore.RED}Something Wrong!{Fore.RESET}")
        fix_command = [
            "addlicense",
            "-f",
            os.path.join(
                "templates",
                "addlicense.template",
            ),
        ]
        fix_command.extend(ignored.copy())
        fix_command.append(".")
        print(Fore.CYAN + "--- Fix cmmand" + Fore.RESET)
        print(
            Back.LIGHTBLACK_EX
            + " && ".join(
                [f"cd {project_root}", " ".join(fix_command), f"cd {work_cwd}"]
            )
            + Back.RESET
        )
        print(Style.DIM + "End of command ---" + Style.RESET_ALL)
        sys.exit(1)
    else:
        print("Check Finish.")


if __name__ == "__main__":
    main()
