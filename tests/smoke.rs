#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;

#[test]
fn smoke() {
    // create a temporary dir
    let temp_dir = tempfile::TempDir::new().unwrap();
    let temp_path: std::path::PathBuf = temp_dir.path().to_path_buf();

    // copy cmdainer there
    let cmdainer = assert_cmd::cargo::cargo_bin("cmdainer");
    let tmp_cmdainer = temp_path.join("cmdainer");
    std::fs::copy(cmdainer, tmp_cmdainer.clone()).unwrap();

    // make it executable
    #[cfg(not(windows))]
    std::fs::File::open(tmp_cmdainer)
        .unwrap()
        .set_permissions(std::fs::Permissions::from_mode(0o700))
        .unwrap();

    // copy docker there
    let tmp_docker = temp_path.join("docker");
    std::fs::copy(which::which("docker").unwrap(), tmp_docker).unwrap();

    // check foo does not exist in temporary dir beforehand
    assert!(!temp_path.join("foo").exists());

    // create a touch wrapper
    assert_cmd::Command::new("cmdainer")
        .arg("add-wrapper")
        .arg("busy_touch")
        .arg("/bin/touch")
        .arg("busybox")
        .env_clear()
        .env("PATH", temp_path.clone())
        .env("HOME", temp_path.clone())
        .assert()
        .success();

    // run touch wrapper
    assert_cmd::Command::new("busy_touch")
        .arg("foo")
        .env_clear()
        .env(
            "PATH",
            format!(
                "{}:{}",
                temp_path.clone().to_str().unwrap(),
                std::env::var("PATH").unwrap()
            ),
        )
        .env("HOME", temp_path.clone())
        .current_dir(temp_path.clone())
        .assert()
        .success();

    // check touch worked
    assert!(temp_path.join("foo").exists());
}
