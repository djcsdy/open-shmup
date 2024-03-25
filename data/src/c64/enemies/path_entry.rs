use crate::geometry::EightWayDirection;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct C64EnemyPathEntry {
    direction: EightWayDirection,
    distance: u8,
}
