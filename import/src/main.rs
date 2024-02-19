use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    let mut file = File::open("TODO").unwrap();
    let signature = file.read_u16::<LittleEndian>().unwrap();
    if signature != 0x42 {
        panic!("Not a SEUCK game");
    }
}
