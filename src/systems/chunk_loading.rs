use bevy::prelude::*;
use crate::util::PlayerMoveEvent;

pub(crate) fn log_player_chunk_boundary_crossing(mut last_chunk: Local<IVec3>, mut events: EventReader<PlayerMoveEvent>) {
    for ev in events.iter() {
        *last_chunk = ev.moved_from_chunk();
        if ev.moved_to_chunk() != *last_chunk {
            println!("crossed boundary {} -> {}", *last_chunk, ev.moved_to_chunk());
        }
    }
}