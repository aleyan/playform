// Thanks to http://procworld.blogspot.com/2011/02/space-colonization.html
// for the basic idea used to generate these trees!

use cgmath::{Point, Point2, Point3, EuclideanVector, Vector, Vector2, Vector3};
use cgmath::Aabb3;
use rand::{Rng, SeedableRng, IsaacRng};
use std::cmp::{partial_min, partial_max};
use std::collections::VecDeque;
use std::num::Float;
use std::sync::Mutex;

use common::entity::EntityId;
use common::id_allocator::IdAllocator;
use common::lod::LODIndex;
use common::terrain_block::{BLOCK_WIDTH, LOD_QUALITY};

const TREE_NODES: [f32; 4] = [1.0/16.0, 1.0/16.0, 1.0/64.0, 1.0/128.0];
const MAX_BRANCH_LENGTH: [f32; 4] = [4.0, 4.0, 8.0, 16.0];
const LEAF_RADIUS: [f32; 4] = [1.5, 1.5, 8.0, 16.0];

#[inline(always)]
fn fmod(mut dividend: f64, divisor: f64) -> f64 {
  dividend -= divisor * (dividend / divisor).floor();
  if dividend < 0.0 || dividend >= divisor{
    // clamp
    dividend = 0.0;
  }
  dividend
}

fn sqr_distance(p1: &Point3<f32>, p2: &Point3<f32>) -> f32 {
  let d = p1.sub_p(p2);
  d.x*d.x + d.y*d.y + d.z*d.z
}

/// Use one-octave perlin noise local maxima to place trees.
pub struct TreePlacer {
  seed: u32,
}

impl TreePlacer {
  pub fn new(seed: u32) -> TreePlacer {
    TreePlacer {
      seed: seed,
    }
  }

  fn rng_at(&self, center: &Point3<f32>, mut seed: Vec<u32>) -> IsaacRng {
    let center = center.mul_s((LOD_QUALITY[0] as f32) / (BLOCK_WIDTH as f32));
    seed.push_all(&[self.seed, center.x as u32, center.z as u32]);
    SeedableRng::from_seed(seed.as_slice())
  }

  pub fn should_place_tree(&self, center: &Point3<f32>) -> bool {
    let mut rng = self.rng_at(center, vec!(0));
    rng.next_u32() > 0xFF7FFFFF
  }

  pub fn place_tree(
    &self,
    mut center: Point3<f32>,
    id_allocator: &Mutex<IdAllocator<EntityId>>,
    vertex_positions: &mut Vec<Point3<f32>>,
    vertex_normals: &mut Vec<Vector3<f32>>,
    pixel_coords: &mut Vec<Point2<f32>>,
    triangle_ids: &mut Vec<EntityId>,
    block_bounds: &mut Vec<(EntityId, Aabb3<f32>)>,
    lod_index: LODIndex,
  ) {
    let lod_index = lod_index.0 as usize;
    let normals = [
      Vector3::new(-1.0, -1.0, -1.0).normalize(),
      Vector3::new(-1.0, -1.0,  1.0).normalize(),
      Vector3::new( 1.0, -1.0,  1.0).normalize(),
      Vector3::new( 1.0, -1.0, -1.0).normalize(),
      Vector3::new(-1.0,  1.0, -1.0).normalize(),
      Vector3::new(-1.0,  1.0,  1.0).normalize(),
      Vector3::new( 1.0,  1.0,  1.0).normalize(),
      Vector3::new( 1.0,  1.0, -1.0).normalize(),
    ];

    let wood_coords = Point2::new(0.0, 3.0);
    let leaf_coords = Point2::new(0.0, 4.0);

    let mut place_side = |
        corners: &[Point3<f32>],
        coords: Point2<f32>,
        idx1: usize,
        idx2: usize,
        idx3: usize,
        idx4: usize,
      | {
        let n1 = normals[idx1];
        let n2 = normals[idx2];
        let n3 = normals[idx3];
        let n4 = normals[idx4];

        let v1 = corners[idx1];
        let v2 = corners[idx2];
        let v3 = corners[idx3];
        let v4 = corners[idx4];

        vertex_positions.push_all(&[v1, v2, v4, v1, v4, v3]);
        vertex_normals.push_all(&[n1, n2, n4, n1, n4, n3]);
        pixel_coords.push_all(&[coords, coords, coords, coords, coords, coords]);

        let minx = partial_min(v1.x, v2.x).unwrap();
        let maxx = partial_max(v1.x, v2.x).unwrap();
        let minz = partial_min(v1.z, v2.z).unwrap();
        let maxz = partial_max(v1.z, v2.z).unwrap();

        let bounds =
          Aabb3::new(
            Point3::new(minx, v1.y, minz),
            Point3::new(maxx, v3.y, maxz),
          );

        let id1 = id_allocator.lock().unwrap().allocate();
        let id2 = id_allocator.lock().unwrap().allocate();
        triangle_ids.push_all(&[id1, id2]);

        block_bounds.push((id1, bounds.clone()));
        block_bounds.push((id2, bounds));
      };

    let mut place_block = |
        coords: Point2<f32>,
        low_center: &Point3<f32>, low_radius: f32,
        high_center: &Point3<f32>, high_radius: f32,
      | {
        let corners = [
          low_center.add_v(&Vector3::new(-low_radius, 0.0, -low_radius)),
          low_center.add_v(&Vector3::new(-low_radius, 0.0,  low_radius)),
          low_center.add_v(&Vector3::new( low_radius, 0.0,  low_radius)),
          low_center.add_v(&Vector3::new( low_radius, 0.0, -low_radius)),
          high_center.add_v(&Vector3::new(-high_radius, 0.0, -high_radius)),
          high_center.add_v(&Vector3::new(-high_radius, 0.0,  high_radius)),
          high_center.add_v(&Vector3::new( high_radius, 0.0,  high_radius)),
          high_center.add_v(&Vector3::new( high_radius, 0.0, -high_radius)),
        ];

        place_side(&corners, coords, 0, 1, 4, 5);
        place_side(&corners, coords, 1, 2, 5, 6);
        place_side(&corners, coords, 2, 3, 6, 7);
        place_side(&corners, coords, 3, 0, 7, 4);
        place_side(&corners, coords, 1, 0, 2, 3);
        place_side(&corners, coords, 4, 5, 7, 6);
      };

    let mut rng = self.rng_at(&center, vec!(1));
    let mass = (rng.next_u32() as f32) / (0x10000 as f32) / (0x10000 as f32);
    let mass = 0.1 + mass * 0.9;
    let mass = partial_min(partial_max(0.0, mass).unwrap(), 1.0).unwrap();

    let sqr_mass = mass * mass;
    let trunk_radius = sqr_mass * 2.0;
    let trunk_height = sqr_mass * 16.0;

    {
      place_block(
        wood_coords,
        &center, trunk_radius,
        &(center.add_v(&Vector3::new(0.0, trunk_height, 0.0))), trunk_radius,
      );
      center = center.add_v(&Vector3::new(0.0, trunk_height, 0.0));
    }

    {
      let crown_radius = sqr_mass * 16.0;
      let crown_height = sqr_mass * 16.0;
      let crown_width = crown_radius * 2.0;

      let mut points: Vec<Point3<_>> = {
        let n_points =
          (crown_width * crown_width * crown_height * TREE_NODES[lod_index]) as u32;
        range(0, n_points)
        .map(|_| {
          let x = rng.next_u32();
          let y = rng.next_u32();
          let z = rng.next_u32();
          Point3::new(
            fmod(x as f64, crown_width as f64) as f32 - crown_radius,
            fmod(y as f64, crown_height as f64) as f32,
            fmod(z as f64, crown_width as f64) as f32 - crown_radius,
          )
        })
        .map(|p| p.add_v(&center.to_vec()))
        .collect()
      };

      let mut fringe = VecDeque::new();
      fringe.push_back((center, trunk_radius));

      while let Some((center, thickness)) = fringe.pop_front() {
        let mut i = 0;
        let mut any_branches = false;

        let radius = MAX_BRANCH_LENGTH[lod_index];
        while i < points.len() {
          if sqr_distance(&center, &points[i]) <= radius * radius {
            let next_thickness = thickness * 0.6;
            if center.y < points[i].y {
              place_block(wood_coords, &center, thickness, &points[i], next_thickness);
            } else {
              place_block(wood_coords, &points[i], next_thickness, &center, thickness);
            }
            fringe.push_back((points[i], next_thickness));
            points.swap_remove(i);
            any_branches = true;
          } else {
            i += 1;
          }
        }

        if !any_branches {
          // A node with no branches gets leaves.

          let radius = LEAF_RADIUS[lod_index];
          let height = 2.0 * radius;

          place_block(
            leaf_coords,
            &center, radius,
            &(center.add_v(&Vector3::new(0.0, height, 0.0))), radius,
          );
        }
      }
    }
  }
}
