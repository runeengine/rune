use bevy::app::{App, Plugin, Update};
use bevy::ecs::schedule::IntoScheduleConfigs;

pub struct MyDebugPlugin;
impl Plugin for MyDebugPlugin {
    fn build(&self, app: &mut App) {
        use std::sync::{Arc, Mutex};

        let is_dump = Arc::new(Mutex::new(false));
        let is_dump2 = is_dump.clone();

        std::thread::spawn(move || {
            use crossterm::event::{self, Event, KeyCode};
            use std::time::Duration;
            loop {
                if event::poll(Duration::from_millis(500)).unwrap() {
                    if let Event::Key(key_event) = event::read().unwrap() {
                        if key_event.code == KeyCode::Char('d') {
                            *is_dump2.lock().unwrap() = true;
                        }
                    }
                }
            }
        });

        let dump_cond = move || std::mem::take(&mut *(is_dump.lock().unwrap()));

        app.add_systems(Update, dump_debug_info.run_if(dump_cond));
    }
}

fn dump_debug_info(world: &mut bevy::ecs::world::World) {
    println!("world: {:?}", world);
}
