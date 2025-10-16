use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;


// 提示：须与 build.gradle.kts 和 Makefile 中的值保持一致
const FIXED_VERSION_CODE: u32 = 12128;
const BASE_VERSION_NAME: &str = "1.0.5-47";

fn get_git_short_hash() -> String {
    // 尝试执行 git 命令
    let result = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();

    match result {
        // 如果成功执行
        Ok(output) if output.status.success() => {
            let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if hash.is_empty() {
                "dev".to_string()
            } else {
                hash
            }
        }
        // 如果执行失败
        _ => "dev".to_string(),
    }
}

fn main() {
    let version_code = FIXED_VERSION_CODE;
    let git_hash = get_git_short_hash();
    let version_name = format!("{}-{}", BASE_VERSION_NAME, git_hash);
    // 打印信息
    println!("cargo:rustc-env=FINAL_VERSION_NAME={}", version_name);
    println!("cargo:rustc-env=FINAL_VERSION_CODE={}", version_code);

    let out_dir = env::var("OUT_DIR").expect("Failed to get $OUT_DIR");
    let out_dir = Path::new(&out_dir);
    File::create(out_dir.join("VERSION_CODE"))
        .expect("Failed to create VERSION_CODE")
        .write_all(version_code.to_string().as_bytes())
        .expect("Failed to write VERSION_CODE");

    File::create(out_dir.join("VERSION_NAME"))
        .expect("Failed to create VERSION_NAME")
        .write_all(version_name.as_bytes())
        .expect("Failed to write VERSION_NAME");
}
