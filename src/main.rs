use std::process::Command;

fn main() {
  launch_lxc("fedora/36", "fedora-container");
}

fn template(cm: &str, args: Vec<&str>, _err_message: &str) {
   let cmd = Command::new(cm).args(&args).output().unwrap_or_else(|e| {
        panic!("{}", e)
   });

   let result = String::from_utf8_lossy(&cmd.stdout);

   print!("{}", result);
}

pub fn lxd_init() {
   template("lxd", vec!["init"], "Try to initialize lxd was failed");
}

pub fn get_lxc_version() {
   template("lxc", vec!["--version"], "Try of get lxc was failed");
}

pub fn get_my_lxc() {
   template("lxc", vec!["list"], "Try of get lxc was failed");
}

pub fn get_my_lxc_images() {
   template("lxc", vec!["image", "list"], "Try of get lxc was failed");
}

pub fn get_lxc_images() {
   template("lxc", vec!["image", "list", "images:"], "Try of get lxc images was failed");
}

pub fn search_lxc_image(image: &str) {
   template("lxc", vec!["image", "list", "images:", &image], "Try of get some lxc image was failed");
}

pub fn get_image_info(image: &str) {
   template("lxc", vec!["image", "info", &image], "Try of getting image information was failed");
}

pub fn get_image_show(image: &str) {
    template("lxc", vec!["image", "show", &image], "Try of getting image information was failed");
}

pub fn del_image(image: &str) {
   template("lxc", vec!["image", "delete", &image], "Try of delete image was failed");
}

pub fn launch_lxc(image: &str, container: &str) {
   template("lxc", vec!["launch", "images:", &image, &container], "Try of launching container was failed");
}

pub fn start_lxc(container: &str) {
   template("lxc", vec!["start", &container], "Try of starting lxc container was failed");
}

pub fn stop_lxc(container: &str) {
   template("lxc", vec!["stop", &container], "Try of stopping lxc container was failed");
}

pub fn del_lxc(container: &str) {
   template("lxc", vec!["image", "delete", &container], "Failed to delete linux container");
}

pub fn rename_lxc(container: &str, new_name: &str) {
   template("lxc", vec!["move", &container, &new_name], "Failed to rename {&container#?}");
}

pub fn restart_lxc(container: &str) {
   template("lxc", vec!["restart", &container], "Failed to restart container");
}

pub fn copy_lxc(container: &str, to_container: &str) {
   template("lxc", vec!["copy", &container, &to_container], "Failed to copy from 1 to 2");
}

pub fn get_lxc_config(container: &str) {
   template("lxc", vec!["config", "show", &container], "Failed to get lxc container configuration");
}

pub fn push_file_in_lxc(file_path: &str, container_path: &str) {
   template("lxc", vec!["file", "push", &file_path, &container_path], "Failed to push files into container");
}

pub fn pull_file_from_lxc(container_path: &str, file_path: &str) {
   template("lxc", vec!["file", "pull", &container_path, &file_path], "Failed to pull files from container to current path");
}
