#[tokio::test]
async fn handle() {
    use crate::*;

    let chunk_strategy: ChunkStrategy<'_> = ChunkStrategy::new(
        0,
        "./uploads",
        "abcdefg",
        "test.txt",
        1,
        |file_id: &str, chunk_index: usize| format!("{file_id}.{chunk_index}"),
    )
    .unwrap();
    chunk_strategy.save_chunk(b"test", 0).await.unwrap();
    chunk_strategy.merge_chunks().await.unwrap();
}
