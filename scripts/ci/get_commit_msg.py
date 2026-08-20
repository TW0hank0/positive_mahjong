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
