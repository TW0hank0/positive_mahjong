// SPDX-License-Identifier: AGPL-3.0-only
// 版權所有 (C) 2026 TW0hank0
//
// 本檔案屬於 positive_mahjong 專案的一部分。
// 專案儲存庫：https://gitlab.com/TW0hank0/positive_mahjong
//
// 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
// 第 3 版（僅此版本）重新發佈及/或修改本程式。
//
// 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
// 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
//
// 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
// 如果沒有，請參見 <https://www.gnu.org/licenses/>。

use std::{env, fs, io::{Read, Write}, path, env::consts::EXE_SUFFIX};
use zip;

const INCLUDE_FILES: [&str;2] = ["pmj-client-desktop", "pmj-server-desktop"];

fn main() {
    println!("rerun-if-changed=**/crates/**");
    println!("rerun-if-changed=**/target/**");
    println!("rerun-if-changed=Cargo.toml");
    println!("rerun-if-changed=Cargo.lock");
    let file = fs::OpenOptions::new().create(true).write(true).read(true).open(path::PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("inst-stored.zip")).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let mut target_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    loop {
        if target_dir.file_name().unwrap() == "build"{
            target_dir=target_dir.parent().unwrap().to_path_buf();
            break;
        } else {
            target_dir=target_dir.parent().unwrap().to_path_buf();
        }
    }
    for file in INCLUDE_FILES {
        let fullpath = target_dir.join(format!("{}{}", file, EXE_SUFFIX));
        if fs::exists(
            fullpath.clone()).unwrap_or(false) {
                zip.start_file_from_path(fullpath.file_name().unwrap(), zip::write::FileOptions::DEFAULT.compression_method(zip::CompressionMethod::Stored)).unwrap();
                let mut f = fs::File::open(fullpath).unwrap();
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).ok();
                zip.write_all(&buffer).ok();
            } else {
                    println!("Skip file: {}", fullpath.display())
                }}
    zip.finish().unwrap();
    println!("Zip file wrote: {}", path::PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("inst-stored.zip").display());
}
