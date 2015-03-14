//! Common entity datatypes.

use std::default::Default;
use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(RustcDecodable, RustcEncodable)]
/// Unique ID for a loaded entity.
pub struct EntityId(pub u32);

impl Default for EntityId {
  fn default() -> EntityId {
    EntityId(0)
  }
}

impl Add<u32> for EntityId {
  type Output = EntityId;

  fn add(self, rhs: u32) -> EntityId {
    let EntityId(i) = self;
    EntityId(i + rhs)
  }
}
