use std::os::unix::fs::PermissionsExt;

#[test]
fn smoke() {
    // create a temporary dir
    let temp_dir = tempdir::TempDir::new("cmdocker_tests").unwrap();
    let temp_path: std::path::PathBuf = temp_dir.path().to_path_buf();

    // copy cmdocker there
    let cmdocker = assert_cmd::cargo::cargo_bin("cmdocker");
    let tmp_cmdocker = temp_path.join("cmdocker");
    std::fs::copy(cmdocker, tmp_cmdocker.clone()).unwrap();

    // make it executable
    std::fs::File::open(tmp_cmdocker)
        .unwrap()
        .set_permissions(std::fs::Permissions::from_mode(0o700))
        .unwrap();

    // copy docker there
    let tmp_docker = temp_path.join("docker");
    std::fs::copy(which::which("docker").unwrap(), tmp_docker).unwrap();

    // check foo does not exist in temporary dir beforehand
    assert!(!temp_path.join("foo").exists());

    // create a touch wrapper
    assert_cmd::Command::new("cmdocker")
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
        .env("PATH", temp_path.clone())
        .env("HOME", temp_path.clone())
        .current_dir(temp_path.clone())
        .assert()
        .success();

    // check touch worked
    assert!(temp_path.join("foo").exists());
}
