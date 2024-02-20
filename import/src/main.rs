use std::fs::File;

use clap::Parser;

use open_shmup_data::Game;

use crate::options::Options;

mod options;

fn main() {
    let options = Options::parse();

    let mut file = File::open(options.path).unwrap();
    let game = Game::read_c64_prg(&mut file).unwrap();
}
