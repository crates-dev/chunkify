/// Errors that can occur during chunking operations.
///
/// Represents various failure scenarios when processing file chunks.
#[derive(Debug)]
pub enum ChunkStrategyError {
    /// Missing file ID header in request.
    MissingFileId,
    /// Invalid chunk index value.
    InvalidChunkIndex,
    /// Missing chunk index header in request.
    MissingChunkIndex,
    /// Invalid total chunks value.
    InvalidTotalChunks,
    /// Missing total chunks header in request.
    MissingTotalChunks,
    /// Missing file name header in request.
    MissingFileName,
    /// Received empty chunk data.
    EmptyChunkData,
    /// Chunk index exceeds total chunks.
    IndexOutOfBounds(usize, usize),
    /// Failed to merge chunks.
    Merge,
    /// Failed to create directory.
    CreateDirectory(String),
    /// Failed to create chunk file.
    CreateChunkFile(String),
    /// Failed to write chunk data.
    WriteChunk(String),
    /// Failed to create output file.
    CreateOutputFile(String),
    /// Failed to read chunk file.
    ReadChunk(String),
    /// Failed to write to output file.
    WriteOutput(String),
}
