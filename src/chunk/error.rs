use crate::*;

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

/// Provides display formatting for chunk strategy errors.
impl fmt::Display for ChunkStrategyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ChunkStrategyError::MissingFileId => "Missing X-File-Id header",
            ChunkStrategyError::InvalidChunkIndex => "Invalid X-Chunk-Index header",
            ChunkStrategyError::MissingChunkIndex => "Missing X-Chunk-Index header",
            ChunkStrategyError::InvalidTotalChunks => "Invalid X-Total-Chunks header",
            ChunkStrategyError::MissingTotalChunks => "Missing X-Total-Chunks header",
            ChunkStrategyError::MissingFileName => "Missing X-File-Name header",
            ChunkStrategyError::EmptyChunkData => "Empty chunk data",
            ChunkStrategyError::CreateDirectory(msg) => {
                &format!("Failed to create directory: {}", msg)
            }
            ChunkStrategyError::CreateChunkFile(msg) => {
                &format!("Failed to create chunk file: {}", msg)
            }
            ChunkStrategyError::WriteChunk(msg) => &format!("Failed to write chunk: {}", msg),
            ChunkStrategyError::CreateOutputFile(msg) => {
                &format!("Failed to create output file: {}", msg)
            }
            ChunkStrategyError::ReadChunk(msg) => &format!("Failed to read chunk: {}", msg),
            ChunkStrategyError::WriteOutput(msg) => {
                &format!("Failed to write to output file: {}", msg)
            }
            ChunkStrategyError::Merge => &format!("Failed to complete the file merge operation"),
            ChunkStrategyError::IndexOutOfBounds(chunk_index, total_chunks) => &format!(
                "Index {} out of bounds(total: {})",
                chunk_index, total_chunks
            ),
        };
        write!(f, "{}", message)
    }
}

/// Marks ChunkStrategyError as a standard error type.
impl std::error::Error for ChunkStrategyError {}

/// Converts ChunkStrategyError to a byte vector.
///
/// Used for error responses in HTTP handlers.
impl From<ChunkStrategyError> for Vec<u8> {
    fn from(error: ChunkStrategyError) -> Self {
        error.to_string().into_bytes()
    }
}
