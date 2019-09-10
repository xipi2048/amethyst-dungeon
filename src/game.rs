use {
	amethyst::{
		animation::{get_animation_set, AnimationControlSet, AnimationSet, EndControl, AnimationCommand},
		assets::{PrefabData, PrefabLoader, ProgressCounter, RonFormat},
		core::transform::Transform,
		derive::PrefabData,
		ecs::{prelude::Entities, Entity, Join, ReadStorage, WriteStorage},
		error::Error,
		prelude::*,
		renderer::{camera::Camera, sprite::SpriteRender},
		window::ScreenDimensions,
		SimpleTrans,
	},
	serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct MyPrefabData {}

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
	Stand
}

#[derive(Default)]
pub struct GameState {
	pub progress_counter: Option<ProgressCounter>,
}

impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let StateData { world, .. } = data;

		self.progress_counter = Some(Default::default());

		let dungeon_prefab = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
			loader.load(
				"prefab/dungeon_tiles.ron",
				RonFormat,
				self.progress_counter.as_mut().unwrap(),
			)
		});

		world.create_entity().with(dungeon_prefab).build();

		initialize_camera(world);
	}

	fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		if let Some(ref progress_counter) = self.progress_counter {
			if progress_counter.is_complete() {
				let StateData { world, .. } = data;

				world.exec(
					|(entities, animation_sets, mut control_sets): (
						Entities,
						ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
						WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
					)| {
						for (entity, animation_set) in (&entities, &animation_sets).join() {
							let control_set = get_animation_set(&mut control_sets, entity).unwrap();

							control_set.add_animation(
								AnimationId::Stand,
								&animation_set.get(&AnimationId::Stand).unwrap(),
								EndControl::Loop(None),
								1.0,
								AnimationCommand::Start
							);
						}
					},
				);
			}
		}

		Trans::None
	}
}

fn initialize_camera(world: &mut World) {
	let (width, height) = {
		let dim = world.read_resource::<ScreenDimensions>();
		(dim.width(), dim.height())
	};

	println!("Init camera with dimensions: {}x{}", width, height);

	let mut camera_transform = Transform::default();
	camera_transform.set_translation_z(1.0);

	world
		.create_entity()
		.with(camera_transform)
		.with(Camera::standard_2d(width, height))
		.build();
}
