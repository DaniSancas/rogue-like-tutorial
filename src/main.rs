use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod component;
pub use component::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;

pub struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();
        player_input(self, ctx);

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() -> rltk::RltkError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };

    gs.ecs.insert(new_map_rooms_and_corridors());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position {
            x: MAX_WIDTH / 2,
            y: MAX_HEIGHT / 2,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gs)
}
