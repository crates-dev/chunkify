use crate::*;

#[derive(Debug)]
pub enum ChunkStrategyError {
    MissingFileId,
    InvalidChunkIndex,
    MissingChunkIndex,
    InvalidTotalChunks,
    MissingTotalChunks,
    MissingFileName,
    EmptyChunkData,
    CreateDirectory(String),
    CreateChunkFile(String),
    WriteChunk(String),
    CreateOutputFile(String),
    ReadChunk(String),
    WriteOutput(String),
}

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
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for ChunkStrategyError {}

impl From<ChunkStrategyError> for Vec<u8> {
    fn from(error: ChunkStrategyError) -> Self {
        error.to_string().into_bytes()
    }
}
