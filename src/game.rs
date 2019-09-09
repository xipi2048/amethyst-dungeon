use amethyst::prelude::*;

#[derive(Default)]
pub struct GameState {

}

impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let StateData { world, .. } = data;

		
	}
}