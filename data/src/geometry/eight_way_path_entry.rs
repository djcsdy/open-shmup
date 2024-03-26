use crate::geometry::EightWayDirection;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct EightWayPathEntry {
    direction: EightWayDirection,
    distance: u8,
}
