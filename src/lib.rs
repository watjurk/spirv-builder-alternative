use std::env;
use std::{path::PathBuf, process};

const TARGET_CRATE_NAME: &str = "target_crate";
pub fn dylib_path() -> PathBuf {
    let file_dir = file_dir_abs_path();

    let out_dir = file_dir.join("out");
    let out_dir_str = out_dir.to_str().unwrap();

    let mut cmd = process::Command::new("cargo");
    cmd.arg("build");

    cmd.args(&["-Z", "unstable-options"]);
    cmd.args(&["--out-dir", out_dir_str]);
    cmd.args(&["--release", "--quiet"]);

    let target_crate_dir = file_dir.join(TARGET_CRATE_NAME);
    cmd.current_dir(target_crate_dir.clone());

    cmd.stdout(process::Stdio::inherit());
    cmd.stderr(process::Stdio::inherit());
    cmd.spawn().unwrap().wait().unwrap();

    out_dir.join(format!(
        "{}{TARGET_CRATE_NAME}{}",
        env::consts::DLL_PREFIX,
        env::consts::DLL_SUFFIX
    ))
}

fn file_dir_abs_path() -> PathBuf {
    let mut file_path = PathBuf::from(file!());
    file_path.pop();

    let current_dir = env::current_dir().expect("unable to get current dir");
    current_dir.join(file_path)
}
