use clap::Parser;
use log::debug;
use serde::{Deserialize, Serialize};
use std::io::IsTerminal;

#[derive(Parser, Debug)]
#[clap(about = "Tries to make it easy to run commands from Docker images.")]
enum CmdainerArgs {
    Wrapper {
        wrapper: String,
        args: Vec<String>,
    },
    AddWrapper {
        name: String,
        path: String,
        image: String,
        arch: Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
struct Command {
    path: String,
    image: String,
    arch: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct CmdainerConfig {
    commands: std::collections::HashMap<String, Command>,
}

fn main() {
    env_logger::init();
    let config: CmdainerConfig = confy::load("cmdainer", "cmdainer").unwrap();
    let arg0_str = std::env::args().next().unwrap();
    let arg0 = std::path::Path::new(&arg0_str)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let not_wrapper = arg0.ends_with("cmdainer") || arg0.ends_with("cmdainer.exe");
    if !not_wrapper {
        std::process::exit(run_wrapper(
            &config,
            arg0.to_string(),
            std::env::args().skip(1).collect(),
        ));
    }
    std::process::exit(match CmdainerArgs::parse() {
        CmdainerArgs::AddWrapper {
            name,
            path,
            image,
            arch,
        } => add_wrapper(config, name, path, image, arch),
        CmdainerArgs::Wrapper { wrapper, args } => run_wrapper(&config, wrapper, args),
    });
}

fn add_wrapper(
    config: CmdainerConfig,
    name: String,
    path: String,
    image: String,
    arch: Option<String>,
) -> i32 {
    let mut config = config;
    config
        .commands
        .insert(name.clone(), Command { path, image, arch });
    let current_exe = std::env::current_exe().unwrap();
    let mut wrapper_path = current_exe.clone();
    wrapper_path.set_file_name(if cfg!(not(windows)) {
        name
    } else {
        format!("{name}.exe")
    });
    println!("Creating {wrapper_path:?} as symlink to {current_exe:?}");
    create_link(current_exe, wrapper_path).unwrap();
    confy::store("cmdainer", "cmdainer", config).unwrap();
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

fn run_wrapper(config: &CmdainerConfig, wrapper: String, args: std::vec::Vec<String>) -> i32 {
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
    let process = &mut std::process::Command::new(if is_podman() { "podman" } else { "docker" });
    process
        .arg("run")
        .arg(if std::io::stdin().is_terminal() {
            "-it"
        } else {
            "-i"
        })
        .arg("--rm")
        .arg("-v")
        .arg(format!("{home}:{home_target}"))
        .arg("-e")
        .arg(format!("HOME={home_target}"))
        .arg("-w")
        .arg(get_cwd());
    if is_podman() {
        process.arg("--security-opt").arg("label=disable");
        process.arg("--userns=keep-id");
    } else {
        #[cfg(not(windows))]
        process.arg("-u").arg(format!(
            "{}:{}",
            users::get_current_uid(),
            users::get_current_gid()
        ));
    }

    if let Some(arch) = &command.arch {
        process.arg("--arch");
        process.arg(arch);
    }

    #[cfg(target_os = "linux")]
    process.arg("--network=host");

    process.arg(format!("--entrypoint={path}"));
    process.arg(image).args(args);
    debug!("Running {:?}", process);
    process.status().unwrap().code().unwrap()
}

fn is_podman() -> bool {
    true
}
