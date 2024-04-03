#![allow(dead_code)]

extern crate clap;
use std::path::PathBuf;

use clap::Parser;

#[cfg(test)]
mod test;

#[derive(Debug, Parser)]
pub struct Find {
    location: Option<PathBuf>,
    #[arg(short, long, value_name = "FILE NAME OR FILE EXTENSION")]
    name: Option<String>,
}

#[derive(Debug)]
pub struct FindImpl {
    find_instance: Find,
    sub_dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

impl FindImpl {
    pub fn new(find_instance: Find) -> Self {
        FindImpl {
            find_instance,
            sub_dirs: vec![],
            files: vec![],
        }
    }

    pub fn get_sub_dirs(&mut self, path: PathBuf) {
        match std::fs::read_dir(path) {
            Ok(contents) =>{
                
            },
            Err(error) => {
                println!("\x1b[31m {error} \x1b[0m")
            }
        }
    }

    fn populate(&mut self) {}

    fn print_dir_contents(&mut self) {}
}
