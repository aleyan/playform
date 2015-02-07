use color::Color4;
use common::*;
use gl::types::*;
use id_allocator::IdAllocator;
use lod_map::{LOD, OwnerId};
use mob;
use nalgebra::{Vec3, Pnt3, Norm};
use nalgebra;
use ncollide::bounding_volume::{AABB, AABB3};
use physics::Physics;
use shaders;
use state::{App, EntityId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use surroundings_loader::SurroundingsLoader;
use terrain::terrain;
use yaglw::gl_context::GLContext;

fn center(bounds: &AABB3<f32>) -> Pnt3<GLfloat> {
  (*bounds.mins() + bounds.maxs().to_vec()) / (2.0 as GLfloat)
}

pub fn make_mobs<'a, 'b:'a>(
  gl: &'a mut GLContext,
  physics: &mut Physics,
  id_allocator: &mut IdAllocator<EntityId>,
  owner_allocator: &mut IdAllocator<OwnerId>,
  shader: &shaders::color::ColorShader<'b>,
) -> (HashMap<EntityId, Rc<RefCell<mob::Mob<'b>>>>, mob::MobBuffers<'b>) {
  let mut mobs = HashMap::new();
  let mut mob_buffers = mob::MobBuffers::new(gl, shader);

  fn mob_behavior(world: &App, mob: &mut mob::Mob) {
    let to_player = center(world.get_bounds(world.player.id)) - center(world.get_bounds(mob.id));
    if nalgebra::norm(&to_player) < 2.0 {
      mob.behavior = wait_for_distance;
    }

    fn wait_for_distance(world: &App, mob: &mut mob::Mob) {
      let to_player = center(world.get_bounds(world.player.id)) - center(world.get_bounds(mob.id));
      if nalgebra::norm(&to_player) > 8.0 {
        mob.behavior = follow_player;
      }
    }

    fn follow_player(world: &App, mob: &mut mob::Mob) {
      let to_player = center(world.get_bounds(world.player.id)) - center(world.get_bounds(mob.id));
      if to_player.sqnorm() < 4.0 {
        mob.behavior = wait_to_reset;
        mob.speed = Vec3::new(0.0, 0.0, 0.0);
      } else {
        mob.speed = to_player / 2.0 as GLfloat;
      }
    }

    fn wait_to_reset(world: &App, mob: &mut mob::Mob) {
      let to_player = center(world.get_bounds(world.player.id)) - center(world.get_bounds(mob.id));
      if nalgebra::norm(&to_player) >= 2.0 {
        mob.behavior = mob_behavior;
      }
    }
  }

  add_mob(
    gl,
    physics,
    &mut mobs,
    &mut mob_buffers,
    id_allocator,
    owner_allocator,
    Pnt3::new(0.0, terrain::AMPLITUDE as f32, -1.0),
    mob_behavior
  );

  (mobs, mob_buffers)
}

fn add_mob(
  gl: &mut GLContext,
  physics: &mut Physics,
  mobs: &mut HashMap<EntityId, Rc<RefCell<mob::Mob>>>,
  mob_buffers: &mut mob::MobBuffers,
  id_allocator: &mut IdAllocator<EntityId>,
  owner_allocator: &mut IdAllocator<OwnerId>,
  low_corner: Pnt3<GLfloat>,
  behavior: mob::Behavior,
) {
  // TODO: mob loader instead of pushing directly to gl buffers

  let id = id_allocator.allocate();
  let bounds = AABB::new(low_corner, low_corner + Vec3::new(1.0, 2.0, 1.0 as GLfloat));

  let mob =
    mob::Mob {
      position: (*bounds.mins() + bounds.maxs().to_vec()) / 2.0,
      speed: Vec3::new(0.0, 0.0, 0.0),
      behavior: behavior,
      id: id,
      solid_boundary:
        SurroundingsLoader::new(owner_allocator.allocate(), 1, Box::new(|&: _| LOD::Placeholder)),
    };
  let mob = Rc::new(RefCell::new(mob));

  mob_buffers.push(gl, id, &to_triangles(&bounds, &Color4::of_rgba(1.0, 0.0, 0.0, 1.0)));

  physics.insert_misc(id, bounds);
  mobs.insert(id, mob);
}