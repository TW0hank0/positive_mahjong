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
import shutil

import mistune
from colorama import Back, Fore, Style

import util

ignored_paths: list[str] = [
    ".git",
    "__pycache__",
    "__pypy_cache__",
    ".ruff_cache",
    ".venv",
]


def main():
    website_root_path = util.fix_path("website")
    nav_template_path = os.path.join(website_root_path, "nav.html.template")
    with open(nav_template_path, "r", encoding="utf-8") as f:
        nav_template_content = f.read()
    # with open(util.fix_path("website", ".gitignore"), "r", encoding="utf-8") as f:
    #     ignored_paths.extend(f.read().split("\n"))
    build_root = util.fix_path("website_build")
    if os.path.exists(build_root) is True:
        if os.path.isdir(build_root) is True:
            shutil.rmtree(build_root)
        elif os.path.isfile(build_root) is True:
            os.remove(build_root)
        else:
            print("build_root already exists and is not file or dir!")
    shutil.copytree(website_root_path, build_root, ignore=copytree_ignore)
    os.mkdir(os.path.join(build_root, "files"))
    build_files_dl(os.path.join(build_root, "files"))
    process_dir(
        dir_path=build_root,
        ignored=ignored_paths,
        nav_template=nav_template_content,
        website_root_path=build_root,
    )
    copy_website_dep(build_root)


def copytree_ignore(src: str, names: list[str], /) -> list[str]:
    ignored_names = []
    for name in names:
        if name in ignored_paths:
            ignored_names.append(name)
        else:
            full_name = os.path.join(src, name)
            rel_name = os.path.relpath(full_name, start=util.fix_path()).replace(
                "\\",
                "/",  # 處理 windows 路徑問題
            )
            if rel_name in ignored_paths:
                ignored_names.append(name)
    return ignored_names


def build_files_dl(dir_path: str):
    dlable_files: list[str | tuple[str, str]] = [
        util.fix_path("LICENSE"),
        util.fix_path("README.md"),
        util.fix_path("TODO.md"),
        util.fix_path(
            "CHANGELOG.md",
        ),
        util.fix_path(
            "ROADMAP.md",
        ),
        util.fix_path(
            "auto_generated",
            "ThirdPartyLicense-Rust.html",
        ),
        util.fix_path(
            "auto_generated",
            "ThirdPartyLicense-Rust.json",
        ),
        util.fix_path(
            "auto_generated",
            "ThirdPartyLicense-Rust.md",
        ),
        (
            util.fix_path(
                "assets",
                "Noto_Sans_TC",
                "OFL.txt",
            ),
            "Noto_Sans_TC_OFL.txt",
        ),
        (
            util.fix_path(
                "assets",
                "material_symbols",
                "LICENSE",
            ),
            "material_symbols_LICENSE",
        ),
    ]
    if os.path.exists(util.fix_path("artifacts")) is True:
        pre_dlable_files = util.list_files(util.fix_path("artifacts"))
        for file in pre_dlable_files:
            if os.path.basename(os.path.dirname(file)) != "github-pages":
                dlable_files.append(file)
            else:
                print(f"  {Fore.LIGHTBLACK_EX}ignored {file}")
    for license_file in util.list_files(util.fix_path("LICENSES")):
        dlable_files.append(
            (license_file, f"LICENSES_{os.path.basename(license_file)}")
        )
    files_summary_template = """\
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
    </html>\n"""
    summary_prepare = ""
    for dlfile in dlable_files:
        if type(dlfile) is str:
            file_path = dlfile
            new_name = os.path.basename(dlfile)
        elif type(dlfile) is tuple:
            file_path = dlfile[0]
            new_name = dlfile[1]
        else:
            raise RuntimeError("type(dlfile) is not (str, tuple)")
        print(
            f"{Style.DIM}{dlfile}{Style.NORMAL} -> {Style.DIM}{os.path.join(dir_path, new_name)}{Style.NORMAL}"
        )
        _ = shutil.copy2(file_path, os.path.join(dir_path, new_name))
        summary_prepare = (
            summary_prepare
            + f"""
        <div class="dlable-file">
          <a href="./{new_name}" target="_blank" download>{new_name}</a>
        </div>"""
        )
    files_summary = files_summary_template.replace(
        "{{$DY_VAR_FILES_SUMMARY$}}", summary_prepare
    )
    with open(os.path.join(dir_path, "index.html"), "w", encoding="utf-8") as f:
        _ = f.write(files_summary)


def process_dir(
    dir_path: str, ignored: list[str], nav_template: str, website_root_path: str
):
    for dir in os.listdir(dir_path):
        full_dir_path = os.path.join(dir_path, dir)
        if dir in ignored:
            continue
        else:
            if os.path.isfile(full_dir_path) is True:
                if dir.endswith(".html") is True:
                    print(
                        f"Found html file: {Back.LIGHTBLACK_EX}{full_dir_path}{Back.RESET}"
                    )
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
                print(f"???: isfile=False, isdir=False, dir={dir}")


def replace_var(
    replace_html_path: str, nav_template: str, website_root_path: str
) -> str:
    orig_work_dir = os.getcwd()
    template_vars: dict[str, str] = {}
    os.chdir(website_root_path)
    rel_path = os.path.relpath(
        website_root_path, start=os.path.dirname(replace_html_path)
    )
    template_vars["VAR_ROOT_DIR"] = rel_path
    new_template = nav_template
    for key in list(template_vars.keys()):
        key_fixed_name = "{{$" + key + "$}}"
        if key_fixed_name in new_template:
            new_template = new_template.replace(key_fixed_name, template_vars[key])
            print(f"{Style.DIM}Replaced `{key}` in template.{Style.NORMAL}")
    html_vars: dict[str, str] = {}
    html_vars["VAR_NAV"] = new_template
    with open(
        util.fix_path("LICENSE"),
        "r",
        encoding="utf-8",
    ) as f:
        html_vars["VAR_LICENSE"] = f.read()
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
    fixed_license = html_vars["VAR_LICENSE"]
    for orig, replacement in html_fixed_contents:
        fixed_license = fixed_license.replace(orig, replacement)
    html_vars["VAR_FIXED_LICENSE"] = fixed_license
    with open(
        util.fix_path(
            "auto_generated",
            "ThirdPartyLicense-Rust.md",
        ),
        "r",
        encoding="utf-8",
    ) as f:
        t = f.read()
    html_vars["VAR_THIRD_PARTY_LICENSE_RUST_MD"] = t
    html_vars["VAR_THIRD_PARTY_LICENSE_RUST_MD_TO_HTML"] = str(mistune.html(t))
    commit_info = util.get_commit_info()
    html_vars["VAR_COMMIT_SHA"] = commit_info.sha
    html_vars["VAR_COMMIT_SHORT_SHA"] = commit_info.short_sha
    html_vars["VAR_COMMIT_TIME"] = commit_info.time
    html_vars["VAR_COMMIT_MSG"] = commit_info.msg
    html_vars["VAR_COMMIT_MSG_PREFER_HTML"] = str(
        mistune.html(commit_info.msg.replace("<", "\\<").replace(">", "\\>"))
    )
    html_vars["VAR_COMMIT_COMMITTER_NAME"] = commit_info.committer_name
    html_vars["VAR_COMMIT_COMMITTER_EMAIL"] = commit_info.committer_email
    html_vars["VAR_COMMIT_AUTHOR_NAME"] = commit_info.author_name
    html_vars["VAR_COMMIT_AUTHOR_EMAIL"] = commit_info.author_email
    # read and replace
    with open(replace_html_path, "r", encoding="utf-8") as f:
        new_html_content = f.read()
    for key in list(html_vars.keys()):
        key_fixed_name = "{{$" + key + "$}}"
        if key_fixed_name in new_html_content:
            new_html_content = new_html_content.replace(key_fixed_name, html_vars[key])
            print(
                f"{Style.DIM}Replaced `{key}` in file: {replace_html_path}.{Style.NORMAL}"
            )
    os.chdir(orig_work_dir)
    return new_html_content


def copy_website_dep(build_root: str):
    orig_cwd = os.getcwd()
    os.chdir(util.fix_path("docs"))
    _ = util.run_cmd(["mdbook", "build"])
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
    os.chdir(orig_cwd)


if __name__ == "__main__":
    main()
