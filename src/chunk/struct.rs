use crate::*;

/// Configuration for chunking operations.
///
/// Contains all necessary parameters for performing chunked file operations.
pub struct ChunkStrategy<'a> {
    /// The starting index for chunking operations.
    pub(crate) start_chunk_index: usize,
    /// Directory where chunks will be uploaded.
    pub(crate) upload_dir: &'a str,
    /// Function for generating chunk file names.
    pub(crate) file_name_func: Box<dyn ChunkNaming<'a>>,
    /// Unique identifier for the file being chunked.
    pub(crate) file_id: &'a str,
    /// Original name of the file being chunked.
    pub(crate) file_name: &'a str,
    /// Total number of chunks to create.
    pub(crate) total_chunks: usize,
}
