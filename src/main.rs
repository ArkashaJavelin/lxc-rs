extern crate lxc_rs;
use lxc_rs::daemon;

fn main() {
  daemon::recover_lxd_instance_lost_quorum();
}

