use std::fs::read_to_string;

use serde_yaml::from_str;

use super::vars::Vars;
use super::block::Block;

const PATH_TO_VARS: &str = "inputs/vars.yaml";
const PATH_TO_BLOCKS: &str = "inputs/blocks.yaml";

pub fn read() {
    readvars();
    readblocks();
}

fn readvars() {
    let yaml = read_to_string(PATH_TO_VARS).unwrap();
    let data: Vars = from_str(&yaml).unwrap();
    println!("{:?}", data);
}

fn readblocks() {
    let yaml = read_to_string(PATH_TO_BLOCKS).unwrap();
    let data: Vec<Block> = from_str(&yaml).unwrap();
    println!("{:?}", data);
}
