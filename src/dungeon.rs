use amethyst::{
    animation::{
        AnimationSetPrefab
    },
    core::transform::Transform,
    ecs::prelude::*,
    prelude::*,
    renderer::{
        Camera, Projection,
        sprite::{prefab::SpriteScenePrefab, SpriteRender}
    }
};

pub struct Dungeon;

#[derive(Default, Deserialize, Serialize)]
pub struct Prefab<T> {
    entities: Vec<PrefabEntity<T>>
}

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    Stand, Walk, Jump
}

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct SpriteData {
    sprite_scene: SpriteScenePrefab,
    animation_set, AnimationSetPrefab<AnimationId, SpriteRender>
}

impl Component for GameObject {
    type Storage = VecStorage<Self>;
}

impl SimpleState for Dungeon {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, 
            VIEW_WIDTH as f32,            
            0.0,
            VIEW_HEIGHT as f32,
        )))
        .with(transform)
        .build();
}
