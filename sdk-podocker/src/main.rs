use std::env;
use std::io::Result;
use std::process::{exit, Command, Stdio};

fn main() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let code = if args.iter().any(|item| item == "images" || item == "image") {
        handle_image(args)?
    } else if args.iter().any(|item| item == "start") {
        start_and_connect_ssh(args)?
    } else {
        delegate(args)?
    };
    exit(code);
}

fn handle_image(args: Vec<String>) -> Result<i32> {
    let output = Command::new("podman")
        .args(args)
        .stderr(Stdio::inherit())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let images = stdout.replace("localhost/", "");
    print!("{}", images);
    Ok(output.status.code().unwrap_or(0))
}

fn delegate(args: Vec<String>) -> Result<i32> {
    let exit = Command::new("podman").args(args).status()?;
    Ok(exit.code().unwrap_or(0))
}

const ADD_GROUP: &[&str] = &[
    "exec",
    "-d",
    "sailfish-os-build-engine",
    "usermod",
    "-a",
    "-G",
    "root",
    "mersdk",
];
const START_SSH: &[&str] = &[
    "exec",
    "-d",
    "sailfish-os-build-engine",
    "/usr/sbin/sshd",
    "-D",
    "-f",
    "/etc/ssh/sshd_config_engine",
];

fn start_and_connect_ssh(args: Vec<String>) -> Result<i32> {
    let status = delegate(args)?;
    Command::new("podman").args(ADD_GROUP).status()?;
    Command::new("podman").args(START_SSH).status()?;

    if status != 0 {
        Ok(status)
    } else {
        Ok(0)
    }
}
