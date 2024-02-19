use std::fs::File;

use byteorder::{LittleEndian, ReadBytesExt};
use clap::Parser;

use crate::options::Options;

mod options;

fn main() {
    let options = Options::parse();

    let mut file = File::open(options.path).unwrap();
    let signature = file.read_u16::<LittleEndian>().unwrap();
    if signature != 0x42 {
        panic!("Not a SEUCK game");
    }
}
