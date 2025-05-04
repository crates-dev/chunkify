use crate::*;

pub trait ChunkNaming<'a>: Fn(&'a str, usize) -> String + Send + Sync {}

pub trait HandleStrategy<'a>: Send + Sync {
    fn save_chunk(
        &self,
        chunk_data: &'a [u8],
        chunk_index: usize,
    ) -> impl Future<Output = ChunkStrategyResult> + Send;
}
