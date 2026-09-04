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

import datetime
import json
import os
import subprocess
import sys
import time
import tomllib
from typing import Literal
from zoneinfo import ZoneInfo

from colorama import Back, Fore


def run_cmd(command: list[str], cwd: str | None = None) -> tuple[int, str]:
    print(
        f"{Fore.CYAN}Running command:{Fore.RESET} {Back.LIGHTBLACK_EX}{' '.join(command)}{Back.RESET}"
    )
    start_time = time.time()
    process = subprocess.run(
        command,
        timeout=60 * 25,
        stderr=subprocess.PIPE,
        stdout=subprocess.PIPE,
        cwd=cwd,
    )
    end_time = time.time()
    process_time = end_time - start_time
    if int(process_time) > 1:
        process_time_str = f"({process_time:.3f} secs)"
    else:
        process_time_str = ""
    if process.returncode == 0:
        print(
            f"{Fore.LIGHTBLACK_EX}=>{Fore.RESET} {Fore.GREEN}Process fnished sucessful.{Fore.RESET} {Fore.LIGHTBLACK_EX}{process_time_str}{Fore.RESET}"
        )
    else:
        print(
            f"{Fore.LIGHTBLACK_EX}=>{Fore.RESET} {Fore.RED}Process exited with non-zero code{Fore.RESET} {Fore.LIGHTBLACK_EX}({process.returncode}){Fore.RESET}! {Fore.LIGHTBLACK_EX}{process_time_str}{Fore.RESET}"
        )
        for name, data in [
            ("stdout", process.stdout.decode()),
            ("stderr", process.stderr.decode()),
        ]:
            if (data == "\n") or (data.strip() == ""):
                print(f"{name} does not have any content.")
            else:
                print(f"{Fore.LIGHTBLACK_EX}---{Fore.RESET} {name}")
                line_num = 1
                for line in data.split("\n"):
                    print(
                        f"{Fore.LIGHTBLACK_EX}{str(line_num).rjust(3)}|{Fore.RESET} {line}"
                    )
                    line_num += 1
                print(f"{Fore.LIGHTBLACK_EX}--- end-of {name}{Fore.RESET}")
        sys.exit(1)
    return (process.returncode, process.stdout.decode().rstrip("\n"))


def get_commit_info() -> CommitInfo:
    """return CommitInfo"""
    (_returncode, sha_stdout) = run_cmd(["git", "rev-parse", "HEAD"], cwd=fix_path())
    commit_sha = sha_stdout.replace("\n", "")
    (_returncode, ssha_stdout) = run_cmd(
        ["git", "rev-parse", "--short", "HEAD"], cwd=fix_path()
    )
    commit_short_sha = ssha_stdout.replace("\n", "")
    (_returncode, commit_time) = run_cmd(
        ["git", "log", "-1", "--format=%cd", "--date=iso"], cwd=fix_path()
    )
    (_returncode, commit_msg) = run_cmd(
        ["git", "log", "-1", "--format=%B"], cwd=fix_path()
    )
    (_, commit_committer_name) = run_cmd(["git", "log", "-1", "--format=%cn"])
    (_, commit_committer_email) = run_cmd(["git", "log", "-1", "--format=%ce"])
    (_, commit_author_name) = run_cmd(["git", "log", "-1", "--format=%an"])
    (_, commit_author_email) = run_cmd(["git", "log", "-1", "--format=%ae"])
    return CommitInfo(
        commit_sha,
        commit_short_sha,
        commit_time,
        commit_msg,
        commit_committer_name,
        commit_committer_email,
        commit_author_name,
        commit_author_email,
    )


class CommitInfo:
    __slots__: list[str] = [
        "sha",
        "short_sha",
        "time",
        "msg",
        "committer_name",
        "committer_email",
        "author_name",
        "author_email",
    ]
    sha: str
    short_sha: str
    time: str
    msg: str
    committer_name: str
    committer_email: str
    author_name: str
    author_email: str

    def __init__(
        self,
        commit_sha: str,
        commit_short_sha: str,
        commit_time: str,
        commit_msg: str,
        committer_name: str,
        committer_email: str,
        author_name: str,
        author_email: str,
    ) -> None:
        self.sha = commit_sha
        self.short_sha = commit_short_sha
        self.time = commit_time
        self.msg = commit_msg
        self.committer_name = committer_name
        self.committer_email = committer_email
        self.author_name = author_name
        self.author_email = author_email


def fix_path(*p: str) -> str:
    return os.path.abspath(os.path.join(os.path.dirname(os.path.dirname(__file__)), *p))


def get_version(
    how_get: Literal["file_workspace", "file_package", "cargo"] = "file_workspace",
) -> str:
    if how_get == "file_package" or how_get == "file_workspace":
        with open(
            fix_path("Cargo.toml"),
            "rb",
        ) as f:
            data = tomllib.load(f)
        if how_get == "file_package":
            version = str(data["package"]["version"])
        elif how_get == "file_workspace":
            version = str(data["workspace"]["package"]["version"])
    elif how_get == "cargo":
        (_returncode, stdout) = run_cmd(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"], cwd=fix_path()
        )
        metadata = json.loads(stdout)
        for pkg in metadata["packages"]:
            if pkg["name"] == "pmj_client_desktop":
                version = str(pkg["version"])
                break
        else:
            raise RuntimeError("no version found.")
    return version


def get_datetime():
    tz = ZoneInfo("Asia/Taipei")
    return datetime.datetime.now(tz).strftime("%Y-%m-%d_%H-%M-%S")


def list_files(path: str) -> list[str]:
    files: list[str] = []
    for file in os.listdir(path):
        file_path = os.path.join(path, file)
        if os.path.isfile(file_path) is True:
            files.append(file_path)
        elif os.path.isdir(file_path) is True:
            files.extend(list_files(file_path))
        else:
            print(f"???? not file not dir: {file_path}")
    return files


if __name__ == "__main__":
    import typer

    app = typer.Typer()

    @app.command()
    def main(
        command: Literal[
            "get_version", "get_datetime", "commit_sha", "commit_short_sha"
        ],
    ):
        match command:
            case "get_version":
                print(get_version())
            case "get_datetime":
                print(get_datetime())
            case "commit_sha":
                commit_info = get_commit_info()
                print(commit_info.sha)
            case "commit_short_sha":
                commit_info = get_commit_info()
                print(commit_info.short_sha)

    app()
