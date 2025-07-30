use crate::*;

/// Trait for generating chunk file names.
///
/// Implementations should provide a function that generates unique names for chunk files.
pub trait ChunkNaming<'a>: Fn(&'a str, usize) -> String + Send + Sync {}

/// Trait for handling chunk operations.
///
/// Defines the interface for saving and merging file chunks.
pub trait HandleStrategy<'a>: Send + Sync {
    /// Saves a chunk of data.
    ///
    /// # Arguments
    ///
    /// - `&'a [u8]` - The chunk data to save.
    /// - `usize` - The chunk index.
    ///
    /// # Returns
    ///
    /// - `impl Future<Output = ChunkStrategyResult>` - Future of the save operation.
    fn save_chunk(
        &self,
        chunk_data: &'a [u8],
        chunk_index: usize,
    ) -> impl Future<Output = ChunkStrategyResult> + Send;

    /// Merges all chunks into the final file.
    ///
    /// # Returns
    ///
    /// - `impl Future<Output = ChunkStrategyResult>` - Future of the merge operation.
    fn merge_chunks(&self) -> impl Future<Output = ChunkStrategyResult> + Send;
}
