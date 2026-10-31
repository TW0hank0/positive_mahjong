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

import logging
import os
import sys

import gitlab
from rich.logging import RichHandler

import util

PROJECT = "TW0hank0/positive_mahjong"
REQUIRED_APPROVER_USERNAME = "TW0hank0"
REQUIRED_APPROVER_ID = 35460034
WORK_DIR = os.path.expanduser(os.path.join("~", "pmj_mr_script_workdir"))

LOGGER = logging.getLogger("pmj-scripts.mr")
LOGGER.setLevel(logging.DEBUG)
LOGGER.addHandler(RichHandler(logging.DEBUG))


def upsert_note(mr_obj, current_user_id: int, new_body: str):
    """
    透過 User ID 比對是否已有當前 Token 帳號發出的留言。
    若存在則直接編輯更新 (Save)，不存在則建立新留言 (Create)。
    """
    notes = mr_obj.notes.list(get_all=True)
    LOGGER.debug(f"notes: {notes}")
    target_note = None
    for note in notes:
        if note.system is False:
            author_id = note.author.get("id") if isinstance(note.author, dict) else None
            if author_id is not None and author_id == current_user_id:
                target_note = note
                break
    if target_note is not None:
        target_note.body = new_body
        target_note.save()
        LOGGER.info(f"Updated existing note (Note ID: {target_note.id})")
    else:
        created_note = mr_obj.notes.create({"body": new_body})
        LOGGER.info(f"Created new note (Note ID: {created_note.id})")


def main():
    if os.path.exists(WORK_DIR) is False:
        os.mkdir(WORK_DIR)
    LOGGER.debug("start mr script")
    private_token = os.environ.get("GITLAB_TOKEN")
    if private_token is None:
        LOGGER.error("GITLAB_TOKEN environment variable is not set")
        sys.exit(1)

    util.run_cmd(
        ["git", "clone", f"https://gitlab.com/{PROJECT}.git", WORK_DIR],
        cwd=WORK_DIR,
    )
    gl = gitlab.Gitlab(private_token=private_token)
    gl.auth()
    if gl.user is None:
        LOGGER.error("gl.user is None")
        sys.exit(1)
    LOGGER.debug(gl.user.attributes)
    current_user_id = gl.user.attributes["id"]
    current_username = gl.user.attributes["username"]
    LOGGER.info(f"Authenticated as: {current_username} (User ID: {current_user_id})")

    try:
        project = gl.projects.get(PROJECT)
    except gitlab.exceptions.GitlabGetError as e:
        LOGGER.error(f"Failed to get project '{PROJECT}': {e}")
        sys.exit(1)
    LOGGER.debug(f"Got project {project.attributes}")
    open_mrs: list | None = project.mergerequests.list(state="opened", all=True)
    if open_mrs is None:
        LOGGER.info(f"No open MRs found in project '{PROJECT}'.")
        sys.exit(0)
    LOGGER.info(f"Found {len(open_mrs)} open MRs. Starting checks...")
    LOGGER.debug(f"Open MRs: {open_mrs}")
    for mr in open_mrs:
        mr_iid = mr.iid
        mr_title = mr.title
        LOGGER.debug(f"Checking mr !{mr_iid}: {mr_title}")
        mr_detail = project.mergerequests.get(mr_iid)
        approvals = mr_detail.approvals.get()
        LOGGER.debug(approvals)
        if approvals.approved is False:
            LOGGER.info("MR not approved yet, skip")
            continue
        approved_by_userid = []
        approved_by_username = []
        for approved_by_detail in approvals.approved_by:
            aby_id = approved_by_detail["user"]["id"]
            aby_username = approved_by_detail["user"]["username"]
            LOGGER.debug(f"Approved by {aby_username} (id: {aby_id})")
            approved_by_userid.append(aby_id)
            approved_by_username.append(aby_username)

        LOGGER.info(f"Approved by: {approved_by_username}")
        if (
            REQUIRED_APPROVER_ID not in approved_by_userid
            or REQUIRED_APPROVER_USERNAME not in approved_by_username
        ):
            LOGGER.info(
                f"MR !{mr_iid} is not approved by @{REQUIRED_APPROVER_USERNAME}, Skipping."
            )
            continue
        else:
            if (
                current_user_id in approved_by_userid
                or current_username in approved_by_username
            ):
                for approved_by_detail in approvals.approved_by:
                    if (
                        approved_by_detail["user"]["id"] == current_user_id
                        and approvals.updated_at >= approved_by_detail["approved_at"]
                    ):
                        mr_detail.unapprove()
                        break
                else:
                    LOGGER.info(
                        f"Already checked (already approved by {current_user_id})"
                    )
                    continue
            LOGGER.info(
                f"MR !{mr_iid} is approved by @{REQUIRED_APPROVER_USERNAME}. Fetching code..."
            )
            fetch_ref = f"refs/merge-requests/{mr_iid}/head"
            util.run_cmd(["git", "fetch", "origin", fetch_ref], cwd=WORK_DIR)
            util.run_cmd(["git", "checkout", "FETCH_HEAD"], cwd=WORK_DIR)
            LOGGER.info(f"Successfully checked out MR !{mr_iid}.")
            LOGGER.info("Running mr check...")
            check_result = check()
            if check_result is None:
                msg = f"#### ✅MR Check Passed (!{mr_iid})"
                LOGGER.info(f"Check SUCCESS: {msg}")
                mr_detail.approve()
                mr_detail.save()
                upsert_note(mr_detail, current_user_id, msg)
            else:
                msg = f"#### ❌MR Check Failed (!{mr_iid})"
                (command, returncode, stdout, stderr) = check_result
                msg += f"\n\nCommand `{command}` return code `{returncode}`."
                msg += f"\n\n<details><summary>stdout</summary>{stdout}</details>"
                msg += f"\n\n<details><summary>stderr</summary>{stderr}</details>"
                LOGGER.warning(f"Check FAILED: {msg}")
                upsert_note(mr_detail, current_user_id, msg)
    LOGGER.info("mr script finish")


def check() -> None | tuple[str, int, str, str]:
    commands = [
        (["cargo", "check", "--workspace"], "."),
        (["uv", "run", "license_check.py"], "scripts"),
        (["ty", "check", "."], "scripts"),
        (["ruff", "check", "."], "scripts"),
    ]
    for command, relpath in commands:
        (check_returncode, check_stdout, check_stderr) = util.run_cmd(
            command,
            cwd=os.path.abspath(os.path.join(WORK_DIR, relpath)),
            exit_on_failed=False,
        )
        if check_returncode != 0:
            return (" ".join(command), check_returncode, check_stdout, check_stderr)
    return None


if __name__ == "__main__":
    main()
