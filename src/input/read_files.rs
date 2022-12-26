use std::fs::read_to_string;

use serde_yaml::from_str;

use super::block::Block;
use super::vars::Vars;

const PATH_TO_VARS: &str = "input/vars.yaml";
const PATH_TO_BLOCKS: &str = "input/blocks.yaml";

pub fn read() -> (Box<Vars>, Box<Vec<Block>>) {
    let vars = readvars();
    let blocks = readblocks();
    (vars, blocks)
}

fn readvars() -> Box<Vars> {
    let yaml = read_to_string(PATH_TO_VARS).unwrap();
    let data: Vars = from_str(&yaml).unwrap();
    Box::new(data)
}

fn readblocks() -> Box<Vec<Block>> {
    let yaml = read_to_string(PATH_TO_BLOCKS).unwrap();
    let data: Vec<Block> = from_str(&yaml).unwrap();
    Box::new(data)
}
