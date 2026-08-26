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

"""`positive_mahjong` script util"""

import os
import subprocess
import sys
import tomllib
from typing import Literal

from colorama import Back, Fore, Style
from positive_tool import verify


def run_cmd(command: list[str], cwd: str | None = None) -> tuple[int, str]:
    verify.ArgType("command", command, list[str]).check_value_type()
    verify.ArgType("cwd", cwd, str | None).check_value_type()
    print(
        f"{Fore.CYAN}Running command:{Fore.RESET} {Back.LIGHTBLACK_EX}{' '.join(command)}{Back.RESET}"
    )
    process = subprocess.run(
        command,
        timeout=60 * 10,
        stderr=subprocess.PIPE,
        stdout=subprocess.PIPE,
        cwd=cwd,
    )
    if process.returncode == 0:
        print(
            f"{Style.DIM}=>{Style.NORMAL} {Fore.GREEN}Process fnished sucessful.{Fore.RESET}"
        )
    else:
        print(
            f"{Style.DIM}=>{Style.NORMAL} {Fore.RED}Process exited with non-zero code{Fore.RESET} {Style.DIM}({process.returncode}){Style.NORMAL}!"
        )
        for name, data in [
            ("stdout", process.stdout.decode()),
            ("stderr", process.stderr.decode()),
        ]:
            print(f"{Style.DIM}---{Style.NORMAL} {name}")
            for line in data.split("\n"):
                print(
                    f" {Style.DIM}{Fore.LIGHTBLACK_EX}|{Fore.RESET}{Style.NORMAL} {line}"
                )
            print(f"{Style.DIM}---{Style.NORMAL} end-of {name}")
        sys.exit(1)
    return (process.returncode, process.stdout.decode())


def get_commit_info():
    """return (commit_sha, commit_time)"""
    (_returncode, stdout) = run_cmd(["git", "rev-parse", "HEAD"])
    commit_sha = stdout.replace("\n", "")
    (_returncode, commit_time) = run_cmd(
        ["git", "log", "-1", "--format='%cd'", "--date=iso"]
    )
    return (commit_sha, commit_time)


def fix_path(*p: str) -> str:
    return os.path.join(os.path.dirname(os.path.dirname(__file__)), *p)


def get_version(workspace_type: Literal["workspace", "package"] = "workspace") -> str:
    verify.ArgType(
        "workspace_type", workspace_type, Literal["workspace", "package"]
    ).check_value_type()
    with open(
        fix_path("Cargo.toml"),
        "rb",
    ) as f:
        data = tomllib.load(f)
    match workspace_type:
        case "package":
            version = str(data["package"]["version"])
        case "workspace":
            version = str(data["workspace"]["package"]["version"])
    return version


if __name__ == "__main__":
    import typer

    app = typer.Typer()

    @app.command()
    def main(command: Literal["get_version"]):
        match command:
            case "get_version":
                print(get_version())
