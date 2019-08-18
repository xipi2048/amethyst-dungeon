use amethyst::{
    core::transform::Transform,
    ecs::prelude::*,
    prelude::*,
    renderer::{
        Camera, Projection
    }
};

const VIEW_WIDTH: i32 = 160;
const VIEW_HEIGHT: i32 = 90;

pub struct Dungeon;

pub struct GameObject {
    pub health: f32,
    pub grid_x: i32,
    pub grid_y: i32,
    pub name: String
}

impl Component for GameObject {
    type Storage = VecStorage<Self>;
}

impl SimpleState for Dungeon {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        initialise_hero(world);
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

fn initialise_hero(world: &mut World) {
    let mut transform = Transform::default();
    world
        .create_entity()
        .with(GameObject {
            health: 100.0, 
            grid_x: VIEW_WIDTH / 2, 
            grid_y: VIEW_HEIGHT / 2, 
            name: "Hero".to_string()
        })
        .with(transform)
        .build();
}
