extern crate shipyard;

use std::sync::Arc;

use parking_lot::Mutex;
use player::Player;
use position::Position;
use shipyard::{Borrow, IntoBorrow, View, World};

mod player;
mod position;

struct Map {
    world: Arc<Mutex<World>>,
}

impl Map {
    pub fn new() -> Self {
        let world = World::new();
        let world = Arc::new(Mutex::new(world));

        Self { world }
    }

    pub fn run_on_world<R>(&self, f: impl FnOnce(View<Player>, View<Position>) -> R) -> R {
        let world_guard = self.world.lock();
        let (v_player, v_wpos) = world_guard
            .borrow::<(View<Player>, View<Position>)>()
            .unwrap();
        f(v_player, v_wpos)
    }

    pub fn run_on_world_generic<'s, Views, R>(&'s self, f: impl FnOnce(Views) -> R) -> R
    where
        Views: IntoBorrow,
        Views::Borrow: Borrow<'s, View = Views>,
    {
        let world_guard = self.world.lock();
        let views: Views = world_guard.borrow().unwrap();
        f(views)
    }
}

fn main() {
    let map = Map::new();

    map.run_on_world(|_v_player, _v_pos| {
        todo!();
    });

    map.run_on_world_generic(|(_v_player, _v_pos): (View<Player>, View<Position>)| {
        todo!();
    });
}
