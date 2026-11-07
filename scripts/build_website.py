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

import json
import os
import shutil
from typing import Literal

import mistune
import pydantic
import requests
from colorama import Back, Fore

import util

IGNORED_PATHS: list[str] = [
    ".git",
    "__pycache__",
    "__pypy_cache__",
    ".ruff_cache",
    ".venv",
    util.fix_path("website_build", "docs"),
    util.fix_path("website_build", "files", "raw"),
    util.fix_path("website_build", "files", "tree"),
]
SKIP_PREV_ARTIFACTS: bool = True
GH_PAGES_BASE_URL: str = "https://tw0hank0.github.io/positive_mahjong/"
ARTIFACTS_LIST_FILE: str = "releases-artifacts-list.json"
TYPE_HINT_RUST_TARGETS = Literal[
    "aarch64-pc-windows-msvc",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "aarch64-unknown-linux-gnu",
]
TYPE_HINT_RELEASE_ARTIFACTS = dict[
    TYPE_HINT_RUST_TARGETS,
    str,
]
TYPE_HINT_ARTIFACTS_LIST = dict[
    str,
    dict[
        Literal[
            "is_released", "release_artifacts", "ci_test_artifacts", "release_name"
        ],
        bool | dict[str, str] | dict[str, dict] | str | TYPE_HINT_RELEASE_ARTIFACTS,
    ],
]
RUST_TARGETS: list[TYPE_HINT_RUST_TARGETS] = [
    "aarch64-pc-windows-msvc",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "aarch64-unknown-linux-gnu",
]
FILES_PAGE_TEMPLATE = """\
<!doctype html>
<html lang="zh-TW">
    <head>
        <meta charset="UTF-8" />
        <meta
            name="viewport"
            content="width=device-width, initial-scale=1.0"
        />
        <meta
            name="description"
            content="positive_mahjong project website"
        />
        <title>positive_mahjong —— 檔案</title>
        <link rel="stylesheet" href="../style.css" />
        <link rel="icon" href="../icon.svg" />
        <link rel="shortcut icon" href="../icon.png" />
    </head>
    <body>
        <header>
            <nav>{{$VAR_NAV$}}</nav>
        </header>
        <section class="content">
            {{$DY_VAR_FILES_SUMMARY$}}
        </section>
    </body>
</html>
"""
DOWNLOAD_PAGE_TEMPLATE = """<!doctype html>
<!--
 SPDX-License-Identifier: AGPL-3.0-only
 著作權所有 (C) 2026 TW0hank0

 本檔案屬於 positive_mahjong 專案的一部分。
 專案儲存庫：https://gitlab.com/TW0hank0/positive_mahjong

 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
 第 3 版（僅此版本）重新發佈及/或修改本程式。

 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。

 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
 如果沒有，請參見 <https://www.gnu.org/licenses/>。
-->

<html lang="zh-TW">
    <head>
        <meta charset="UTF-8" />
        <meta
            name="viewport"
            content="width=device-width, initial-scale=1.0"
        />
        <meta
            name="description"
            content="positive_mahjong project website"
        />
        <title>positive_mahjong —— 下載</title>
        <link rel="stylesheet" href="style.css" />
        <link rel="icon" href="icon.svg" />
        <link rel="shortcut icon" href="icon.png" />
    </head>
    <body>
        <header>
            <nav>{{$VAR_NAV$}}</nav>
        </header>
        <section class="content">
            <h2>穩定版 v{{$DY_VAR_VERSION$}}</h2>
            {{$DY_VAR_DOWNLOAD_LINKS$}}
        </section>
    </body>
</html>
"""


class VarStore:
    __slots__ = ("prefix", "suffix", "vars")

    def __init__(self, prefix: str = "{{$", suffix: str = "$}}"):
        self.vars: dict[str, str] = {}
        self.prefix = prefix
        self.suffix = suffix

    def set_var(self, key: str, value: str):
        self.vars[key] = value

    def replace(self, content: str) -> str:
        replaced_content = content
        for key, value in self.vars.items():
            mixed_var_key = self.prefix + key + self.suffix
            if mixed_var_key in replaced_content:
                replaced_content = replaced_content.replace(mixed_var_key, value)
        return replaced_content


class ArtifactList(pydantic.BaseModel):
    is_released: bool
    release_name: str
    release_artifacts: TYPE_HINT_RELEASE_ARTIFACTS = {}
    ci_tests: dict[str, TYPE_HINT_RELEASE_ARTIFACTS] = {}


def main():
    website_root_path = util.fix_path("website")
    nav_template_path = os.path.join(website_root_path, "nav.html.template")
    with open(nav_template_path, "r", encoding="utf-8") as f:
        nav_template_content = f.read()
    build_root = util.fix_path("website_build")
    if os.path.exists(build_root) is True:
        if os.path.isdir(build_root) is True:
            shutil.rmtree(build_root)
        elif os.path.isfile(build_root) is True:
            os.remove(build_root)
        else:
            print("build_root already exists and is not file or dir!")
    shutil.copytree(website_root_path, build_root, ignore=copytree_ignore)
    if os.path.exists(util.fix_path("artifacts")) is False:
        os.mkdir(util.fix_path("artifacts"))
    artifacts_list()
    os.mkdir(os.path.join(build_root, "files"))
    build_files_page(os.path.join(build_root, "files"))
    if SKIP_PREV_ARTIFACTS is False:
        build_download_page(build_root)
    process_dir(
        dir_path=build_root,
        ignored=IGNORED_PATHS,
        nav_template=nav_template_content,
        website_root_path=build_root,
    )
    copy_website_dep(build_root)


def build_download_page(build_root: str):
    dy_vars = {}
    dy_vars["VERSION"] = util.get_version()
    with open(util.fix_path("artifacts", ARTIFACTS_LIST_FILE), "r") as f:
        i_artifacts_list: dict[str, ArtifactList | dict] = json.load(f)
    artifacts_list = {}
    for iprev_key, iprev_value in i_artifacts_list.items():
        artifacts_list[iprev_key] = ArtifactList.model_validate_json(
            json.dumps(iprev_value)
        )
    print(f"{artifacts_list=}")
    latest_stable_version_num = next(iter(artifacts_list.keys()))
    for version_num in artifacts_list:
        if int(version_num) > int(latest_stable_version_num):
            latest_stable_version_num = version_num
    while True:
        if artifacts_list[latest_stable_version_num].is_released is True:
            break
        else:
            latest_stable_version_num = str(int(latest_stable_version_num) - 1)
            if int(latest_stable_version_num) <= 0:
                return  # happend when no release artifacts gen yet
    dl_link_template = '<a href="/files/{}" class="dl-button">{}</a>'
    dl_links = []
    for target, filename in artifacts_list[
        latest_stable_version_num
    ].release_artifacts.items():
        dl_links.append(dl_link_template.format(filename, target))
    for dy_var in list(dy_vars.keys()):
        download_page = DOWNLOAD_PAGE_TEMPLATE.replace(
            "{{$DY_VAR_" + dy_var + "$}}", dy_vars[dy_var]
        )
    with open(os.path.join(build_root, "download.html"), "r") as f:
        f.write(download_page)


def artifacts_list() -> None:
    # download prev
    if SKIP_PREV_ARTIFACTS is False:
        print(f"requests.get: {GH_PAGES_BASE_URL}files/{ARTIFACTS_LIST_FILE}")
        prev_artifacts_list = requests.get(
            f"{GH_PAGES_BASE_URL}files/{ARTIFACTS_LIST_FILE}"
        )
        prev_artifacts_list.raise_for_status()
        i_prev_list: dict[str, ArtifactList | dict] = prev_artifacts_list.json()
        prev_list = {}
        for iprev_key, iprev_value in i_prev_list.items():
            prev_list[iprev_key] = ArtifactList.model_validate_json(
                json.dumps(iprev_value)
            )
        print("prepared prev_list")
        print(f"prev_list={prev_list}")
        for arelease in prev_list.values():
            if arelease.is_released is True:
                arelease_artifcats: dict = arelease.release_artifacts
                for ara_filename in arelease_artifcats.values():
                    print(f"requests.get({GH_PAGES_BASE_URL}files/{ara_filename})")
                    ara_file_resp = requests.get(
                        f"{GH_PAGES_BASE_URL}files/{ara_filename}"
                    )
                    ara_file_resp.raise_for_status()
                    with open(util.fix_path("artifacts", "history", ara_filename)) as f:
                        shutil.copyfileobj(ara_file_resp.raw, f)
            arelease_ci_artifacts = arelease.ci_tests
            for aca_id in arelease_ci_artifacts.values():
                for aca_filename in aca_id.values():
                    ara_file_resp = requests.get(
                        f"{GH_PAGES_BASE_URL}files/{aca_filename}"
                    )
                    ara_file_resp.raise_for_status()
                    with open(
                        util.fix_path("artifacts", "history", "ci_test", ara_filename)
                    ) as f:
                        shutil.copyfileobj(ara_file_resp.raw, f)
    # gen new list
    version = util.get_version()
    if SKIP_PREV_ARTIFACTS is True:
        new_list = {
            "1": ArtifactList(
                is_released=False,
                release_name=version,
                ci_tests={},
                release_artifacts={},
            ),
        }
    else:
        new_list = prev_list
    # prepare file list
    files_list: TYPE_HINT_RELEASE_ARTIFACTS = {}
    for file in util.list_files(
        util.fix_path("artifacts"),
        ignores=[
            util.fix_path("artifacts", "history"),
            util.fix_path("artifacts", ARTIFACTS_LIST_FILE),
        ],
    ):
        for target in RUST_TARGETS:
            if target in file:
                files_list[target] = os.path.relpath(file, util.fix_path("artifacts"))
                break
        else:
            print(f"unknown target, file: {file}")
    lastest_num = max(list(new_list.keys()))
    if new_list[lastest_num].is_released is True:
        new_list[str(int(lastest_num) + 1)] = ArtifactList(
            is_released=False,
            release_name=version,
            ci_tests={},
            release_artifacts={},
        )
        lastest_num = str(int(lastest_num) + 1)
    commit_info = util.get_commit_info()
    if commit_info.is_release is False:
        ci_test_id = f"{commit_info.time.replace(' ', '_')}::{commit_info.sha}"
        new_list[lastest_num].ci_tests[ci_test_id] = files_list
    else:
        new_list[lastest_num].is_released = True
        new_list[lastest_num].release_artifacts = files_list
    print(f"new_list={new_list}")
    new_list_json = {}
    for key, value in new_list.items():
        new_list_json[key] = json.loads(value.model_dump_json(indent=4))
    with open(
        util.fix_path("artifacts", ARTIFACTS_LIST_FILE), "w", encoding="utf-8"
    ) as f:
        json.dump(new_list_json, f, indent=4)


def copytree_ignore(src: str, names: list[str], /) -> list[str]:
    ignored_names = []
    for name in names:
        full_name = os.path.join(src, name)
        if (
            name in IGNORED_PATHS
            or full_name in IGNORED_PATHS
            or os.path.relpath(full_name, util.fix_path()) in IGNORED_PATHS
        ):
            ignored_names.append(name)
    del full_name
    return ignored_names


class FilesPageIncludeFile:
    def __init__(self, path: str, rename: str | None = None):
        self.abs_path = os.path.abspath(path)
        self.path_rel_project = os.path.relpath(os.path.abspath(path), util.fix_path())
        self.rename = rename

    def raw_name(self) -> str:
        """like: path/to/file.txt -> path-_-to-file.txt"""
        return (
            "-_-".join(os.path.dirname(self.path_rel_project).split(os.path.sep))
            + (
                "-"
                if self.path_rel_project != os.path.basename(self.path_rel_project)
                else ""
            )
            + os.path.basename(
                self.path_rel_project if self.rename is None else self.rename
            )
        )


def build_files_page(dir_path: str):
    include_files: list[FilesPageIncludeFile] = [
        FilesPageIncludeFile(util.fix_path("LICENSE.txt")),
        FilesPageIncludeFile(util.fix_path("README.md")),
        FilesPageIncludeFile(util.fix_path("TODO.md")),
        FilesPageIncludeFile(
            util.fix_path(
                "CHANGELOG.md",
            )
        ),
        FilesPageIncludeFile(
            util.fix_path(
                "ROADMAP.md",
            )
        ),
        FilesPageIncludeFile(
            util.fix_path(
                "auto_generated",
                "ThirdPartyLicense-Rust.html",
            )
        ),
        FilesPageIncludeFile(
            util.fix_path(
                "auto_generated",
                "ThirdPartyLicense-Rust.json",
            )
        ),
        FilesPageIncludeFile(
            util.fix_path(
                "auto_generated",
                "ThirdPartyLicense-Rust.md",
            )
        ),
    ]
    include_files.extend(
        [FilesPageIncludeFile(f) for f in util.list_files(util.fix_path("LICENSES"))]
    )
    if os.path.exists(util.fix_path("artifacts")) is True:
        include_files.extend(
            [
                FilesPageIncludeFile(f)
                for f in util.list_files(
                    util.fix_path("artifacts"),
                    ignores=[util.IGNORE_GH_PAGES, util.IGNORE_GH_PAGES_WITH_ARTIFACTS],
                )
            ]
        )
    files_summary = ""
    os.mkdir(os.path.join(dir_path, "raw"))
    os.mkdir(os.path.join(dir_path, "tree"))
    for include_file in include_files:
        for cfrom, cto in [
            (
                include_file.abs_path,
                os.path.join(dir_path, "raw", include_file.raw_name()),
            ),
            (
                include_file.abs_path,
                os.path.join(dir_path, "tree", include_file.path_rel_project),
            ),
        ]:
            print(f"Copy: {cfrom=}, {cto=}")
            os.makedirs(os.path.dirname(cto), exist_ok=True)
            _ = shutil.copy2(cfrom, cto)
        files_summary = (
            files_summary
            + f"""
        <div class="dlable-file">
          <a href="./tree/{include_file.path_rel_project}" target="_blank" download>{os.path.basename(include_file.abs_path) if include_file.rename is None else include_file.rename}</a>
        </div>"""
        )
    replaced_template = FILES_PAGE_TEMPLATE.replace(
        "{{$DY_VAR_FILES_SUMMARY$}}", files_summary
    )
    with open(os.path.join(dir_path, "index.html"), "w", encoding="utf-8") as f:
        _ = f.write(replaced_template)


def process_dir(
    dir_path: str, ignored: list[str], nav_template: str, website_root_path: str
):
    for dir in os.listdir(dir_path):
        full_dir_path = os.path.join(dir_path, dir)
        if (
            dir in ignored
            or full_dir_path in ignored
            or os.path.relpath(full_dir_path, util.fix_path()) in ignored
        ):
            continue
        else:
            if os.path.isfile(full_dir_path) is True:
                if dir.endswith(".html") is True:
                    print(f"Found html file: {full_dir_path}")
                    with open(full_dir_path, "r", encoding="utf-8") as f:
                        orig_content = f.read()
                    new_content = replace_var(
                        full_dir_path, nav_template, website_root_path
                    )
                    if new_content != orig_content:
                        with open(full_dir_path, "w", encoding="utf-8") as f:
                            _ = f.write(new_content)
                        print(
                            f"{Fore.GREEN}Wrote html file:{Fore.RESET} {Back.LIGHTBLACK_EX}{full_dir_path}{Back.RESET}."
                        )
            elif os.path.isdir(full_dir_path) is True:
                process_dir(
                    dir_path=full_dir_path,
                    ignored=ignored,
                    nav_template=nav_template,
                    website_root_path=website_root_path,
                )
            else:
                print(f"???: isfile=False, isdir=False, {dir=}")


def replace_var(
    replace_html_path: str, nav_template: str, website_root_path: str
) -> str:
    template_vars = VarStore()
    template_vars.set_var(
        "VAR_ROOT_DIR",
        os.path.relpath(website_root_path, os.path.dirname(replace_html_path)),
    )
    replaced_template = template_vars.replace(nav_template)
    html_vars = VarStore()
    html_vars.set_var("VAR_NAV", replaced_template)
    with open(
        util.fix_path("LICENSE.txt"),
        "r",
        encoding="utf-8",
    ) as f:
        license_text = f.read()
    html_vars.set_var("VAR_LICENSE", license_text)
    html_fixed_contents = [
        (
            "<https://fsf.org/>",
            '<a href="https://fsf.org/" target="_blank">&lt;https://fsf.org/&gt;</a>',
        ),
        (
            """<one line to give the program's name and a brief idea of what it does.>
    Copyright (C) <year>  <name of author>""",
            """&lt;one line to give the program&apos;s name and a brief idea of what it does.&gt;
    Copyright (C) &lt;year&gt;  &lt;name of author&gt;""",
        ),
        (
            "<https://www.gnu.org/licenses/>",
            '<a href="https://www.gnu.org/licenses/" target="_blank">&lt;https://www.gnu.org/licenses/&gt;</a>',
        ),
    ]
    fixed_license = license_text
    for orig, replacement in html_fixed_contents:
        fixed_license = fixed_license.replace(orig, replacement)
    html_vars.set_var("VAR_FIXED_LICENSE", fixed_license)
    with open(
        util.fix_path(
            "auto_generated",
            "ThirdPartyLicense-Rust.md",
        ),
        "r",
        encoding="utf-8",
    ) as f:
        third_party_license = f.read()
    html_vars.set_var("VAR_THIRD_PARTY_LICENSE_RUST_MD", third_party_license)
    html_vars.set_var(
        "VAR_THIRD_PARTY_LICENSE_RUST_MD_TO_HTML",
        str(mistune.html(third_party_license)),
    )
    del third_party_license
    commit_info = util.get_commit_info()
    html_vars.set_var("VAR_COMMIT_SHA", commit_info.sha)
    html_vars.set_var("VAR_COMMIT_SHORT_SHA", commit_info.short_sha)
    html_vars.set_var("VAR_COMMIT_TIME", commit_info.time)
    html_vars.set_var("VAR_COMMIT_MSG", commit_info.msg)
    html_vars.set_var(
        "VAR_COMMIT_MSG_PREFER_HTML",
        str(mistune.html(commit_info.msg.replace("<", "\\<").replace(">", "\\>"))),
    )
    html_vars.set_var("VAR_COMMIT_COMMITTER_NAME", commit_info.committer_name)
    html_vars.set_var("VAR_COMMIT_COMMITTER_EMAIL", commit_info.committer_email)
    html_vars.set_var("VAR_COMMIT_AUTHOR_NAME", commit_info.author_name)
    html_vars.set_var("VAR_COMMIT_AUTHOR_EMAIL", commit_info.author_email)
    del commit_info
    # read and replace
    with open(replace_html_path, "r", encoding="utf-8") as f:
        html_file_content = f.read()
    new_html_content = html_vars.replace(html_file_content)
    return new_html_content


def copy_website_dep(build_root: str):
    _ = util.run_cmd(["mdbook", "build"], cwd=util.fix_path("docs"))
    if os.path.exists(os.path.join(build_root, "docs")) is True:
        shutil.rmtree(os.path.join(build_root, "docs"))
    _ = shutil.copytree(
        util.fix_path("docs", "book"),
        os.path.join(build_root, "docs"),
    )
    _ = shutil.copy2(
        util.fix_path("assets", "icon", "icon.png"),
        os.path.join(build_root, "icon.png"),
    )
    _ = shutil.copy2(
        util.fix_path("assets", "icon", "icon.svg"),
        os.path.join(build_root, "icon.svg"),
    )


if __name__ == "__main__":
    main()
