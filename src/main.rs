extern crate lxc_rust;

use lxc_rust::daemon;

fn main() {
  daemon::recover_lxd_instance_lost_quorum();
}

