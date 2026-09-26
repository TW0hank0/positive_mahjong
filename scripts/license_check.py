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
import sys

from colorama import Back, Fore, Style

import util


def main():
    print(f"{Fore.LIGHTBLACK_EX}addlicense check{Fore.RESET}")
    addlicense_check()
    print(f"{Fore.LIGHTBLACK_EX}reuse check{Fore.RESET}")
    reuse_check()


def reuse_check():
    _ = util.run_cmd(["reuse", "lint"], cwd=util.fix_path())


def addlicense_check():
    ignore_dir = [
        "**/.git/**",
        "**/.venv/**",
        "dist/**",
        "pkg/**",
        "target/**",
        "build/**",
        "**/__pycache__/**",
        "**/*.lock",
        "**/.python-version",
        # "**/*.png",
        # "**/*.kra",
        # "**/*.ttf",
        # "**/*.otf",
        "assets/**",
        # "**/*.json",
        # auto generated license info
        "**/ThirdPartyLicense-Rust.*",
        "**/ThirdPartyLicense-Python.*",
        "auto_generated/**",
        # "**/*.icon",
        # "**/*.ico",
        "**/docs/book/**",
        # for ci
        "**/rustup-init.sh",
        # for ci
        "**/rustup-init.exe",
        "website_build/**",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj-client-desktop/src/easing.rs",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj-client-desktop/src/circular.rs",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj-client-desktop-base/src/easing.rs",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj-client-desktop-base/src/circular.rs",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj-desktop/src/easing.rs",
        # Author: [iced team](https://github.com/iced-rs/)
        # This file is from project [iced](https://github.com/iced-rs/iced/).
        "crates/pmj-desktop/src/circular.rs",
        "supply-chain/**",
        "scripts/pylock.toml",
    ]
    ignored: list[str] = []
    for dir in ignore_dir:
        ignored.extend(["-ignore", dir])
    command = [
        "addlicense",
        "-check",
        "-f",
        util.fix_path("templates", "addlicense.template"),
    ]
    command.extend(ignored.copy())
    command.append(".")
    (returncode, _stdout) = util.run_cmd(
        command, cwd=util.fix_path(), exit_on_failed=False
    )
    if returncode != 0:
        print(str(Style.DIM) + "-" * 10 + str(Style.NORMAL))
        print(f"{Fore.RED}Something Wrong!{Fore.RESET}")
        fix_command = [
            "addlicense",
            "-f",
            util.fix_path(
                "templates",
                "addlicense.template",
            ),
        ]
        fix_command.extend(ignored.copy())
        fix_command.append(".")
        cwd_now = os.getcwd()
        print(f"{Fore.CYAN}--- Fix command{Fore.RESET}")
        print(
            str(Back.LIGHTBLACK_EX)
            + f"cd {util.fix_path()}"
            + " && "
            + " ".join(fix_command)
            + " && "
            + f"cd {cwd_now}"
            + str(Back.RESET)
        )
        print(str(Style.DIM) + "End of command ---" + str(Style.RESET_ALL))
        sys.exit(1)
    else:
        print("Check Finish.")


if __name__ == "__main__":
    main()
