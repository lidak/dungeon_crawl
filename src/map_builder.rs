use crate::prelude::*;

const ROOMS_NUM: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub start: Point,
    pub rooms: Vec<Rect>
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            start: Point::zero()
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.start = mb.rooms[0].center();
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < ROOMS_NUM {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10)
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = get_map_index_from_coords(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_corridor(&mut self, y1:i32, y2:i32, x:i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2) ..= max(y1, y2) {
            if let Some(idx) = self.map.try_point(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_corridor(&mut self, x1:i32, x2:i32, y: i32) {
        use std::cmp::{min, max};

        for x in min(x1, x2) ..= max(x1, x2) {
            if let Some(idx) = self.map.try_point(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();

        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let next = rooms[i].center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_corridor(prev.x, next.x, prev.y);
                self.apply_vertical_corridor(prev.y, next.y, next.x);
            } else {
                self.apply_horizontal_corridor(prev.x, next.x, prev.y);
                self.apply_vertical_corridor(prev.y, next.y, next.x);
            }
        }
    }
}