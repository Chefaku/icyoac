use std::io::Write;
use std::path::Path;
use std::{fs, fs::File};

use super::block::create_block;
use crate::input::{block::Block, vars::Vars};

pub fn create(vars: Box<Vars>, blocks: Box<Vec<Block>>) {
    dir_output();
    create_blocks(blocks);
}

fn dir_output() {
    let path = Path::new("./output");
    if path.exists() {
        return;
    }
    fs::create_dir(path).unwrap();
}

fn create_blocks(blocks: Box<Vec<Block>>) {
    let mut vec: Vec<String> = Vec::new();
    for block in blocks.iter() {
        vec.push(create_block(block));
    }

    let mut file = File::create("./output/index.html").unwrap();
    for s in vec {
        file.write_all(s.as_bytes()).unwrap();
    }
}
