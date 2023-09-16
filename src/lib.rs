use bevy::prelude::*;

mod spawner;
pub use spawner::*;

pub struct Module;

impl Plugin for Module {
    fn build(&self, app: &mut App) {
        todo!("Add plugins here");
    }
}
