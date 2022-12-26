use crate::input::{block::Block, vars::Vars};

use super::block::create_block;

pub fn create(vars: Box<Vars>, blocks: Box<Vec<Block>>) {
    dir_output();
    create_blocks(blocks);
}

fn dir_output() {
    let path = std::path::Path::new("./output");
    if path.exists() {
        return;
    }
    std::fs::create_dir(path).unwrap();
}

fn create_blocks(blocks: Box<Vec<Block>>) {
    let mut vec: Vec<String> = Vec::new();

    for block in blocks.iter() {
        vec.push(create_block(block));
    }

    for string in vec {
        print!("{}", string);
    }
}
