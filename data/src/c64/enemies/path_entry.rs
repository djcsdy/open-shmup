use crate::c64::enemies::C64EnemyDirection;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct C64EnemyPathEntry {
    direction: C64EnemyDirection,
    distance: u8,
}
