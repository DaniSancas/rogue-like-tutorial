use rltk::{RandomNumberGenerator, Rltk, RGB};
use std::cmp::{max, min};

use super::rect::Rect;

pub const MAX_WIDTH: i32 = 80;
pub const MAX_HEIGHT: i32 = 50;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

pub const fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAX_WIDTH as usize) + x as usize
}

/// Makes a map with solid boundaries and 40 randomly placed walls.
///
/// No guarantees that it won't look awful.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; MAX_WIDTH as usize * MAX_HEIGHT as usize];

    // Make the boundaries walls
    for x in 0..MAX_WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, MAX_HEIGHT - 1)] = TileType::Wall;
    }

    for y in 0..MAX_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(MAX_WIDTH - 1, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..(MAX_WIDTH * MAX_HEIGHT / 10) {
        let x = rng.roll_dice(1, MAX_WIDTH - 1);
        let y = rng.roll_dice(1, MAX_HEIGHT - 1);
        let idx = xy_idx(x, y);
        if idx != xy_idx(MAX_WIDTH / 2, MAX_HEIGHT / 2) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;
    
    let mut map = vec![TileType::Wall; MAX_WIDTH as usize * MAX_HEIGHT as usize];

    let mut rooms: Vec<Rect> = Vec::new();


    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x: i32 = rng.roll_dice(1, MAX_WIDTH - w - 1) - 1;
        let y: i32 = rng.roll_dice(1, MAX_HEIGHT - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);

        let mut ok = true;
        for other_room in &rooms {
            if new_room.intersect(other_room) {
                ok = false;
            }
        }

        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);

                }
            }

            rooms.push(new_room);
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => ctx.set(
                x,
                y,
                RGB::from_f32(0.5, 0.5, 0.5),
                RGB::from_f32(0., 0., 0.),
                rltk::to_cp437('.'),
            ),
            TileType::Wall => ctx.set(
                x,
                y,
                RGB::from_f32(0.0, 1.0, 0.0),
                RGB::from_f32(0., 0., 0.),
                rltk::to_cp437('#'),
            ),
        }

        // Move the coordinates
        x += 1;
        if x >= MAX_WIDTH {
            x = 0;
            y += 1;
        }
    }
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        apply_floor_tile(map, x, y);
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        apply_floor_tile(map, x, y);
    }
}

fn apply_floor_tile(map: &mut [TileType], x: i32, y: i32) {
    let idx = xy_idx(x, y);
    if idx > 0 && idx < MAX_WIDTH as usize * MAX_HEIGHT as usize {
        map[idx] = TileType::Floor;
    }
}
