pub mod lxc {
  mod Template {
    use std::process::Command;

    pub fn template(cm: &str, args: Vec<String>, _err_message: &str) {
      let cmd = Command::new(&cm).args(args).output().unwrap_or_else(|e| {
        panic!("{}", &e)
      });

      let result = String::from_utf8_lossy(&cmd.stdout);

      print!("{}", result);
    }
  }

  // LXdaemon
  pub mod Daemon {
    use crate::lxc::Template::template;

    pub fn lxd_init() {
      template("lxd", vec!["init".to_string()], "Try to initialize lxd was failed");
    }

    pub fn get_lxc_version() {
      template("lxc", vec!["--version".to_string()], "Try of get lxc was failed");
    }
  }

  // Images
  pub mod Image {
    use crate::lxc::Template::template;

    pub fn get_my_lxc_images() {
      template("lxc", vec!["image".to_string(), "list".to_string()], "Try of get lxc was failed");
    }

    pub fn get_lxc_images() {
      template("lxc", vec!["image".to_string(), "list".to_string(), "images:".to_string()], "Try of get lxc images was failed");
    }

    pub fn search_lxc_image(image: String) {
      template("lxc", vec!["image".to_string(), "list".to_string(), "images:".to_string(), image.to_string()], "Try of get some lxc image was failed");
    }

    pub fn get_lxc_image_info(image: String) {
      template("lxc", vec!["image".to_string(), "info".to_string(), image.to_string()], "Try of getting image information was failed");
    }

    pub fn get_lxc_image_show(image: String) {
      template("lxc", vec!["image".to_string(), "show".to_string(), image.to_string()], "Try of getting image information was failed");
    }

    pub fn copy_lxc_image(image: String, alias: String) {
      template("lxc", vec!["image".to_string(), "copy".to_string(), "images:".to_string(), image.to_string(), "--alias".to_string(), alias.to_string()], "Failed to copy image with alias from remote store to local");
    }

    pub fn publish_lxc_image(container: String, alias: String) {
      template("lxc", vec!["publish".to_string(), container.to_string(), "--alias".to_string(), alias.to_string()], "Failed to publish linux container image");
    }

    pub fn export_lxc_image(image: String, name: String) {
      template("lxc", vec!["image".to_string(), "export".to_string(), image.to_string(), name.to_string()], "Failed to export image");
    }

    pub fn import_lxc_image(image: String, import_name: String) {
      template("lxc", vec!["image".to_string(), "import".to_string(), image.to_string(), "--alias".to_string(), import_name.to_string()], "Failed to import image");
    }

    pub fn del_lxc_image(image: String) {
      template("lxc", vec!["image".to_string(), "delete".to_string(), image.to_string()], "Try of delete image was failed");
    }

    pub fn refresh_image(image: String) {
      template("lxc", vec!["image".to_string(), "refresh".to_string(), image.to_string()], "Failed to refresh a current image");
    }

    pub fn set_image_property(image: String, key: String, value: String) {
      template("lxc", vec!["image".to_string(), "set-property".to_string(), image.to_string(), key.to_string(), value.to_string()], "Failed to set image property");
    }

    pub fn unset_image_property(image: String, key: String) {
      template("lxc", vec!["image".to_string(), "unset-property".to_string(), image.to_string(), key.to_string()], "Failed to unset image property");
    }

    pub fn get_image_aliases() {
      template("lxc", vec!["image".to_string(), "alias".to_string(), "list".to_string()], "Failed to get image aliases");
    }

    pub fn create_image_alias(alias: String, fingerprint: String) {
      template("lxc", vec!["image".to_string(), "create".to_string(), "create".to_string(), alias.to_string(), fingerprint.to_string()], "Failed to create image alias");
    }

    pub fn delete_image_alias(alias: String) {
      template("lxc", vec!["image".to_string(), "delete".to_string(), alias.to_string()], "Failed to delete image alias");
    }

    pub fn rename_image_alias(old_name: String, new_name: String) {
      template("lxc", vec!["image".to_string(), "rename".to_string(), old_name.to_string(), new_name.to_string()], "Failed to rename image alias"); 
    }
  }

  // Container
  pub mod Container {
    use crate::lxc::Template::template;

    pub fn get_my_lxc() {
      template("lxc", vec!["list".to_string()], "Try of get lxc was failed");
    }
 
    pub fn launch_lxc(image: String, container: String) {
      template("lxc", vec!["launch".to_string(), format!("images:{}", image.to_string()), container.to_string()], "Try of launching container was failed");
    }

    pub fn get_lxc_info(container: String) {
      template("lxc", vec!["info".to_string(), container.to_string()], "Failed to get linux container information");
    }

    pub fn start_lxc(container: String) {
      template("lxc", vec!["start".to_string(), container.to_string()], "Try of starting lxc container was failed");
    }

    pub fn stop_lxc(container: String) {
      template("lxc", vec!["stop".to_string(), container.to_string()], "Try of stopping lxc container was failed");
    }

    pub fn del_lxc(container: String) {
      template("lxc", vec!["delete".to_string(), container.to_string()], "Failed to delete linux container");
    }

    pub fn exec_lxc(container: String, command: String) {
      template("lxc", vec!["exec".to_string(), container.to_string(), "--".to_string(), String::from(command)], "Failed to execute in lxc !");
    }

    pub fn rename_lxc(container: String, new_name: String) {
      template("lxc", vec!["move".to_string(), container.to_string(), new_name.to_string()], "Failed to rename {&container#?}");
    }

    pub fn restart_lxc(container: String) {
      template("lxc", vec!["restart".to_string(), container.to_string()], "Failed to restart container");
    }

    pub fn copy_lxc(container: String, to_container: String) {
      template("lxc", vec!["copy".to_string(), container.to_string(), to_container.to_string()], "Failed to copy from first to second");
    }

    pub fn get_lxc_config(container: String) {
      template("lxc", vec!["config".to_string(), "show".to_string(), container.to_string()], "Failed to get lxc container configuration");
    }

    pub fn push_file_in_lxc(file_path: String, container_path: String) {
      template("lxc", vec!["file".to_string(), "push".to_string(), file_path.to_string(), container_path.to_string()], "Failed to push files into container");
    }

    pub fn pull_file_from_lxc(container_path: String, file_path: String) {
      template("lxc", vec!["file".to_string(), "pull".to_string(), container_path.to_string(), file_path.to_string()], "Failed to pull files from container to current path");
   }
  }


  // Storage Pool && Storage Volume
  pub mod Storage {
    use crate::lxc::Template::template;

    pub fn get_my_storages() {
      template("lxc", vec!["storage".to_string(), "list".to_string()], "Failed to get storages");
    }

    pub fn get_storage_info(storage: String) {
      template("lxc", vec!["storage".to_string(), "info".to_string(), storage.to_string()], "Failed to getting information about storage");
    }

    pub fn create_storage(storage: String, fs: String, args: Vec<String>) {
      template("lxc", vec!["storage".to_string(), "create".to_string(), storage.to_string(), fs.to_string()], "Failed to create storage");
    }

    pub fn set_storage_config_property(storage: String, key: String, value: String) {
      template("lxc", vec!["storage".to_string(), "set".to_string(), storage.to_string(), key.to_string(), value.to_string()], "Failed to set storage configuration property");
    }

    pub fn unset_storage_config_property(storage: String, key: String) {
      template("lxc", vec!["storage".to_string(), "unset".to_string(), storage.to_string(), key.to_string()], "Failed to unset storage property");
    }

    pub fn get_storage_config_property(storage: String, key: String) {
      template("lxc", vec!["storage".to_string(), "get".to_string(), storage.to_string(), key.to_string()], "Failed to get storage config property");
    }

    pub fn del_storage(storage: String) {
      template("lxc", vec!["storage".to_string(), "delete".to_string(), storage.to_string()], "Failed to delete current storage");
    }
  }

  pub mod Volume {
    use crate::lxc::Template::template;

    pub fn get_volumes_by_storage(storage: String) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "list".to_string(), storage.to_string()], "Failed to get volumes by current storage");
    }

    pub fn create_volume(storage: String, name: String) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "create".to_string(), storage.to_string(), name.to_string()], "Failed to create volume");
    }

    pub fn attach_volume_lxc(storage: String, volume: String, container: String, path: String) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "attach".to_string(), storage.to_string(), volume.to_string(), container.to_string(), "data".to_string(), path.to_string()], "Failed to attach lxc volume");
    }

    pub fn attach_profile_volume_lxc(storage: String, volume: String, profile: String) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "attach-profile".to_string(), storage.to_string(), volume.to_string(), profile.to_string()], "Failed to attach profile lxc volume");
    }

    pub fn detach_volume_lxc(storage: String, volume: String, container: String) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "detach".to_string(), storage.to_string(), volume.to_string(), container.to_string()], "Failed to detach lxc volume"); 
    }

    pub fn detach_profile_volume_lxc(storage: String, volume: String, profile: String) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "detach-profile".to_string(), storage.to_string(), volume.to_string(), profile.to_string()], "Failed to detach profile volume lxc");
    }

    pub fn del_volume_lxc(storage: String, volume: String) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "delete".to_string(), storage.to_string(), volume.to_string()], "Failed to delete lxc volume");
    }

    pub fn rename_volume_lxc(storage: String, old_name: String, new_name: String) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "rename".to_string(), storage.to_string(), old_name.to_string(), new_name.to_string()], "Failed to rename current volume by that storage");
    }
  }

  //Profiles
  pub mod Profile {
    use crate::lxc::Template::template;
 
    pub fn get_profiles() {
      template("lxc", vec!["profile".to_string(), "list".to_string()], "FF");
    }

    pub fn get_profile_info(profile: String) {
      template("lxc", vec!["profile".to_string(), "show".to_string(), profile.to_string()], "Failed....");
    }

    pub fn del_profile(profile: String) {
      template("lxc", vec!["profile".to_string(), "delete".to_string(), profile.to_string()], "Failed....");
    }

    pub fn copy_profile(first: String, second: String) {
      template("lxc", vec!["profile".to_string(), "copy".to_string(), first.to_string(), second.to_string()], "Failed....");
    }

    pub fn rename_profile(first: String, second: String) {
      template("lxc", vec!["profile".to_string(), "rename".to_string(), first.to_string(), second.to_string()], "Failed...");
    }

    pub fn create_profile(profile: String) {
      template("lxc", vec!["profile".to_string(), "create".to_string(), profile.to_string()], "Failed.....");
    }

    pub fn take_off_profile_from_lxc(container: String) {
      template("lxc", vec!["profile".to_string(), "remove".to_string(), container.to_string()], "Failed to...");
    }
  }

  // Networks
  pub mod Network {
    use crate::lxc::Template::template;

    pub fn get_networks() {
      template("lxc", vec!["network".to_string(), "list".to_string()], "Failed to....");
    }

    pub fn del_network(network: String) {
      template("lxc", vec!["network".to_string(), "delete".to_string(), network.to_string()], "Failed to delete network");
    }

    pub fn get_network_info(network: String) {
      template("lxc", vec!["network".to_string(), "show".to_string(), network.to_string()], "Failed to showing information about current network");
    }

    pub fn create_network(network: String) {
      template("lxc", vec!["network".to_string(), "create".to_string(), network.to_string()], "Failed to create network");
    }

    pub fn rename_network(first: String, second: String) {
      template("lxc", vec!["network".to_string(), "rename".to_string(), first.to_string(), second.to_string()], "Failed to rename current network");
    }

    pub fn copy_network(first: String, second: String) {
      template("lxc", vec!["network".to_string(), "copy".to_string(), first.to_string(), second.to_string()], "Failed to copy network");
    }

    pub fn del_network_acl(acl: String) {
      template("lxc", vec!["network".to_string(), "acl".to_string(), "delete".to_string(), acl.to_string()], "Failed to delete acl network");
    }

    pub fn get_network_zones() {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "list".to_string()], "Failed to get network zones");
    }

    pub fn get_dhcp_network_leases(network: String) {
      template("lxc", vec!["network".to_string(), "list-leases".to_string(), network.to_string()], "Failed to get network dhcp leases");
    }

    pub fn get_network_forwards(network: String) {
      template("lxc", vec!["network".to_string(), "forward".to_string(), "list".to_string(), network.to_string()], "Failed to get network forwards");
    }

    pub fn set_network_config_key(network: String, key: String, value: String) {
      template("lxc", vec!["network".to_string(), "set".to_string(), network.to_string(), key.to_string(), value.to_string()], "Failed to set key/value in network config");
    }

    pub fn unset_network_config_key(network: String, key: String) {
      template("lxc", vec!["network".to_string(), "unset".to_string(), network.to_string(), key.to_string()], "Failed to unset key in network config");
    }

    pub fn create_network_zone(title: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "create".to_string(), title.to_string()], "Failed to create network zone");
    }

    pub fn set_network_zone_key(zone: String, title: String, value: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "set".to_string(), zone.to_string(), title.to_string(), value.to_string()], "Failed to set network zone key/value");
    }

    pub fn unset_network_zone_key(zone: String, key: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "unset".to_string(), zone.to_string(), key.to_string()], "Failed to unset network zone key");
    }

    pub fn get_network_zone_info(zone: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "show".to_string(), zone.to_string()], "Failed to get network zone information");
    }

    pub fn del_network_zone(zone: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "delete".to_string(), zone.to_string()], "Failed to delete network zone");
    }

    pub fn get_network_zone_records(zone: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "list".to_string(), zone.to_string()], "Failed to get network zone records");
    }

    pub fn create_network_zone_record(zone: String, title: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "create".to_string(), zone.to_string(), title.to_string()], "Failed to create network zone record");
    }

    pub fn del_network_zone_record(zone: String, title: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "delete".to_string(), zone.to_string(), title.to_string()], "Failed to delete network zone record");
    }

    pub fn get_network_zone_record_info(zone: String, title: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "show".to_string(), zone.to_string(), title.to_string()], "Failed to get network zone record information");
    }

    pub fn set_network_zone_record_key(zone: String, title: String, key: String, value: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "set".to_string(), zone.to_string(), title.to_string(), key.to_string(), value.to_string()], "Failed to set network zone record key/value");
    }

    pub fn unset_network_zone_record_key(zone: String, title: String, key: String) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "unset".to_string(), zone.to_string(), title.to_string(), key.to_string()], "Failed to unset network zone record key");
    }
  }

  // Snapshots
  pub mod Snapshot {
    use crate::lxc::Template::template;
 
    pub fn create_lxc_stateless_snapshot(container: String, name: String) {
      template("lxc", vec!["snapshot".to_string(), container.to_string(), name.to_string()], "Failed to create stateless snapshot");
    }

    pub fn restore_lxc_snapshot(container: String, name: String) {
      template("lxc", vec!["restore".to_string(), container.to_string(), name.to_string()], "Failed to restore snapshot");
    }

    pub fn del_lxc_snapshot(container: String, name: String) {
      template("lxc", vec!["delete".to_string(), format!("{}/{}", container.to_string(), name.to_string())], "Failed to delete snapshot");
    }
  }

  // Config
  pub mod Config {
    use crate::lxc::Template::template;

    pub fn set_config_changes(key: String, value: String) {
      template("lxc", vec!["config".to_string(), "set".to_string(), key.to_string(), value.to_string()], "Failed to set some changes to config");
    }

    pub fn get_config_value(key: String) {
      template("lxc", vec!["config".to_string(), "get".to_string(), key.to_string()], "Failed to get value from config");
    }

    pub fn unset_config_key(key: String) {
      template("lxc", vec!["config".to_string(), "unset".to_string(), key.to_string()], "Failed to unset key from config file...");
    }

    pub fn get_trust_config_users() {
      template("lxc", vec!["config".to_string(), "trust".to_string(), "list".to_string()], "Failed to get trust configuration users..........");
    }

    pub fn get_active_certificate_config_trust_tokens() {
      template("lxc", vec!["config".to_string(), "trust".to_string(), "list-tokens".to_string()], "Failed to get trust active tokens in config");
    }

    pub fn del_trust_config_user(fingerprint: String) {
      template("lxc", vec!["config".to_string(), "trust".to_string(), "remove".to_string(), fingerprint.to_string()], "Failed to delete trusted config users");
    }

    pub fn show_trust_config_user(fingerprint: String) {
      template("lxc", vec!["config".to_string(), "trust".to_string(), "show".to_string(), fingerprint.to_string()], "Failed to show trust configuration information");
    }

    pub fn get_config_templates(fingerprint: String) {
      template("lxc", vec!["config".to_string(), "template".to_string(), "list".to_string(), fingerprint.to_string()], "Failed to get config templates");
    }

    pub fn del_config_template(fingerprint: String, title: String) {
      template("lxc", vec!["config".to_string(), "template".to_string(), "delete".to_string(), fingerprint.to_string(), title.to_string()], "Failed to delete configuration template by current config");
    }

    pub fn get_config_template_details(fingerprint: String, title: String) {
      template("lxc", vec!["config".to_string(), "template".to_string(), "show".to_string(), fingerprint.to_string(), title.to_string()], "Failed to get details about current configuration template");
    }

    pub fn create_config_template(fingerprint: String, title: String) {
      template("lxc", vec!["config".to_string(), "template".to_string(), "create".to_string(), fingerprint.to_string(), title.to_string()], "Failed to create config template");
    }

    pub fn show_config_metadata(fingerprint: String) {
      template("lxc", vec!["config".to_string(), "metadata".to_string(), "show".to_string(), fingerprint.to_string()], "Failed to get config metadatas by current fingerprint");
    }

    pub fn get_config_devices(fingerprint: String) {
      template("lxc", vec!["config".to_string(), "device".to_string(), "list".to_string(), fingerprint.to_string()], "Failed to get config devices");
    }

    pub fn add_config_device() {
      template("lxc", vec!["config".to_string(), "device".to_string(), "add".to_string()], "Failed to add config device");
    }

    pub fn unset_config_device(fingerprint: String, device: String, key: String) {
      template("lxc", vec!["config".to_string(), "device".to_string(), "unset".to_string(), fingerprint.to_string(), device.to_string(), key.to_string()], "Failed to unset configuration device");
    }

    pub fn del_config_device(fingerprint: String, title: String) {
      template("lxc", vec!["config".to_string(), "device".to_string(), "remove".to_string(), fingerprint.to_string(), title.to_string()], "Failed to delete configuration device");
    }

    pub fn get_config_device_details(fingerprint: String) {
      template("lxc", vec!["config".to_string(), "device".to_string(), "show".to_string(), fingerprint.to_string()], "Failed to get config device configuration details");
    }
  }

  // Remote connection
  pub mod RemoteConnection {
    use crate::lxc::Template::template;

    pub fn connect_to_remote_lxc(name: String, address: String) {
      template("lxc", vec!["remote".to_string(), "add".to_string(), name.to_string(), address.to_string()], "Failed to connect to remote lxc");
    }

    pub fn rename_remote(instance: String, title: String) {
      template("lxc", vec!["remote".to_string(), "rename".to_string(), instance.to_string(), title.to_string()], "Failed to rename remote");
   }

    pub fn get_remote_storages() {
      template("lxc", vec!["remote".to_string(), "list".to_string()], "Failed to get remote storages");
    }

    pub fn get_remote_default_storage() {
      template("lxc", vec!["remote".to_string(), "get-default".to_string()], "Failed to get default remote storage....");
    }
  }

  // Operation
  pub mod Operation {
    use crate::lxc::Template::template;

    pub fn get_background_operations() {
      template("lxc", vec!["operation".to_string(), "list".to_string()], "Failed to get background operations");
    }

    pub fn del_background_operation(operation: String) {
      template("lxc", vec!["operation".to_string(), "delete".to_string(), operation.to_string()], "Failed to delete background operation");
    }

    pub fn get_background_operation_details(operation: String) {
      template("lxc", vec!["operation".to_string(), "delete".to_string(), operation.to_string()], "Failed to get background operation details");
    }
  }

  // Project
  pub mod Project {
    use crate::lxc::Template::template;

    pub fn get_projects() {
      template("lxc", vec!["project".to_string(), "list".to_string()], "Failed to get all projects");
    }

    pub fn rename_project(oldname: String, newname: String) {
      template("lxc", vec!["project".to_string(), "rename".to_string(), oldname.to_string(), newname.to_string()], "Failed to rename project");
    }

    pub fn delete_project(project: String) {
      template("lcx", vec!["project".to_string(), "delete".to_string(), project.to_string()], "Failed to delete project");
    }

    pub fn get_project_details(project: String) {
      template("lxc", vec!["project".to_string(), "info".to_string(), project.to_string()], "Failed to get project details");
    }

    pub fn get_project_options(project: String) {
      template("lxc", vec!["project".to_string(), "show".to_string(), project.to_string()], "Failed to get project options");
    }

    pub fn switch_current_project(another_project: String) {
      template("lxc", vec!["project".to_string(), "switch".to_string(), another_project.to_string()], "Failde to switch from current project to another");
    }

    pub fn create_project(title: String) {
      template("lxc", vec!["project".to_string(), "create".to_string(), title.to_string()], "Failed to create new project");
    }

    pub fn set_project_config_key(project: String, key: String, value: String) {
      template("lxc", vec!["project".to_string(), "set".to_string(), project.to_string(), key.to_string(), value.to_string()], "Failed to set project configuration key");
    }

    pub fn unset_project_config_key(project: String, key: String) {
      template("lxc", vec!["project".to_string(), "unset".to_string(), project.to_string(), key.to_string()], "Failed to unset project configuration key");
    }
  }
}
