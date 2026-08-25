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

import datetime
import os
import subprocess
import sys

import gitlab
import requests
import typer

from ci import get_version
from util import run_cmd

OWNER = "TW0hank0"
REPO = "positive_mahjong"
app = typer.Typer()


def get_latest_commit_message(repo_path: str = ".") -> str | None:
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
        result = subprocess.run(
            ["git", "log", "-n", "1", "--format=%B"],
            cwd=repo_path,
            capture_output=True,
            text=True,
            check=True,
            timeout=30,
        )

        if result.stdout == "":
            return None
        else:
            return result.stdout

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


@app.command()
def main():
    msg: str | None = get_latest_commit_message()
    commit_sha = (
        subprocess.run(
            ["git", "rev-parse", "HEAD"], stdout=subprocess.PIPE, timeout=10, check=True
        )
        .stdout.decode()
        .replace("\n", "")
    )
    if msg is None:
        raise RuntimeError("msg=None")
    else:
        if "release pmj:" in msg.lower():
            print("Release PMJ:")

            version = get_version.main()
            repo = "TW0hank0/positive_mahjong"
            tag = f"v{version}"
            title = f"{repo} v{version}"
            date = datetime.datetime.now().date()
            notes = f"v{version} released: {date.year}/{date.month}/{date.day}"
            create_release_gh(
                tag, title, notes, is_prerelease=False, owner=OWNER, repo=REPO
            )
            upload_file_gh(
                files=os.listdir(
                    os.path.join(
                        os.path.dirname(os.path.dirname(__file__)), "artifacts"
                    )
                ),
                tag=tag,
                owner=OWNER,
                repo=REPO,
            )
            gl_token = os.environ.get("GITLAB_PAT_TOKEN")
            if gl_token is None:
                print("No gitlab token!", file=sys.stderr)
                sys.exit(1)
            else:
                _ = release_gitlab(
                    gitlab_pat=gl_token,
                    project_id=f"{OWNER}/{REPO}",
                    tag_name=tag,
                    release_name=title,
                    description=notes,
                )
            cb_token = os.environ.get("CODEBERG_PAT_TOKEN")
            if cb_token is None:
                print("No codeberg token!", file=sys.stderr)
                sys.exit(1)
            else:
                _ = codeberg_release(
                    token=cb_token,
                    owner=OWNER,
                    repo=REPO,
                    release_tag=tag,
                    release_notes=notes,
                    release_title=title,
                    release_files=os.listdir(
                        os.path.join(
                            os.path.dirname(os.path.dirname(__file__)), "artifacts"
                        )
                    ),
                )
        else:
            print("PreRelease:", end="")
            version = get_version.main()
            date = datetime.datetime.now().date()
            time = datetime.datetime.now().time()
            tag = f"ci-v{version}+{date.month}_{date.day}-{commit_sha}"
            title = f"PreRelease v{version}+{date.month}/{date.day}+{time.hour}:{time.minute}"
            notes = f"""這是使用 Github Action 制作的測試版
#### 版本
{version}
#### 時間
{date.year}/{date.month}/{date.day} {time.hour}:{time.minute}:{time.second}
#### 提交訊息（{commit_sha}）
{msg}
"""
            create_release_gh(
                tag, title, notes, is_prerelease=True, owner=OWNER, repo=REPO
            )
            upload_file_gh(
                files=os.listdir(
                    os.path.join(
                        os.path.dirname(os.path.dirname(__file__)), "artifacts"
                    )
                ),
                tag=tag,
                owner=OWNER,
                repo=REPO,
            )


def create_release_gh(
    tag: str, title: str, notes: str, is_prerelease: bool, owner: str, repo: str
):
    command: list[str] = [
        "gh",
        "release",
        "create",
        tag,
        "--title",
        title,
        "--notes",
        notes,
        f"--repo={owner}/{repo}",
    ]
    if is_prerelease is True:
        command.append("--prerelease")
    run_cmd(command)


def upload_file_gh(files: list[str], tag: str, owner: str, repo: str):
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


def codeberg_release(
    token: str,
    owner: str,
    repo: str,
    release_tag: str,
    release_title: str,
    release_notes: str,
    release_files: list[str] | None = None,
    draft: bool = False,
    prerelease: bool = False,
) -> dict:
    """
    在 Codeberg 建立 Release 並上傳附檔。

    :param token: Codeberg API Personal Access Token
    :param owner: 儲存庫擁有者
    :param repo: 儲存庫名稱
    :param release_tag: 標籤名稱
    :param release_title: Release 標題
    :param release_notes: Release 內容說明 (Markdown)
    :param release_files: 欲上傳的檔案路徑列表 (str)
    :param draft: 是否設為草稿
    :param prerelease: 是否設為預覽版
    :return: 建立好的 Release API 回傳資料 (dict)
    """
    base_url = f"https://codeberg.org/api/v1/repos/{owner}/{repo}/releases"
    headers = {
        "Authorization": f"token {token}",
        "Accept": "application/json",
    }
    # 建立 Release
    payload = {
        "tag_name": release_tag,
        "title": release_title,
        "body": release_notes,
        "draft": draft,
        "prerelease": prerelease,
    }

    response = requests.post(base_url, json=payload, headers=headers, timeout=30)
    response.raise_for_status()
    release_data = response.json()

    # 2. 上傳 Release 附件
    if release_files:
        release_id = str(release_data["id"])
        upload_url = f"{base_url}/{release_id}/attachments"

        for file_path in release_files:
            if not os.path.isfile(file_path):
                raise FileNotFoundError(f"找不到檔案: {file_path}")

            filename = os.path.basename(file_path)

            with open(file_path, "rb") as f:
                files = {"attachment": (filename, f)}
                upload_resp = requests.post(
                    upload_url, headers=headers, files=files, timeout=60
                )
                upload_resp.raise_for_status()

    return release_data


def release_gitlab(
    gitlab_pat: str,
    project_id: int | str,
    tag_name: str,
    release_name: str,
    description: str,
    file_paths: list[str] | None = None,
    ref: str = "master",
    gitlab_url: str = "https://gitlab.com",
) -> dict:
    """建立 GitLab Release

    :param gitlab_url: GitLab 伺服器網址 (例如 'https://gitlab.com')
    :param gitlab_pat: GitLab Personal Access Token
    :param project_id: 專案 ID 或 'owner/repo' 路徑
    :param tag_name: Git Tag 名稱 (如 'v1.0.0')
    :param release_name: Release 標題
    :param description: Release 內文 (支援 Markdown)
    :param file_paths: 要上傳的本地檔案路徑列表
    :param ref: 若 Tag 不存在時，指定建 Tag 的來源分支
    :return: 建立好的 Release 資訊字典
    """
    # 1. 初始化連線 (使用 gitlab_pat)
    gl = gitlab.Gitlab(gitlab_url, private_token=gitlab_pat)
    gl.auth()
    project = gl.projects.get(project_id)

    # 2. 上傳檔案並收集 assets 連結
    assets_links: list[dict[str, str]] = []
    if file_paths is not None:
        for file in file_paths:
            if os.path.exists(file) is False:
                raise FileNotFoundError(f"找不到檔案: {file}")
            else:
                # 呼叫 GitLab Project Uploads API
                upload_result = project.upload(os.path.basename(file), filepath=file)
                # 組合 Release Asset Link 格式
                assets_links.append(
                    {
                        "name": os.path.basename(file),
                        "url": f"{gitlab_url.rstrip('/')}{upload_result['full_path']}",
                        "filepath": f"/{os.path.basename(file)}",
                        "link_type": "package",
                    }
                )

        # 組裝 Release 資料並建立
        release_data: dict[str, str | dict[str, list[dict[str, str]]]] = {
            "name": release_name,
            "tag_name": tag_name,
            "description": description,
            "ref": ref,
        }

        if len(assets_links) > 0:
            release_data["assets"] = {"links": assets_links}
        release = project.releases.create(release_data)
        return release.attributes


if __name__ == "__main__":
    main()
