use std::{env, fs, io::{Read, Write}, path};
use zip;

const INCLUDE_FILES: [&str;2] = ["pmj-client-desktop", "pmj-server-desktop"];
#[cfg(target_os = "windows")]
const EXE_SUFFIX: &str = ".exe";
#[cfg(not(target_os = "windows"))]
const EXE_SUFFIX: &str = "";

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
