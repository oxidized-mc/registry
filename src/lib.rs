//! Block, item, and biome registries with O(1) state ID lookup and generated data.
//!
//! Block data is generated at compile time from embedded vanilla JSON
//! (see `generated`).  Provides O(1) lookup of block states by numeric ID
//! and blocks/items by name.

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod biome_registry;
mod block;
mod block_registry;
mod constants;
mod error;
pub(crate) mod generated;
mod item;
mod item_generated;
mod item_registry;
mod tags;

pub use biome_registry::{PLAINS_BIOME_ID, biome_count, biome_id_to_name, biome_name_to_id};
pub use block::{BlockDef, BlockStateEntry, BlockStateFlags, BlockStateId, PropertyDef};
pub use block_registry::BlockRegistry;
pub use constants::*;
pub use error::RegistryError;
pub use item::{Item, ItemStack};
pub use item_registry::ItemRegistry;
pub use tags::{BlockTags, TagSet};
