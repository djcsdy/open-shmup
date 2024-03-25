use crate::screen::Screen;
use crate::tile::{TileMap, TileSet};
use open_shmup_data::c64::stage::{C64StageSetData, EndBehaviour, ScrollType};
use open_shmup_data::geometry::{Point, Rect};
use open_shmup_data::palette::SrgbPalette;
use open_shmup_data::GameData;
use web_sys::CanvasRenderingContext2d;

pub struct Stage {
    stage_data: C64StageSetData,
    tile_map: TileMap,
    stage_index: usize,
    scroll_position: i32,
    scroll_frame_count: u16,
}

impl Stage {
    pub async fn new(palette: &SrgbPalette<16>, data: &GameData) -> Self {
        let tile_set = TileSet::new(palette, &data.tile_set).await;
        let tile_map = TileMap::new(&tile_set, &data.background_scroll_data);
        Self {
            stage_data: data.stage_data.clone(),
            tile_map,
            stage_index: 0,
            scroll_position: 200,
            scroll_frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.scroll_frame_count += 1;

        let stage = self.stage_data.get(self.stage_index);
        let scroll_type = stage.scroll_type().read();

        if scroll_type.contains(ScrollType::SCROLL) && !scroll_type.contains(ScrollType::PUSH) {
            let frames_per_pixel = if scroll_type.contains(ScrollType::FAST) {
                1
            } else {
                2
            };
            if self.scroll_frame_count >= frames_per_pixel {
                self.scroll_position += 1;
                self.scroll_frame_count = 0
            }
        }

        let stage_complete = if scroll_type.contains(ScrollType::SCROLL) {
            self.scroll_position > stage.map_rect().height()
        } else {
            self.scroll_frame_count > stage.duration().read() as u16 * 50
        };

        if stage_complete {
            let end_behaviour = stage.end_behaviour().read();
            self.stage_index = if end_behaviour == EndBehaviour::Loop {
                0
            } else {
                self.stage_index + 1
            };
            self.scroll_frame_count = 0;
            self.scroll_position = if end_behaviour == EndBehaviour::Continue {
                0
            } else {
                200
            };
        }
    }

    pub fn draw(&self, screen: &Screen, context: &CanvasRenderingContext2d) {
        if self.scroll_position > 8 {
            let stage = self.stage_data.get(self.stage_index);
            let map_rect = stage.map_rect();
            self.tile_map.draw(
                context,
                map_rect.intersection(screen.play_area.move_top_left_to(Point {
                    x: 0,
                    y: map_rect.bottom() - self.scroll_position + 8,
                })),
                screen.play_area.top_left(),
            );
        }

        if self.stage_index > 0 {
            let stage = self.stage_data.get(self.stage_index - 1);
            let map_rect = stage.map_rect();
            if self.scroll_position < 8 {
                self.tile_map.draw(
                    context,
                    map_rect.intersection(screen.play_area.move_top_left_to(Point {
                        x: 0,
                        y: map_rect.top() - self.scroll_position + 8,
                    })),
                    screen.play_area.top_left(),
                );
            } else if self.scroll_position < 192 {
                self.tile_map.draw(
                    context,
                    map_rect.intersection(Rect::from_top_left_width_height(
                        Point {
                            x: 0,
                            y: map_rect.top(),
                        },
                        screen.play_area.width(),
                        screen.play_area.height() - self.scroll_position,
                    )),
                    screen.play_area.top_left()
                        + Point {
                            x: 0,
                            y: self.scroll_position - 8,
                        },
                );
            }
        }
    }
}
