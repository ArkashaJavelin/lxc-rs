use std::process::Command;
use std::borrow::Cow;

fn main() {
   del_snapshot("ubuntu-container", "first");
}

fn template(cm: &str, args: Vec<&str>, _err_message: &str) {
   let cmd = Command::new(cm).args(&args).output().unwrap_or_else(|e| {
        panic!("{}", e)
   });
   
   let result: Cow<'_, str> = String::from_utf8_lossy(&cmd.stdout);

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
   template("lxc", vec!["launch", "images", &image, &container], "Try of launching container was failed");
}

pub fn start_lxc(container: &str) {
   template("lxc", vec!["start", &container], "Try of starting lxc container was failed");
}

pub fn stop_lxc(container: &str) {
   template("lxc", vec!["stop", &container], "Try of stopping lxc container was failed");
}

pub fn del_lxc(container: &str) {
   template("lxc", vec!["delete", &container], "Failed to delete linux container");
}

pub fn exec_lxc(container: &str, command: &str) {
   template("lxc", vec!["exec", &container, "--", &command], "Failed to execute in lxc !");
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

// Storage Pool && Storage Volume
pub fn get_my_storages() {
   template("lxc", vec!["storage", "list"], "Failed to get storages");
}

pub fn get_storage_info(storage: &str) {
   template("lxc", vec!["storage", "info", &storage], "Failed to getting information about storage");
}

//pub fn create_storage(storage: &str, fs: &str, size: &str, source: &str) {
//   template("lxc", vec!["storage", "create", &storage, &fs, "size=", &size, "source=", &source], "F");
//}

pub fn set_storage_changes(storage: &str, key: &str, value: &str) {
   template("lxc", vec!["storage", "set", &storage, &key, &value], "Failed to ....");
}

pub fn del_storage(storage: &str) {
   template("lxc", vec!["storage", "delete", &storage], "Failed...");
}

pub fn get_volumes_by_storage(storage: &str) {
   template("lxc", vec!["storage", "volume", "list", &storage], "Failed...");
}

pub fn create_volume(storage: &str, name: &str) {
   template("lxc", vec!["storage", "volume", "create", &storage, &name], "Failed....");
}

pub fn attach_volume_lxc(storage: &str, volume: &str, container: &str, path: &str) {
   template("lxc", vec!["storage", "volume", "attach", &storage, &volume, &container, "data", &path], "Failed to ..");
}

pub fn detach_volume_lxc(storage: &str, volume: &str, container: &str) {
   template("lxc", vec!["storage", "volume", "detach", &storage, &volume, &container], "Failed to detach lxc volume"); 
}

pub fn delete_volume_lxc(storage: &str, volume: &str) {
   template("lxc", vec!["storage", "volume", "delete", &storage, &volume], "FF");
}

//Profiles

pub fn get_profiles() {
   template("lxc", vec!["profile", "list"], "FF");
}

pub fn get_profile_info(profile: &str) {
   template("lxc", vec!["profile", "show", &profile], "Failed....");
}

pub fn del_profile(profile: &str) {
   template("lxc", vec!["profile", "delete", &profile], "Failed....");
}

pub fn copy_profile(first: &str, second: &str) {
   template("lxc", vec!["profile", "copy", &first, &second], "Failed....");
}

pub fn rename_profile(first: &str, second: &str) {
   template("lxc", vec!["profile", "rename", &first, &second], "Failed...");
}

pub fn create_profile(profile: &str) {
   template("lxc", vec!["profile", "create", &profile], "Failed.....");
}

pub fn take_off_profile_from_lxc(container: &str) {
   template("lxc", vec!["profile", "remove", &container], "Failed to...");
}

// Networks

pub fn get_networks() {
   template("lxc", vec!["network", "list"], "Failed to....");
}

pub fn del_network(network: &str) {
   template("lxc", vec!["network", "delete", &network], "Failed to delete network");
}

pub fn get_network_info(network: &str) {
   template("lxc", vec!["network", "show", &network], "Failed to showing information about current network");
}

pub fn create_network(network: &str) {
   template("lxc", vec!["network", "create", &network], "Failed to create network");
}

pub fn edit_network(network: &str) {
   template("lxc", vec!["network", "edit", &network], "Failed to edit current network");
}

pub fn rename_network(first: &str, second: &str) {
   template("lxc", vec!["network", "rename", &first, &second], "Failed to rename current network");
}

pub fn copy_network(first: &str, second: &str) {
   template("lxc", vec!["network", "copy", &first, &second], "Failed to ");
}

// Users Image

pub fn publish_lxc_image(container: &str, alias: &str) {
   template("lxc", vec!["publish", &container, "--alias", &alias], "Failed to publish linux container image");
}

pub fn export_lxc_image(image: &str, name: &str) {
   template("lxc", vec!["image", "export", &image, &name], "Failed to export image");
}

pub fn import_lxc_image(image: &str, import_name: &str) {
   template("lxc", vec!["image", "import", &image, "--alias", &import_name], "Failed to import image");
}

// Snapshots

pub fn create_stateless_snapshot(container: &str, name: &str) {
   template("lxc", vec!["snapshot", &container, &name], "Failed");
}

pub fn create_statefull_snapshot() {}

pub fn restore_snapshot(container: &str, name: &str) {
   template("lxc", vec!["restore", &container, &name], "Failed....");
}

pub fn del_snapshot(container: &str, name: &str) {
   template("lxc", vec!["snapshot", "delete", &container, "/", &name], "F");
}
