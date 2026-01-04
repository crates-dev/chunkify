use crate::*;

/// Result type for creating new chunk strategies.
pub type NewChunkStrategyResult<'a> = Result<ChunkStrategy<'a>, ChunkStrategyError>;

/// Result type for chunk strategy operations.
pub type ChunkStrategyResult = Result<(), ChunkStrategyError>;

/// Type alias for chunk status tracking map.
///
/// Combines DashMap with RwLock for concurrent chunk status tracking.
pub(crate) type ChunkStatusMap = DashMap<String, RwLock<Vec<bool>>, BuildHasherDefault<XxHash3_64>>;
