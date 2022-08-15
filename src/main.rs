use std::process::Command;

fn main() {
    let new_command = Command::new("lxc").arg("--version").output().unwrap_or_else(|e| {
      panic!("Command had been failed {}", e);
    });

    let str = String::from_utf8_lossy(&new_command.stdout);

    print!("{}", str);
}
