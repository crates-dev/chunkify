use crate::*;

pub trait ChunkNaming<'a>: Fn(&'a str, usize) -> String + Send + Sync {}

pub trait HandleStrategy<'a>: Send + Sync {
    fn handle(
        &self,
        file_name: &'a str,
        chunk_data: &'a [u8],
        file_id: &'a str,
        chunk_index: usize,
        total_chunks: usize,
    ) -> impl Future<Output = ChunkStrategyResult> + Send;
}
