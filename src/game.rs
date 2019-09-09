use {
	amethyst::{
		assets::{PrefabData, ProgressCounter},
		derive::PrefabData,
		ecs::prelude::Entity,
		error::Error,
		prelude::*
	},	
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct MyPrefabData {}

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {}

#[derive(Default)]
pub struct GameState {

}

impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let StateData { world, .. } = data;


	}
}