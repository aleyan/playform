use cgmath::{Point, Point3, Vector3};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bounds {
  /// x-coordinate as a multiple of 2^lg_size.
  pub x: i32,
  /// y-coordinate as a multiple of 2^lg_size.
  pub y: i32,
  /// z-coordinate as a multiple of 2^lg_size.
  pub z: i32,
  /// The log_2 of the voxel's size.
  pub lg_size: i16,
}

impl Bounds {
  /// Convenience function to create `Bounds`.
  /// N.B. That the input coordinates should be divided by (2^lg_size) relative to world coords.
  pub fn new(x: i32, y: i32, z: i32, lg_size: i16) -> Bounds {
    let ret =
      Bounds {
        x: x,
        y: y,
        z: z,
        lg_size: lg_size,
      };
    ret
  }

  /// The width of this voxel.
  #[inline(always)]
  pub fn size(&self) -> f32 {
    if self.lg_size >= 0 {
      (1 << self.lg_size) as f32
    } else {
      1.0 / (1 << -self.lg_size) as f32
    }
  }
}

// NOTE: When voxel size and storage become an issue, this should be shrunk to
// be less than pointer-sized. It'll be easier to transfer to the GPU for
// whatever reasons, but also make it possible to shrink the SVO footprint by
// "flattening" the leaf contents and pointers into the same space (the
// low-order bits can be used to figure out which one it is, since pointers
// have three low-order bits set to zero).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Voxel {
  // The voxel is entirely inside or outside the volume. true is inside.
  Volume(bool),
  // The voxel crosses the surface of the volume.
  Surface(SurfaceVoxel),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// Every voxel keeps track of a single vertex, as well as whether its
// lowest-coordinate corner is inside the volume.
// Since we keep track of an "arbitrarily" large world of voxels, we don't
// leave out any corners.
pub struct SurfaceVoxel {
  /// The position of a free-floating vertex on the surface.
  pub inner_vertex: Vertex,

  /// Is this voxel's low corner inside the field?
  pub corner_inside_surface: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vertex {
  pub x: Fracu8,
  pub y: Fracu8,
  pub z: Fracu8,
}

impl Vertex {
  pub fn to_world_vertex(&self, parent: &Bounds) -> Point3<f32> {
    // Relative position of the vertex.
    let local =
      Vector3::new(
        self.x.numerator as f32 / 256.0,
        self.y.numerator as f32 / 256.0,
        self.z.numerator as f32 / 256.0,
      );
    let fparent = Point3::new(parent.x as f32, parent.y as f32, parent.z as f32);
    fparent.add_v(&local).mul_s(parent.size())
  }
}

/// Express a `[0,1)` fraction using a `u8`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Fracu8 {
  // The denominator is 1 << 8.
  pub numerator: u8,
}

impl Fracu8 {
  pub fn of(numerator: u8) -> Fracu8 {
    Fracu8 {
      numerator: numerator,
    }
  }
}
