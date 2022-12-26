use crate::input::{block::Block, vars::Vars};

use super::block::create_block;

pub fn create(vars: Box<Vars>, blocks: Box<Vec<Block>>) {
    create_blocks(blocks);
}

fn create_blocks(blocks: Box<Vec<Block>>) {
    for block in blocks.iter() {
        create_block(block);
    }
}
