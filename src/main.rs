extern crate lxc_rs;

use lxc_rs::image;

fn main() { 
   image::refresh_lxc_image(String::from("e564b08cf2f6"));
}

