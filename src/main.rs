use lxc_rust::{image, container, storage, snapshot, network};

fn main() {
   //storage::create_local_storage("my-storage", "btrfs");

   network::create_local_network("my-new-net");
}

