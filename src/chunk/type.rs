use crate::*;

/// Result type for creating new chunk strategies.
pub type NewChunkStrategyResult<'a> = Result<ChunkStrategy<'a>, ChunkStrategyError>;

/// Result type for chunk strategy operations.
pub type ChunkStrategyResult = Result<(), ChunkStrategyError>;
