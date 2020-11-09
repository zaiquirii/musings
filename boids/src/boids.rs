use amethyst::prelude::*;
use amethyst::{SimpleState, StateData, GameData, StateEvent, SimpleTrans, Trans};
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode};
use amethyst::core::Transform;
use amethyst::renderer::{Camera, SpriteSheet, Texture, ImageFormat, SpriteSheetFormat, SpriteRender};
use amethyst::assets::{Handle, Loader, AssetStorage};
use amethyst::core::math::Vector3;
use crate::components::Boid;
use crate::config::FlockConfig;
use amethyst::utils::application_root_dir;
use crate::utils::deg_to_rad;

pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

#[derive(Default)]
pub struct Flock {}

impl SimpleState for Flock {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    let (arena_size, boid_count) = {
      let config = world.read_resource::<FlockConfig>();
      (config.arena_size, config.boid_count)
    };

    let sprite_sheet = load_sprite_sheet(world);
    let sprite_render = SpriteRender::new(sprite_sheet, 0);
    for i in 0..boid_count {
      let pct = i as f32 / boid_count as f32;
      create_boid(
        world,
        sprite_render.clone(),
        [arena_size[0] * pct, arena_size[1] * pct],
        360.0 * pct
      )
    }
    init_camera(world, arena_size);
  }

  fn handle_event(
    &mut self,
    data: StateData<'_, GameData<'_, '_>>,
    event: StateEvent,
  ) -> SimpleTrans {
    if let StateEvent::Window(event) = &event {
      if is_close_requested(&event) ||
        is_key_down(&event, VirtualKeyCode::Escape) {
        Trans::Quit
      } else {
        Trans::None
      }
    } else {
      Trans::None
    }
  }
}

fn init_camera(world: &mut World, arena_size: [f32; 2]) {
  let [width, height] = arena_size;
  let mut transform = Transform::default();
  transform.set_translation_xyz(width / 2.0, height / 2.0, 1.0);
  world
    .create_entity()
    .with(Camera::standard_2d(width, height))
    .with(transform)
    .build();
}

fn create_boid(
  world: &mut World,
  sprite: SpriteRender,
  location: [f32; 2],
  rotation: f32) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(location[0], location[1], 0.0);
  transform.set_rotation_2d(deg_to_rad(rotation));
  transform.set_scale(Vector3::new(0.1, 0.1, 1.0));
  world
    .create_entity()
    .with(sprite)
    .with(transform)
    .with(Boid::default())
    .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "textures/triangle.png",
      ImageFormat::default(),
      (),
      &texture_storage
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "textures/triangle.ron",
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store
  )
}