use amethyst::{
    assets::{Loader, AssetStorage},
    core::{
        transform::Transform,
        specs::{
            DenseVecStorage,
        }
    },
    ecs::{Builder, Component},
    prelude::World,
    renderer::{
        Camera,
        Flipped,
        PngFormat,
        Projection,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
    },
    GameData,
    SimpleState,
    SimpleTrans,
    StateData,
    Trans,
};
use nalgebra::{ Vector2, Isometry2 };
use std::ops::DerefMut;
use nphysics2d::volumetric::volumetric::Volumetric;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub struct Pong;

impl SimpleState for Pong {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut physics_world = nphysics2d::world::World::<f32>::new();
        physics_world.set_gravity(Vector2::y() * -1.0);
        let sprite_sheet_handle = load_sprite_sheet(world);
        initialise_paddles(world, sprite_sheet_handle.clone());
        initialise_ball(world, &mut physics_world, sprite_sheet_handle);
        initialise_camera(world);
        world.add_resource(physics_world);
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = data.world;
        let mut physics_world = world.write_resource::<nphysics2d::world::World<f32>>();
        physics_world.step();
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialise_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Create a left plank entity.
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world.create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    // Create right plank entity.
    world.create_entity()
        .with(sprite_render.clone())
        .with(Flipped::Horizontal)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

fn initialise_ball(world: &mut World,
                   physics_world: &mut nphysics2d::world::World<f32>,
                   sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();
    let rigid_body = RigidBody2D::new(physics_world);

    transform.set_xyz(ARENA_HEIGHT / 2.0_, ARENA_WIDTH / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };

    world.create_entity()
        .with(sprite_render.clone())
        .with(transform)
        .with(rigid_body)
        .build();
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let texture_handle = loader.load(
        "texture/pong_spritesheet.png",
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &texture_storage,
    );

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub struct RigidBody2D {
    pub handle: nphysics2d::object::BodyHandle,
}

impl RigidBody2D {
    fn new(world: &mut nphysics2d::world::World<f32>, x: f32, y: f32) -> RigidBody2D {
        let position = Vector2::new(x, y);
        let cuboid = ncollide2d::shape::ShapeHandle::new(ncollide2d::shape::Cuboid::new(Vector2(1.0, 2.0)));
        let local_inertia = cuboid.inertia(1.0);
        let local_center_of_mass = cuboid.center_of_mass();
        let rigid_body_handle = world.add_rigid_body(
            nphysics2d::math::Isometry::new(position, nalgebra::zero()),
            local_inertia,
            local_center_of_mass,
        );
        RigidBody2D {
            handle: rigid_body_handle
        }
    }
}

impl Component for RigidBody2D {
    type Storage = DenseVecStorage<Self>;
}

//pub enum Collider {
//    Sphere(f32),
//}
//
//impl Component for Collider {
//    type Storage = DenseVecStorage<Self>;
//}
