use std::path::PathBuf;

use clap::Parser;
use find_util::{Find, FindImpl};

extern crate clap;

fn main() {
    let find = Find::parse();
    let mut find_impl = FindImpl::new(find);
    let path = PathBuf::from("./");
    find_impl.get_sub_dirs(path);
}
