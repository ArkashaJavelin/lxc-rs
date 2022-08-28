//! Lxc-rs
//!
//! A library for working with Linux Daemon && Linux Containers

pub use daemon::*;
pub use image::*;
pub use container::*; 
pub use storage::*;
pub use volume::*;
pub use profile::*;
pub use project::*;
pub use operation::*;
pub use remote_connection::*;
pub use config::*;
pub use network::*;
pub use snapshot::*;

  mod template {
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
  pub mod daemon {
    use crate::template::template;
    
    /// Initialize a linux daemon
    pub fn lxd_init() {
      template("lxd", vec!["init".to_string()], "Try to initialize lxd was failed");
    }
    
    /// Get current Lxd version
    pub fn get_lxd_version() {
      template("lxd", vec!["version".to_string()], "Try of get lxc was failed");
    }
    
    /// Shutdown LXD with containers and exit
    pub fn shutdown_lxd() {
      template("lxd", vec!["shutdown".to_string()], "Try of shutdown lxd process was failed")
    }
    
    /// Show cluster configuration as YAML.
    pub fn get_lxd_cluster_config() {
      template("lxd", vec!["cluster".to_string(), "show".to_string()], "Try of get lxd cluster configuration was failed") 
    }
    
    /// Print the addresses of the cluster members serving the database
    pub fn get_lxd_cluster_databases() {
      template("lxd", vec!["cluster".to_string(), "list-database".to_string()], "Try of get list of lxd cluster databases was failed")
    }
    
    /// Remove a raft node from the raft configuration
    pub fn del_lxd_cluster_raft_node_config(raft_node: &str) {
      template("lxd", vec!["cluster".to_string(), "remove-raft-node".to_string(), raft_node.to_string()], "Try of delete raft node from lxd cluster raft node config was failed")
    }

    /// Recover a LXD instance whose cluster has lost quorum
    pub fn recover_lxd_instance_lost_quorum() {
      template("lxd", vec!["cluster".to_string(), "recover-from-quorum-loss".to_string()], "Try of recover lxd cluster instance with lost quorum was failed")
    } 
  }

  // Images
  pub mod image {
    use crate::template::template;
    
    /// Get you'r local lxc images
    pub fn get_local_lxc_images() {
      template("lxc", vec!["image".to_string(), "list".to_string(), "local:".to_string()], "Try of get lxc was failed");
    }
    
    /// Get lxc images from lxc registry
    pub fn get_registry_lxc_images() {
      template("lxc", vec!["image".to_string(), "list".to_string(), "images:".to_string()], "Try of get lxc images was failed");
    }
    
    /// Search lxc images in registry 
    pub fn search_lxc_image(image: &str) {
      template("lxc", vec!["image".to_string(), "list".to_string(), "images:".to_string(), image.to_string()], "Try of get some lxc image was failed");
    }
    
    /// Get more infromation about current lxc image
    pub fn get_lxc_image_info(image: &str) {
      template("lxc", vec!["image".to_string(), "info".to_string(), image.to_string()], "Try of getting image information was failed");
    }
    
    /// Get a tiny infromation about current lxc image
    pub fn get_lxc_image_show(image: &str) {
      template("lxc", vec!["image".to_string(), "show".to_string(), image.to_string()], "Try of getting image information was failed");
    }
    
    /// Copy lxc image from registry to local with alias
    pub fn copy_lxc_image(image: &str, alias: &str) {
      template("lxc", vec!["image".to_string(), "copy".to_string(), "images:".to_string(), image.to_string(), "--alias".to_string(), alias.to_string()], "Failed to copy image with alias from remote store to local");
    }
    
    /// 
    pub fn publish_lxc_image(container: &str, alias: &str) {
      template("lxc", vec!["publish".to_string(), container.to_string(), "--alias".to_string(), alias.to_string()], "Failed to publish linux container image");
    }

    pub fn export_lxc_image(image: &str, name: &str) {
      template("lxc", vec!["image".to_string(), "export".to_string(), image.to_string(), name.to_string()], "Failed to export image");
    }
    
    /// Import lxc image with alias
    pub fn import_lxc_image(image: &str, import_name: &str) {
      template("lxc", vec!["image".to_string(), "import".to_string(), image.to_string(), "--alias".to_string(), import_name.to_string()], "Failed to import image");
    }
    
    /// Delete lxc image
    pub fn del_lxc_image(image: &str) {
      template("lxc", vec!["image".to_string(), "delete".to_string(), image.to_string()], "Try of delete image was failed");
    }
    
    /// Refresh lxc image
    pub fn refresh_lxc_image(image: &str) {
      template("lxc", vec!["image".to_string(), "refresh".to_string(), image.to_string()], "Failed to refresh a current image");
    }
    
    /// Set property to image
    pub fn set_image_property(image: &str, key: &str, value: &str) {
      template("lxc", vec!["image".to_string(), "set-property".to_string(), image.to_string(), key.to_string(), value.to_string()], "Failed to set image property");
    }
    
    /// Unset property from image
    pub fn unset_image_property(image: &str, key: &str) {
      template("lxc", vec!["image".to_string(), "unset-property".to_string(), image.to_string(), key.to_string()], "Failed to unset image property");
    }
    
    /// Get image aliases
    pub fn get_image_aliases() {
      template("lxc", vec!["image".to_string(), "alias".to_string(), "list".to_string()], "Failed to get image aliases");
    }
    
    /// Create image alias
    pub fn create_image_alias(alias: &str, fingerprint: &str) {
      template("lxc", vec!["image".to_string(), "create".to_string(), "create".to_string(), alias.to_string(), fingerprint.to_string()], "Failed to create image alias");
    }
    
    /// Delete image alias
    pub fn delete_image_alias(alias: &str) {
      template("lxc", vec!["image".to_string(), "delete".to_string(), alias.to_string()], "Failed to delete image alias");
    }
    
    /// Rename image alias
    pub fn rename_image_alias(old_name: &str, new_name: &str) {
      template("lxc", vec!["image".to_string(), "rename".to_string(), old_name.to_string(), new_name.to_string()], "Failed to rename image alias"); 
    }
  }

  // Container
  pub mod container {
    use crate::template::template;
    
    /// Get local lxc container
    pub fn get_local_lxc() {
      template("lxc", vec!["list".to_string(), "local:".to_string()], "Try of get lxc was failed");
    }
    
    /// Launch new lxc container
    pub fn launch_lxc(image: &str, container: &str) {
      template("lxc", vec!["launch".to_string(), format!("images:{}", image.to_string()), container.to_string()], "Try of launching container was failed");
    }
    
    /// Get information about lxc container
    pub fn get_lxc_info(container: &str) {
      template("lxc", vec!["info".to_string(), container.to_string()], "Failed to get linux container information");
    }
    
    /// Start lxc container
    pub fn start_lxc(container: &str) {
      template("lxc", vec!["start".to_string(), container.to_string()], "Try of starting lxc container was failed");
    }
    
    /// Stop lxc container
    pub fn stop_lxc(container: &str) {
      template("lxc", vec!["stop".to_string(), container.to_string()], "Try of stopping lxc container was failed");
    }
   
    /// Delete lxc container
    pub fn del_lxc(container: &str) {
      template("lxc", vec!["delete".to_string(), container.to_string()], "Failed to delete linux container");
    }
    
    /// Exec in lxc container
    pub fn exec_lxc(container: &str, command: &str) {
      template("lxc", vec!["exec".to_string(), container.to_string(), "--".to_string(), command.to_string()], "Failed to execute in lxc !");
    }
    
    /// Rename lxc container
    pub fn rename_lxc(container: &str, new_name: &str) {
      template("lxc", vec!["move".to_string(), container.to_string(), new_name.to_string()], "Failed to rename {&container#?}");
    }
    
    /// Restart lxc container
    pub fn restart_lxc(container: &str) {
      template("lxc", vec!["restart".to_string(), container.to_string()], "Failed to restart container");
    }
    
    /// Copy lxc container
    pub fn copy_lxc(container: &str, to_container: &str) {
      template("lxc", vec!["copy".to_string(), container.to_string(), to_container.to_string()], "Failed to copy from first to second");
    }
   
    /// Get lxc configuration
    pub fn get_lxc_config(container: &str) {
      template("lxc", vec!["config".to_string(), "show".to_string(), container.to_string()], "Failed to get lxc container configuration");
    }
    
    /// Publish file from local to lxc container
    pub fn push_file_in_lxc(file_path: &str, container_path: &str) {
      template("lxc", vec!["file".to_string(), "push".to_string(), file_path.to_string(), container_path.to_string()], "Failed to push files into container");
    }
    
    /// Pull file from container to local
    pub fn pull_file_from_lxc(container_path: &str, file_path: &str) {
      template("lxc", vec!["file".to_string(), "pull".to_string(), container_path.to_string(), file_path.to_string()], "Failed to pull files from container to current path");
   }
  }


  // Storage Pool && Storage Volume
  pub mod storage {
    use crate::template::template;
    
    /// Get local storages
    pub fn get_local_storages() {
      template("lxc", vec!["storage".to_string(), "list".to_string()], "Failed to get storages");
    }
    
    /// Get infromation about current storage
    pub fn get_storage_info(storage: &str) {
      template("lxc", vec!["storage".to_string(), "info".to_string(), storage.to_string()], "Failed to getting information about storage");
    }
    
    /// Create new storage
    pub fn create_storage(storage: &str, fs: &str, args: Vec<&str>) {
      template("lxc", vec!["storage".to_string(), "create".to_string(), storage.to_string(), fs.to_string()], "Failed to create storage");
    }
    
    /// Set property in config of current storage
    pub fn set_storage_config_property(storage: &str, key: &str, value: &str) {
      template("lxc", vec!["storage".to_string(), "set".to_string(), storage.to_string(), key.to_string(), value.to_string()], "Failed to set storage configuration property");
    }
    
    /// Unset property in config of current storage
    pub fn unset_storage_config_property(storage: &str, key: &str) {
      template("lxc", vec!["storage".to_string(), "unset".to_string(), storage.to_string(), key.to_string()], "Failed to unset storage property");
    }
    
    /// Get current proerty of storage config
    pub fn get_storage_config_property(storage: &str, key: &str) {
      template("lxc", vec!["storage".to_string(), "get".to_string(), storage.to_string(), key.to_string()], "Failed to get storage config property");
    }

    /// Delete current storage
    pub fn del_storage(storage: &str) {
      template("lxc", vec!["storage".to_string(), "delete".to_string(), storage.to_string()], "Failed to delete current storage");
    }
  }

  pub mod volume {
    use crate::template::template;
    
    /// Get volumes by current storage
    pub fn get_volumes_by_storage(storage: &str) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "list".to_string(), storage.to_string()], "Failed to get volumes by current storage");
    }
   
    /// Create volume of current storage
    pub fn create_volume(storage: &str, name: &str) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "create".to_string(), storage.to_string(), name.to_string()], "Failed to create volume");
    }
    
    /// Attach volume from current storage
    pub fn attach_volume_lxc(storage: &str, volume: &str, container: &str, path: &str) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "attach".to_string(), storage.to_string(), volume.to_string(), container.to_string(), "data".to_string(), path.to_string()], "Failed to attach lxc volume");
    }
   
    /// Attach volume profile from current storage
    pub fn attach_profile_volume_lxc(storage: &str, volume: &str, profile: &str) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "attach-profile".to_string(), storage.to_string(), volume.to_string(), profile.to_string()], "Failed to attach profile lxc volume");
    }
    
    /// Detach volume of the current storage
    pub fn detach_volume_lxc(storage: &str, volume: &str, container: &str) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "detach".to_string(), storage.to_string(), volume.to_string(), container.to_string()], "Failed to detach lxc volume"); 
    }
    
    /// Detach volume profile from current storage
    pub fn detach_profile_volume_lxc(storage: &str, volume: &str, profile: &str) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "detach-profile".to_string(), storage.to_string(), volume.to_string(), profile.to_string()], "Failed to detach profile volume lxc");
    }
    
    /// Delete volume of the current storage
    pub fn del_volume_lxc(storage: &str, volume: &str) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "delete".to_string(), storage.to_string(), volume.to_string()], "Failed to delete lxc volume");
    }
    
    /// Rename volume of the current storage
    pub fn rename_volume_lxc(storage: &str, old_name: &str, new_name: &str) {
      template("lxc", vec!["storage".to_string(), "volume".to_string(), "rename".to_string(), storage.to_string(), old_name.to_string(), new_name.to_string()], "Failed to rename current volume by that storage");
    }
  }

  //Profiles
  pub mod profile {
    use crate::template::template;
    
    /// Get local profiles 
    pub fn get_local_profiles() {
      template("lxc", vec!["profile".to_string(), "list".to_string()], "Failed to get local profiles");
    }
    
    /// Get info of the current profile
    pub fn get_profile_info(profile: &str) {
      template("lxc", vec!["profile".to_string(), "show".to_string(), profile.to_string()], "Failed to get info of the current profile");
    }
    
    /// Delete current profile
    pub fn del_profile(profile: &str) {
      template("lxc", vec!["profile".to_string(), "delete".to_string(), profile.to_string()], "Failed to delete current profile");
    }
    
    /// Copy current profile
    pub fn copy_profile(first: &str, second: &str) {
      template("lxc", vec!["profile".to_string(), "copy".to_string(), first.to_string(), second.to_string()], "Failed to copy current profile");
    }
    
    /// Rename current profile
    pub fn rename_profile(first: &str, second: &str) {
      template("lxc", vec!["profile".to_string(), "rename".to_string(), first.to_string(), second.to_string()], "Failed to rename current profile");
    }
    
    /// Create new profile
    pub fn create_profile(profile: &str) {
      template("lxc", vec!["profile".to_string(), "create".to_string(), profile.to_string()], "Failed to create new profile");
    }
    
    /// Remove profile from lxc 
    pub fn take_off_profile_from_lxc(container: &str) {
      template("lxc", vec!["profile".to_string(), "remove".to_string(), container.to_string()], "Failed to remove profile from current linux container");
    }
  }

  // Networks
  pub mod network {
    use crate::template::template;
    
    /// Get local networks
    pub fn get_local_networks() {
      template("lxc", vec!["network".to_string(), "list".to_string(), "local:".to_string()], "Failed to get local networks");
    }
    
    /// Delete current network
    pub fn del_network(network: &str) {
      template("lxc", vec!["network".to_string(), "delete".to_string(), network.to_string()], "Failed to delete network");
    }
    
    /// Get information about current network
    pub fn get_network_info(network: &str) {
      template("lxc", vec!["network".to_string(), "show".to_string(), network.to_string()], "Failed to showing information about current network");
    }
    
    /// Create new network
    pub fn create_network(network: &str) {
      template("lxc", vec!["network".to_string(), "create".to_string(), network.to_string()], "Failed to create network");
    }
    
    /// Rename current network
    pub fn rename_network(first: &str, second: &str) {
      template("lxc", vec!["network".to_string(), "rename".to_string(), first.to_string(), second.to_string()], "Failed to rename current network");
    }
    
    /// Copy current network
    pub fn copy_network(first: &str, second: &str) {
      template("lxc", vec!["network".to_string(), "copy".to_string(), first.to_string(), second.to_string()], "Failed to copy network");
    }
    
    /// Delete current ACL network
    pub fn del_network_acl(acl: &str) {
      template("lxc", vec!["network".to_string(), "acl".to_string(), "delete".to_string(), acl.to_string()], "Failed to delete acl network");
    }
   
    /// Get local network zones
    pub fn get_local_network_zones() {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "list".to_string(), "local".to_string()], "Failed to get network zones");
    }
    
    /// Get dhcp leases by current network
    pub fn get_dhcp_network_leases(network: &str) {
      template("lxc", vec!["network".to_string(), "list-leases".to_string(), network.to_string()], "Failed to get network dhcp leases");
    }
    
    /// Get forwards by current network
    pub fn get_network_forwards(network: &str) {
      template("lxc", vec!["network".to_string(), "forward".to_string(), "list".to_string(), network.to_string()], "Failed to get network forwards");
    }
    
    /// Uset property in current network config
    pub fn set_network_config_property(network: &str, key: &str, value: &str) {
      template("lxc", vec!["network".to_string(), "set".to_string(), network.to_string(), key.to_string(), value.to_string()], "Failed to set key/value in network config");
    }
    
    /// Unset property from current network config 
    pub fn unset_network_config_key(network: &str, key: &str) {
      template("lxc", vec!["network".to_string(), "unset".to_string(), network.to_string(), key.to_string()], "Failed to unset key in network config");
    }
    
    /// Create network zone
    pub fn create_network_zone(title: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "create".to_string(), title.to_string()], "Failed to create network zone");
    }
    
    /// Set network zone property
    pub fn set_network_zone_property(zone: &str, title: &str, value: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "set".to_string(), zone.to_string(), title.to_string(), value.to_string()], "Failed to set network zone key/value");
    }
    
    /// Unset network zone property
    pub fn unset_network_zone_key(zone: &str, key: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "unset".to_string(), zone.to_string(), key.to_string()], "Failed to unset network zone key");
    }
    
    /// Get information about current network zone
    pub fn get_network_zone_info(zone: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "show".to_string(), zone.to_string()], "Failed to get network zone information");
    }
    
    /// Delete current network zone
    pub fn del_network_zone(zone: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "delete".to_string(), zone.to_string()], "Failed to delete network zone");
    }
    
    /// Get network records by current zone
    pub fn get_network_zone_records(zone: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "list".to_string(), zone.to_string()], "Failed to get network zone records");
    }
    
    /// Get network records by current zone
    pub fn create_network_zone_record(zone: &str, title: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "create".to_string(), zone.to_string(), title.to_string()], "Failed to create network zone record");
    }
    
    /// Delete network record by current zone
    pub fn del_network_zone_record(zone: &str, title: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "delete".to_string(), zone.to_string(), title.to_string()], "Failed to delete network zone record");
    }
    
    /// Get information about current zone record
    pub fn get_network_zone_record_info(zone: &str, title: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "show".to_string(), zone.to_string(), title.to_string()], "Failed to get network zone record information");
    }
    
    /// Set network zone record property
    pub fn set_network_zone_record_property(zone: &str, title: &str, key: &str, value: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "set".to_string(), zone.to_string(), title.to_string(), key.to_string(), value.to_string()], "Failed to set network zone record key/value");
    }
    
    /// Unset network zone record property 
    pub fn unset_network_zone_record_property(zone: &str, title: &str, key: &str) {
      template("lxc", vec!["network".to_string(), "zone".to_string(), "record".to_string(), "unset".to_string(), zone.to_string(), title.to_string(), key.to_string()], "Failed to unset network zone record key");
    }
  }

  // Snapshots
  pub mod snapshot {
    use crate::template::template;
    
    /// Create stateless snapshot for current container
    pub fn create_lxc_stateless_snapshot(container: &str, name: &str) {
      template("lxc", vec!["snapshot".to_string(), container.to_string(), name.to_string()], "Failed to create stateless snapshot");
    }
    
    /// Restore snapshot by current container
    pub fn restore_lxc_snapshot(container: &str, name: &str) {
      template("lxc", vec!["restore".to_string(), container.to_string(), name.to_string()], "Failed to restore snapshot");
    }
    
    /// Delete snapshot by current container
    pub fn del_lxc_snapshot(container: &str, name: &str) {
      template("lxc", vec!["delete".to_string(), format!("{}/{}", container.to_string(), name.to_string())], "Failed to delete snapshot");
    }
  }

  // Config
  pub mod config {
    use crate::template::template;
    
    /// Set config property
    pub fn set_config_property(key: &str, value: &str) {
      template("lxc", vec!["config".to_string(), "set".to_string(), key.to_string(), value.to_string()], "Failed to set some changes to config");
    }
    
    /// Get current key from config
    pub fn get_config_key(key: &str) {
      template("lxc", vec!["config".to_string(), "get".to_string(), key.to_string()], "Failed to get value from config");
    }
    
    /// Unset config property
    pub fn unset_config_property(key: &str) {
      template("lxc", vec!["config".to_string(), "unset".to_string(), key.to_string()], "Failed to unset key from config file");
    }
    
    /// Get trust users of the current config
    pub fn get_trust_config_users() {
      template("lxc", vec!["config".to_string(), "trust".to_string(), "list".to_string()], "Failed to get trust configuration users");
    }
    
    /// Get active certificate trust tokens by current config 
    pub fn get_active_certificate_config_trust_tokens() {
      template("lxc", vec!["config".to_string(), "trust".to_string(), "list-tokens".to_string()], "Failed to get trust active tokens in config");
    }
    
    /// Delete trust user by current config
    pub fn del_trust_config_user(fingerprint: &str) {
      template("lxc", vec!["config".to_string(), "trust".to_string(), "remove".to_string(), fingerprint.to_string()], "Failed to delete trusted config users");
    }
    
    /// Show trust user by current config 
    pub fn show_trust_config_user(fingerprint: &str) {
      template("lxc", vec!["config".to_string(), "trust".to_string(), "show".to_string(), fingerprint.to_string()], "Failed to show trust configuration information");
    }
    
    /// Get templates by current config
    pub fn get_config_templates(fingerprint: &str) {
      template("lxc", vec!["config".to_string(), "template".to_string(), "list".to_string(), fingerprint.to_string()], "Failed to get config templates");
    }
    
    /// Delete current template by config
    pub fn del_config_template(fingerprint: &str, title: &str) {
      template("lxc", vec!["config".to_string(), "template".to_string(), "delete".to_string(), fingerprint.to_string(), title.to_string()], "Failed to delete configuration template by current config");
    }
    
    /// Get template details by current config
    pub fn get_config_template_details(fingerprint: &str, title: &str) {
      template("lxc", vec!["config".to_string(), "template".to_string(), "show".to_string(), fingerprint.to_string(), title.to_string()], "Failed to get details about current configuration template");
    }
    
    /// Create config template
    pub fn create_config_template(fingerprint: &str, title: &str) {
      template("lxc", vec!["config".to_string(), "template".to_string(), "create".to_string(), fingerprint.to_string(), title.to_string()], "Failed to create config template");
    }

    /// Show current metadata by config
    pub fn show_config_metadata(fingerprint: &str) {
      template("lxc", vec!["config".to_string(), "metadata".to_string(), "show".to_string(), fingerprint.to_string()], "Failed to get config metadatas by current fingerprint");
    }
    
    /// Get devices by current config
    pub fn get_config_devices(fingerprint: &str) {
      template("lxc", vec!["config".to_string(), "device".to_string(), "list".to_string(), fingerprint.to_string()], "Failed to get config devices");
    }
    
    /// Add new device for config 
    pub fn add_config_device() {
      template("lxc", vec!["config".to_string(), "device".to_string(), "add".to_string()], "Failed to add config device");
    }
    
    /// Unset current device from config
    pub fn unset_config_device(fingerprint: &str, device: &str, key: &str) {
      template("lxc", vec!["config".to_string(), "device".to_string(), "unset".to_string(), fingerprint.to_string(), device.to_string(), key.to_string()], "Failed to unset configuration device");
    }
    
    /// Delete current device from config
    pub fn del_config_device(fingerprint: &str, title: &str) {
      template("lxc", vec!["config".to_string(), "device".to_string(), "remove".to_string(), fingerprint.to_string(), title.to_string()], "Failed to delete configuration device");
    }
    
    /// Get details about current device by config
    pub fn get_config_device_details(fingerprint: &str) {
      template("lxc", vec!["config".to_string(), "device".to_string(), "show".to_string(), fingerprint.to_string()], "Failed to get config device configuration details");
    }
  }

  // Remote connection
  pub mod remote_connection {
    use crate::template::template;
    
    /// Connect to remote lxc registry 
    pub fn connect_to_remote_registry(name: &str, address: &str) {
      template("lxc", vec!["remote".to_string(), "add".to_string(), name.to_string(), address.to_string()], "Failed to connect to remote lxc");
    }
    
    /// Rename remote registry
    pub fn rename_remote_registry(instance: &str, title: &str) {
      template("lxc", vec!["remote".to_string(), "rename".to_string(), instance.to_string(), title.to_string()], "Failed to rename remote");
    }
    
    /// Get remote registries
    pub fn get_remote_registries() {
      template("lxc", vec!["remote".to_string(), "list".to_string()], "Failed to get remote storages");
    }
    
    /// Get remote default registry
    pub fn get_remote_default_registry() {
      template("lxc", vec!["remote".to_string(), "get-default".to_string()], "Failed to get default remote storage....");
    }
  }

  // Operation
  pub mod operation {
    use crate::template::template;
    
    /// Get local background operations
    pub fn get_background_operations() {
      template("lxc", vec!["operation".to_string(), "list".to_string(), "local:".to_string()], "Failed to get background operations");
    }
   
    /// Delete background operation
    pub fn del_background_operation(operation: &str) {
      template("lxc", vec!["operation".to_string(), "delete".to_string(), operation.to_string()], "Failed to delete background operation");
    }
    
    /// Get details about current background operation
    pub fn get_background_operation_details(operation: &str) {
      template("lxc", vec!["operation".to_string(), "delete".to_string(), operation.to_string()], "Failed to get background operation details");
    }
  }

  // Project
  pub mod project {
    use crate::template::template;
    
    /// Get local projects
    pub fn get_local_projects() {
      template("lxc", vec!["project".to_string(), "list".to_string(), "local:".to_string()], "Failed to get all projects");
    }
    
    /// Rename current project
    pub fn rename_project(oldname: &str, newname: &str) {
      template("lxc", vec!["project".to_string(), "rename".to_string(), oldname.to_string(), newname.to_string()], "Failed to rename project");
    }
    
    /// Delete current project
    pub fn delete_project(project: &str) {
      template("lcx", vec!["project".to_string(), "delete".to_string(), project.to_string()], "Failed to delete project");
    }
    
    /// Get details about current project
    pub fn get_project_details(project: &str) {
      template("lxc", vec!["project".to_string(), "info".to_string(), project.to_string()], "Failed to get project details");
    }
    
    /// Get options by current project 
    pub fn get_project_options(project: &str) {
      template("lxc", vec!["project".to_string(), "show".to_string(), project.to_string()], "Failed to get project options");
    }
    
    /// Switch current project
    pub fn switch_current_project(another_project: &str) {
      template("lxc", vec!["project".to_string(), "switch".to_string(), another_project.to_string()], "Failde to switch from current project to another");
    }
    
    /// Create new project
    pub fn create_project(title: &str) {
      template("lxc", vec!["project".to_string(), "create".to_string(), title.to_string()], "Failed to create new project");
    }
    
    /// Set project config property 
    pub fn set_project_config_property(project: &str, key: &str, value: &str) {
      template("lxc", vec!["project".to_string(), "set".to_string(), project.to_string(), key.to_string(), value.to_string()], "Failed to set project configuration key");
    }
    
    /// Unset project config property
    pub fn unset_project_config_property(project: &str, key: &str) {
      template("lxc", vec!["project".to_string(), "unset".to_string(), project.to_string(), key.to_string()], "Failed to unset project configuration key");
    }
  }

