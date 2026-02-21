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

import subprocess
import sys

# import json
import os
import time


def main():
    start_time = time.time()
    print("-" * 10, "cargo-about", "-" * 10)
    subprocess.run(
        [
            "cargo-about",
            "generate",
            "--output-file",
            "ThirdPartyLicense-Rust.html",
            "about.hbs",
            "--threshold",
            "1.0",
        ],
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
    )
    #     print("-" * 10, "pip-licenses", "-" * 10)
    #     subprocess.run(
    #         [
    #             "uv",
    #             "run",
    #             "pip-licenses",
    #             "--format=html",
    #             "--output-file",
    #             "ThirdPartyLicense-Python.html",
    #             "--from=mixed",
    #             "--with-urls",
    #         ],
    #         check=True,
    #         stdout=sys.stdout,
    #         stdin=sys.stdin,
    #         stderr=sys.stderr,
    #     )
    #     print("-" * 10, "licenses_python.rs", "-" * 10)
    #     piplicense_output = subprocess.run(
    #         [
    #             "uv",
    #             "run",
    #             "pip-licenses",
    #             "--format=json",
    #             "--from=mixed",
    #             "--with-urls",
    #         ],
    #         check=True,
    #         capture_output=True,
    #     )
    #     piplicense_data = json.loads(piplicense_output.stdout)
    #     license_file_path = os.path.join(
    #         os.path.dirname(__file__), "src", "licenses_python.rs"
    #     )
    #     piplicense_conversioned_data = """
    # #[derive(Debug)]
    # pub struct LicenseInfo {
    #     pub name: &'static str,
    #     pub version: &'static str,
    #     pub license: &'static str,
    #     pub authors: Vec<&'static str>,
    # }

    # pub fn get_licenses() -> Vec<LicenseInfo> { \nreturn vec!["""
    #     for license_data in piplicense_data:
    #         tmp_str = f"""
    # LicenseInfo {{
    #     name: \"{license_data["Name"]}\",
    #     version: \"{license_data["Version"]}\",
    #     license: \"{license_data["License"]}\",
    #     authors: vec![],
    # }},"""
    #         piplicense_conversioned_data = (
    #             piplicense_conversioned_data + "\n" + tmp_str
    #         )
    #     piplicense_conversioned_data = (
    #         piplicense_conversioned_data + "];}"
    #     )
    #     with open(license_file_path, "w", encoding="utf-8") as f:
    #         f.write(piplicense_conversioned_data)
    #     print("-" * 10, "ptb_launcher", "-" * 10)
    #     subprocess.run(
    #         ["uv", "run", "pyinstaller", "ptb_launcher.spec"],
    #         check=True,
    #         stdout=sys.stdout,
    #         stdin=sys.stdin,
    #         stderr=sys.stderr,
    #     )
    print("-" * 10, "cargo build", "-" * 10)
    subprocess.run(
        ["cargo", "build", "--release"],
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
    )
    print("-" * 10, "zip file", "-" * 10)
    subprocess.run(
        ["uv", "run", os.path.join("ci", "zip_files.py")],
        check=True,
        stdout=sys.stdout,
        stdin=sys.stdin,
        stderr=sys.stderr,
    )
    print("-" * 20)
    print("finish in", time.time() - start_time)


if __name__ == "__main__":
    main()
