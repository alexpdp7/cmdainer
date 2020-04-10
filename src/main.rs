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
    let arg0 = std::path::Path::new(&arg0_str);
    let not_wrapper = arg0.ends_with("cmdocker");
    if !not_wrapper {
        std::process::exit(run_wrapper(
            config,
            arg0_str,
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
    config.commands.insert(name, Command { path, image });
    confy::store("cmdocker", config).unwrap();
    0
}

fn run_wrapper(config: CmdockerConfig, wrapper: String, args: std::vec::Vec<String>) -> i32 {
    let user_dirs = directories::UserDirs::new().unwrap();
    let home = user_dirs.home_dir().to_str().unwrap();
    let command = config.commands.get(&wrapper).unwrap();
    let image = &command.image;
    let path = &command.path;
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
        .arg(path)
        .args(args)
        .status()
        .unwrap()
        .code()
        .unwrap()
}
