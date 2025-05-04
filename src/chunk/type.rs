use crate::*;

pub type NewChunkStrategyResult<'a> = Result<ChunkStrategy<'a>, ChunkStrategyError>;
pub type ChunkStrategyResult = Result<(), ChunkStrategyError>;

pub struct ChunkStrategy<'a> {
    pub(crate) start_chunk_index: usize,
    pub(crate) upload_dir: &'a str,
    pub(crate) file_name_func: Box<dyn ChunkNaming<'a>>,
    pub(crate) file_id: &'a str,
    pub(crate) file_name: &'a str,
    pub(crate) total_chunks: usize,
}
