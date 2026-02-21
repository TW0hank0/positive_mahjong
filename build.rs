// SPDX-License-Identifier: AGPL-3.0-only
// 著作權所有 (C) 2026 TW0hank0
//
// 本檔案屬於 positive_mahjong 專案的一部分。
// 專案儲存庫：https://github.com/TW0hank0/positive_mahjong
//
// 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
// 第 3 版（僅此版本）重新發佈及/或修改本程式。
//
// 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
// 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
//
// 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
// 如果沒有，請參見 <https://www.gnu.org/licenses/>。

use std::fs;
use std::path::Path;

use cargo_metadata::MetadataCommand;

use positive_tool_rs;

fn main() {
    // 告訴 Cargo 重新執行 build.rs 當 Cargo.lock 變動
    println!("cargo:rerun-if-changed=Cargo.lock");

    let out_dir = positive_tool_rs::pt::find_project_path(env!("CARGO_PKG_NAME"), None)
        .unwrap()
        .join("src");
    //let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("licenses_rust.rs");

    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to get cargo metadata");

    let mut licenses: Vec<LicenseEntry> = Vec::new();

    // 遍歷所有解析後的套件（包含 transitive）
    for pkg in &metadata.packages {
        // 跳過 workspace 內的本機套件（避免重複）
        if metadata.workspace_members.contains(&pkg.id) {
            continue;
        }

        let license = pkg.license.as_deref().unwrap_or("Unknown");
        let name = &pkg.name;
        let version = &pkg.version.to_string();
        let authors = &pkg.authors;

        licenses.push(LicenseEntry {
            name: name.clone(),
            version: version.clone(),
            license: license.to_string(),
            authors: authors.clone(),
        });
    }

    // 排序以確保輸出穩定
    licenses.sort_by(|a, b| a.name.cmp(&b.name));

    let content = generate_rust_code(&licenses);
    fs::write(dest_path, content).expect("Failed to write licenses.rs");
    //
    let status = std::process::Command::new("cargo-about")
        .args(vec![
            "generate",
            "--output-file",
            "ThirdPartyLicense-Rust.html",
            "about.hbs",
            "--threshold",
            "1.0",
        ])
        .status()
        .unwrap();
    if !status.success() {
        panic!("error: cargo-about")
    }
}

#[derive(Debug)]
struct LicenseEntry {
    name: String,
    version: String,
    license: String,
    authors: Vec<String>,
}

fn generate_rust_code(entries: &[LicenseEntry]) -> String {
    let mut code = String::from(
        "#[allow(dead_code)]\npub fn get_licenses() -> Vec<LicenseInfo> {\n  return vec![\n",
    );
    for entry in entries {
        let name = escape_str(&entry.name);
        let version = escape_str(&entry.version);
        let license = escape_str(&entry.license);
        let authors_str = entry
            .authors
            .iter()
            .map(|a| format!("\"{}\"", escape_str(a)))
            .collect::<Vec<_>>()
            .join(", ");

        code.push_str(&format!(
            "    LicenseInfo {{
        name: \"{}\",
        version: \"{}\",
        license: \"{}\",
        authors: vec![{}] 
    }},\n",
            name, version, license, authors_str
        ));
    }
    code.push_str("];\n}\n");

    code.push_str(
        "#[derive(Debug)]
pub struct LicenseInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub license: &'static str,
    pub authors: Vec<&'static str>,
}
\n",
    );

    code
}

fn escape_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
