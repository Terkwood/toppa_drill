use resources::ingame::planet::{ChunkIndex, TileIndex};

/// TODO: Encompass error messages for the `failed` variants?
/// Different events regarding [`Chunk`s](struct.Chunk.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ChunkEvent {
    /// An event requesting a chunk to be added to the [`planet`s](struct.Planet.html) `chunk`-HashMap,
    /// either by loading from disk, or generating a new chunk.
    RequestingLoad(ChunkIndex),
    /// Sent if a chunk was successfully loaded/generated.
    Loaded(ChunkIndex),
    /// Sent if a chunk could not be loaded nor generated for the given [`ChunkIndex`](struct.ChunkIndex.html).
    FailedLoad(ChunkIndex),

    /// Counterpart to the [`RequestingLoad`](enum.ChunkEvent.html#variant.RequestingLoad).
    /// Saves a chunk to disk.
    RequestingUnload(ChunkIndex),
    /// Sent if a chunk was sucessfully saved on disk.
    Unloaded(ChunkIndex),
    /// Sent if a chunk could not be saved to disk.
    FailedUnload(ChunkIndex),
}

/// TODO: Encompass error messages for the `failed` variants?
/// Different events regarding `Tile`-entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TileEvent {
    /// Every `tile` that has not been visited by a player yet is covered under a `Fog of War`.
    /// This event should be sent when a player has vision on a `tile`.
    RequestingUncover(TileIndex),
    /// Sent if a `tile`'s `Fog of War` has been sucessfully removed.
    Uncovered(TileIndex),
    /// Sent if a `tile`'s `Fog of War` could not be removed.
    FailedUncover(TileIndex),

    /// When a player drills into a `tile` it should be deleted.
    RequestDeletion(TileIndex),
    /// Sent if a `tile` could not be deleted.
    FailedDelete(TileIndex),
}
