use crate::prelude::*;
const num_tiles: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn get_map_index_from_coords  (x: i32, y: i32) -> usize {
    let map_index: usize = ((y * SCREEN_WIDTH) + x) as usize;
    map_index
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; num_tiles]
        }
    }

    pub fn is_coords_inside_bounds (&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile (&self, point: Point) -> bool {
        self.is_coords_inside_bounds(point) &&
        self.tiles[get_map_index_from_coords(point.x, point.y)] == TileType::Floor
    }

    pub fn try_point (&self, point: Point) -> Option<usize> {
        if self.is_coords_inside_bounds(point) == true {
            return Some(get_map_index_from_coords(point.x, point.y));
        }
        None
    }

    pub fn render (&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let tile_index: usize = get_map_index_from_coords(x, y);
                if tile_index < self.tiles.len() {
                    match self.tiles[tile_index] {
                        TileType::Wall => {
                            ctx.set(x, y, YELLOW, BLACK, to_cp437('#'));
                        },
                        TileType::Floor => {
                            ctx.set(x, y, PALEGREEN2, BLACK, to_cp437('.'));
                        }
                    }
                }
            }
        }
    }
}

