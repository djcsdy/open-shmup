use crate::c64::enemies::EightWayPathEntry;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct C64EnemyPath(Vec<EightWayPathEntry>);
