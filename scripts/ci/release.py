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

import os
import subprocess
import sys
from typing import Optional


def get_latest_commit_message(repo_path: str = ".") -> Optional[str]:
    """
    取得指定 Git 最新一次的 commit message。

    Args:
        repo_path (str): Git 倉庫的路徑，預設為當前目錄。

    Returns:
        Optional[str]: 最新的 commit message，若發生錯誤則回傳 None。
    """
    try:
        # 執行 git log 指令
        # --format=%B: 取得完整的 commit message body
        # -n 1: 只取最新的一筆
        result: subprocess.CompletedProcess = subprocess.run(
            ["git", "log", "-n", "1", "--format=%B"],
            cwd=repo_path,
            capture_output=True,
            text=True,
            check=True,
            timeout=30,
        )

        # 移除首尾空白字元（包含換行符）
        commit_msg: str = result.stdout
        return commit_msg if commit_msg else None

    except FileNotFoundError:
        print(
            "錯誤：找不到 git 執行檔，請確認已安裝 Git 並加入環境變數。",
            file=sys.stderr,
        )
        return None
    except subprocess.CalledProcessError as e:
        print(f"錯誤：Git 指令執行失敗。返回碼: {e.returncode}", file=sys.stderr)
        print(f"stderr: {e.stderr}", file=sys.stderr)
        return None
    except subprocess.TimeoutExpired:
        print("錯誤：Git 指令執行超時。", file=sys.stderr)
        return None


def main():
    msg = get_latest_commit_message()
    if msg is None:
        raise RuntimeError("msg=None")
    else:
        if "release" in msg:
            print("Release:")
            import datetime

            import get_version

            version = get_version.main()
            # owner = "TW0hank0"
            repo = "positive_mahjong"
            tag = f"v{version}"
            title = f"{repo} v{version}"
            date = datetime.datetime.now().date()
            notes = f"v{version} released: {date.year}/{date.month}/{date.day}"
            create_release(tag, title, notes, is_prerelease=False, repo=repo)
            # upload_file(files=os.listdir("artifacts"), tag=tag, owner=owner, repo=repo)
        else:
            print("No release.")


def create_release(tag: str, title: str, notes: str, is_prerelease: bool, repo: str):
    command: list[str] = [
        "gh",
        "release",
        "create",
        tag,
        "--title",
        title,
        "--notes",
        notes,
        f'--repo="{repo}"',
    ]
    if is_prerelease is True:
        command.append("--prerelease")
    run_cmd(command)


def upload_file(files: list[str], tag: str, owner: str, repo: str):
    get_url_command = [
        "gh",
        "api",
        f"repos/{owner}/{repo}/releases/tags/{tag}",
        "--jq",
        ".upload_url",
    ]
    get_url_process = subprocess.run(
        get_url_command, stdout=subprocess.PIPE, text=True, encoding="utf-8"
    )
    upload_url_base = get_url_process.stdout
    print(f"upload_url_base={upload_url_base}")
    subprocess.run(["set", "-euo", "pipefail"])
    subprocess.run(["shopt", "-s", "globstar", "||", "true"])
    for file in files:
        print(f"uploading {file}")
        run_cmd(
            [
                "curl",
                "-X",
                "POST",
                "-H",
                "Authorization: token $GITHUB_TOKEN",
                "-H",
                "Content-Type: application/octet-stream",
                "--data-binary",
                f"@'{file}'",
                str(upload_url_base) + str(os.path.basename(file)),
            ]
        )


def run_cmd(cmd: list[str]):
    print(f"Run:{' '.join(cmd)}")
    subprocess.run(
        cmd, check=True, stderr=sys.stderr, stdout=sys.stdout, stdin=sys.stdin
    )


if __name__ == "__main__":
    main()
