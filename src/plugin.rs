//! The Bevy plugin for Bones3 for setting up all systems and components
//! required to use Bones3 in a Bevy app.

use std::marker::PhantomData;

use bevy::prelude::{App, Plugin};

use crate::anchor::{load_chunks, ChunkAnchor};
use crate::storage::{BlockData, ChunkLoadEvent, ChunkUnloadEvent, VoxelWorld};

/// The root plugin for implementing all Bones Cubed logic components and
/// systems.
///
/// This plugin includes components for creating VoxelWorld components, which
/// are infinite grids that store a single type of data. Normally, all static
/// block data will be stored in this data type and attached to the world. When
/// initializing the Bones3Plugin, the type `T` specifies what type of block
/// data will be stored within the voxel world components. A new instance of
/// this plugin must be defined for each block data type that is defined.
///
/// This plugin also implements systems and component support for adding chunk
/// anchors to entities. This will allow for voxel worlds to automatically load
/// and unload chunk based on the location and effect radius of chunk anchors
/// within the world.
///
/// Note that the "R" generic refers to the maximum radius that may be provided
/// by a chunk anchor. This is used for internal cache purposes. Anchors are
/// allowed to have a smaller value, but larger radius values will be ignored.
/// The value R refers to the cache size, so larger values might add a higher
/// memory and performance overhead.
#[derive(Debug, Default)]
pub struct Bones3Plugin<const R: u8, T: BlockData> {
    /// Phantom data for T.
    _phantom: PhantomData<T>,
}

impl<const R: u8, T: BlockData> Plugin for Bones3Plugin<R, T> {
    fn build(&self, app: &mut App) {
        app.register_type::<VoxelWorld<T>>()
            .register_type::<ChunkAnchor>()
            .add_event::<ChunkLoadEvent>()
            .add_event::<ChunkUnloadEvent>()
            .add_system(load_chunks::<R, T>);
    }
}