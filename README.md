# lxc-rs

Rust Cli (Client in future), that let you doing something with Linux containers

What is Linux Containers ?
If you heard and works with Docker(maybe with K8S too) - It would be easy for you'r understanding and working;

<image src="images/lxc.png" alt="Linux container, guys" title="Linux Container" />

How to use?

Example
```rs
extern crate lxc_rs;

use lxc_rs::{Container};

fn main() {
  Container.get_local_lxc(); // Prints you a list of you'r local lxc in console
}
```

(Docs): https://linuxcontainers.org/lxc/documentation/
(Repo): https://github.com/lxc/lxc
