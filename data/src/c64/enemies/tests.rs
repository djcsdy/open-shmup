use crate::c64::enemies::spawn_at_tilemap_position;
use crate::c64::enemies::spawn_at_tilemap_position::C64SpawnEnemyAtTilemapPosition;

#[test]
fn spawn_enemy_at_tilemap_position_has_expected_size() {
    assert_eq!(
        spawn_at_tilemap_position::layout::SIZE,
        Some(C64SpawnEnemyAtTilemapPosition::SIZE_BYTES)
    );
}
