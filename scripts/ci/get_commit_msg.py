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

import subprocess


def main(repo_path: str = ".") -> str | None:
    """
    取得指定 Git 最新一次的 commit message。

    Args:
        repo_path (str): Git 倉庫的路徑，預設為當前目錄。

    Returns:
        Optional[str]: 最新的 commit message，若發生錯誤則回傳 None。
    """
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

    # 移除首尾空白字元（包含換行符）
    commit_msg: str = result.stdout
    return commit_msg if commit_msg else None


print(main())
