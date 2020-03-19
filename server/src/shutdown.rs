//! Shutdown behavior.
use crossbeam::Sender;
use fecs::World;

pub fn init(tx: Sender<()>) {
    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    })
    .unwrap();
}

pub fn save_chunks(_world: &mut World) {}

pub fn save_level(_world: &World) {}

pub fn save_player_data(_world: &World) {}