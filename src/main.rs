use lxc_rust::{image, container, storage, snapshot};

fn main() {
    snapshot::create_local_lxc_stateless_snapshot("fedora36-container", "my-fedora-snapshot");
}

