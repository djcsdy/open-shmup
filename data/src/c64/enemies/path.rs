use crate::c64::enemies::C64EnemyPathEntry;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct C64EnemyPath(Vec<C64EnemyPathEntry>);
