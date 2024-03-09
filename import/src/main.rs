use std::fs::{File, OpenOptions};
use std::path::Path;

use clap::Parser;

use open_shmup_data::GameData;

use crate::options::Options;

mod options;

fn main() {
    let options = Options::parse();

    let in_path = Path::new(&options.path);
    let out_path = in_path.with_extension("openshmup");

    if in_path == out_path {
        panic!("Invalid input path");
    }

    let mut in_file = File::open(in_path).unwrap();
    let game = GameData::read_c64_prg(&mut in_file).unwrap();

    let mut out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out_path)
        .unwrap();
    game.write(&mut out_file).unwrap();
}
