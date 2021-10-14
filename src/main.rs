use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Tries to make it easy to run commands from Docker images.")]
enum CmdockerArgs {
    Wrapper {
        wrapper: String,
        args: Vec<String>,
    },
    AddWrapper {
        name: String,
        path: String,
        image: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Command {
    path: String,
    image: String,
}

#[derive(Serialize, Deserialize, Default)]
struct CmdockerConfig {
    commands: std::collections::HashMap<String, Command>,
}

fn main() {
    let config: CmdockerConfig = confy::load("cmdocker").unwrap();
    let arg0_str = std::env::args().next().unwrap();
    let arg0 = std::path::Path::new(&arg0_str)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let not_wrapper = arg0.ends_with("cmdocker") || arg0.ends_with("cmdocker.exe");
    if !not_wrapper {
        std::process::exit(run_wrapper(
            config,
            arg0.to_string(),
            std::env::args().skip(1).collect(),
        ));
    }
    std::process::exit(match CmdockerArgs::from_args() {
        CmdockerArgs::AddWrapper { name, path, image } => add_wrapper(config, name, path, image),
        CmdockerArgs::Wrapper { wrapper, args } => run_wrapper(config, wrapper, args),
    });
}

fn add_wrapper(config: CmdockerConfig, name: String, path: String, image: String) -> i32 {
    let mut config = config;
    config
        .commands
        .insert(name.clone(), Command { path, image });
    let current_exe = std::env::current_exe().unwrap();
    let mut wrapper_path = current_exe.clone();
    wrapper_path.set_file_name(if cfg!(not(windows)) {
        name
    } else {
        format!("{}.exe", name)
    });
    println!(
        "Creating {:?} as symlink to {:?}",
        wrapper_path, current_exe
    );
    create_link(current_exe, wrapper_path).unwrap();
    confy::store("cmdocker", config).unwrap();
    0
}

#[cfg(not(windows))]
fn create_link(from: std::path::PathBuf, to: std::path::PathBuf) -> Result<(), std::io::Error> {
    std::os::unix::fs::symlink(from, to)
}

#[cfg(windows)]
fn create_link(from: std::path::PathBuf, to: std::path::PathBuf) -> Result<(), std::io::Error> {
    std::fs::copy(from, to).map(|_| ())
}

#[cfg(not(windows))]
fn get_homes() -> (String, String) {
    let user_dirs = directories::UserDirs::new().unwrap();
    let home = user_dirs.home_dir().to_str().unwrap();
    (home.to_string(), home.to_string())
}

#[cfg(windows)]
fn get_homes() -> (String, String) {
    let user_dirs = directories::UserDirs::new().unwrap();
    let home = user_dirs.home_dir().to_str().unwrap();
    (
        home.to_string().replace("\\", "/"),
        "/home/user".to_string(),
    )
}

#[cfg(not(windows))]
fn get_cwd() -> String {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[cfg(windows)]
fn get_cwd() -> String {
    let user_dirs = directories::UserDirs::new().unwrap();
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .replace(user_dirs.home_dir().to_str().unwrap(), "/home/user")
        .replace("\\", "/")
}

fn run_wrapper(config: CmdockerConfig, wrapper: String, args: std::vec::Vec<String>) -> i32 {
    let (home, home_target) = get_homes();
    let command = config
        .commands
        .get(&if cfg!(not(windows)) {
            wrapper
        } else {
            wrapper.replace(".exe", "")
        })
        .unwrap();
    let image = &command.image;
    let path = &command.path;
    let process = &mut std::process::Command::new("docker");
    process
        .arg("run")
        .arg(if atty::is(atty::Stream::Stdin) {
            "-it"
        } else {
            "-i"
        })
        .arg("--rm")
        .arg("-v")
        .arg(format!("{}:{}", home, home_target))
        .arg("-e")
        .arg(format!("HOME={}", home_target))
        .arg("-w")
        .arg(get_cwd());
    if is_podman() {
        process.arg("--security-opt").arg("label=disable");
    } else {
        #[cfg(not(windows))]
        process.arg("-u").arg(format!(
            "{}:{}",
            users::get_current_uid(),
            users::get_current_gid()
        ));
    }
    process.arg(format!("--entrypoint={}", path));
    process.arg(image).args(args);
    process.status().unwrap().code().unwrap()
}

fn is_podman() -> bool {
    std::str::from_utf8(
        &std::process::Command::new("docker")
            .arg("--help")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .contains("podman")
}
