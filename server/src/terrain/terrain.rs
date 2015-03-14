use capnp::{MessageBuilder, MessageReader, ReaderOptions, MallocMessageBuilder};
use noise::Seed;
use std::collections::hash_map::{HashMap, Entry};
use std::iter::range_inclusive;
use std::io::Cursor;
use std::sync::Mutex;

use common::block_position::BlockPosition;
use common::communicate::terrain_block;
use common::entity::EntityId;
use common::id_allocator::IdAllocator;
use common::lod::LODIndex;
use common::stopwatch::TimerSet;

use opencl_context::CL;
use terrain::heightmap::HeightMap;
use terrain::terrain_gen;
use terrain::texture_generator::TerrainTextureGenerator;
use terrain::tree_placer::TreePlacer;

pub const AMPLITUDE: f64 = 64.0;
pub const FREQUENCY: f64 = 1.0 / 64.0;
pub const PERSISTENCE: f64 = 1.0 / 16.0;
pub const LACUNARITY: f64 = 8.0;
pub const OCTAVES: usize = 2;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TerrainType {
  Grass,
  Dirt,
  Stone,
  Wood,
  Leaf,
}

pub struct TerrainMipMesh {
  pub lods: Vec<Option<MallocMessageBuilder>>,
}

/// This struct contains and lazily generates the world's terrain.
pub struct Terrain {
  pub heightmap: HeightMap,
  pub treemap: TreePlacer,
  // all the blocks that have ever been created.
  pub all_blocks: HashMap<BlockPosition, TerrainMipMesh>,
}

impl Terrain {
  pub fn new(terrain_seed: Seed, tree_seed: u32) -> Terrain {
    Terrain {
      heightmap:
        HeightMap::new(terrain_seed, OCTAVES, FREQUENCY, PERSISTENCE, LACUNARITY, AMPLITUDE),
      treemap: TreePlacer::new(tree_seed),
      all_blocks: HashMap::new(),
    }
  }

  // TODO: Allow this to be performed in such a way that self is only briefly locked.
  pub fn load<F, T>(
    &mut self,
    timers: &TimerSet,
    cl: &CL,
    texture_generator: &TerrainTextureGenerator,
    id_allocator: &Mutex<IdAllocator<EntityId>>,
    position: &BlockPosition,
    lod_index: LODIndex,
    f: F,
  ) -> T
    where F: FnOnce(terrain_block::Reader) -> T
  {
    let heightmap = &self.heightmap;
    let treemap = &self.treemap;

    macro_rules! load_lod(
      ($mip_mesh: expr) => ({
        for _ in range_inclusive($mip_mesh.lods.len(), lod_index.0 as usize) {
          $mip_mesh.lods.push(None);
        }
        let mesh = $mip_mesh.lods.get_mut(lod_index.0 as usize).unwrap();
        if mesh.is_none() {
          *mesh = Some(
            terrain_gen::generate_block(
              timers,
              cl,
              id_allocator,
              heightmap,
              texture_generator,
              treemap,
              position,
              lod_index,
            )
          );
        }

        let mesh = mesh.as_ref().unwrap();
        let mesh = mesh.get_root::<terrain_block::Builder>().as_reader();
        f(mesh)
      })
    );

    match self.all_blocks.entry(*position) {
      Entry::Occupied(mut entry) => {
        load_lod!(entry.get_mut())
      },
      Entry::Vacant(entry) => {
        let r = entry.insert(
          TerrainMipMesh {
            lods: Vec::new(),
          }
        );
        load_lod!(r)
      },
    }
  }
}
