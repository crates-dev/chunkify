#[tokio::test]
async fn handle() {
    use crate::*;
    let chunk_strategy: ChunkStrategy =
        ChunkStrategy::new("./uploads", |file_id: &str, chunk_index: usize| {
            format!("{file_id}.{chunk_index}")
        });
    let res: ChunkStrategyResult = chunk_strategy
        .handle("test.txt", b"test", "abcdefg", 0, 10)
        .await;
    println!("{:?}", res);
}
