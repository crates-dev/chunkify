use crate::*;

pub trait ChunkNaming: Send + Sync {
    fn format_chunk_name(&self, file_id: String, chunk_index: usize) -> String;
}

pub trait HandleStrategy: Send + Sync {
    fn handle(
        &self,
        file_name: &str,
        chunk_data: &[u8],
        file_id: &str,
        chunk_index: usize,
        total_chunks: usize,
    ) -> impl Future<Output = ChunkStrategyResult> + Send;
}
