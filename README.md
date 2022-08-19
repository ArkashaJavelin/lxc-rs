# lxc-rs

Rust Cli (Client in future), that let you doing something with Linux containers

What is Linux Containers ?
If you heard and works with Docker(maybe with K8S too) - It would be easy for you'r understanding and working;

<image src="images/lxc.png" alt="Linux container, guys" title="Linux Container" />

How to use?

Example
```rs
use lxc_rs::lxc::{Container};

fn main() {
  Container.get_my_lxc(); // Prints you a list of you'r lxc in console
}
```

(Docs): Here is link: https://linuxcontainers.org/lxc/documentation/
(Repo): wefwefw
