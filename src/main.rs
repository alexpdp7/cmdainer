fn main() {
    let user_dirs = directories::UserDirs::new().unwrap();
    let home = user_dirs.home_dir().to_str().unwrap();
    let image = "busybox";
    let command = "touch";
    std::process::Command::new("docker")
        .arg("run")
        .arg("-it")
        .arg("--rm")
        .arg("-v")
        .arg(format!("{}:{}", home, home))
        .arg("-e")
        .arg(format!("HOME={}", home))
        .arg("-w")
        .arg(std::env::current_dir().unwrap())
        .arg("-u")
        .arg(format!(
            "{}:{}",
            users::get_current_uid(),
            users::get_current_gid()
        ))
        .arg(image)
        .arg(command)
        .args(std::env::args().skip(1))
        .status()
        .unwrap();
}
