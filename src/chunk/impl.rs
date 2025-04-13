use crate::*;

impl<'a, F> ChunkNaming<'a> for F where F: Fn(&'a str, usize) -> String + Send + Sync {}

impl<'a> ChunkStrategy<'a> {
    pub fn new<F>(upload_dir: &'a str, file_name_func: F) -> Self
    where
        F: ChunkNaming<'a> + 'static,
    {
        Self {
            upload_dir,
            file_name_func: Box::new(file_name_func),
        }
    }

    fn get_chunk_json_path(&self, file_id: &'a str, chunk_index: usize) -> String {
        (self.file_name_func)(file_id, chunk_index)
    }

    fn get_chunk_path(&self, file_id: &'a str, chunk_index: usize) -> String {
        Path::new(&self.upload_dir)
            .join(self.get_chunk_json_path(file_id, chunk_index))
            .to_string_lossy()
            .into_owned()
    }

    async fn save_chunk(&self, chunk_path: &str, chunk_data: &[u8]) -> ChunkStrategyResult {
        async_write_to_file(chunk_path, chunk_data)
            .await
            .map_err(|e| {
                ChunkStrategyError::WriteChunk(format!(
                    "Failed to write chunk to {}: {}",
                    chunk_path, e
                ))
            })?;
        Ok(())
    }

    async fn merge_chunks(
        &self,
        file_id: &'a str,
        file_name: &'a str,
        total_chunks: usize,
    ) -> ChunkStrategyResult {
        let final_path: String = Path::new(&self.upload_dir)
            .join(file_name)
            .to_string_lossy()
            .into_owned();
        let output_file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&final_path)
            .map_err(|e| ChunkStrategyError::CreateOutputFile(e.to_string()))?;
        let mut writer: BufWriter<File> = BufWriter::new(output_file);
        for i in 0..total_chunks {
            let chunk_path: String = self.get_chunk_path(file_id, i);
            let chunk_data: Vec<u8> = async_read_from_file(&chunk_path).await.map_err(|e| {
                ChunkStrategyError::ReadChunk(format!(
                    "Failed to read chunk from {}: {}",
                    chunk_path, e
                ))
            })?;
            writer
                .write_all(&chunk_data)
                .map_err(|e| ChunkStrategyError::WriteOutput(e.to_string()))?;
            let _ = fs::remove_file(&chunk_path);
        }
        Ok(())
    }
}

impl<'a> HandleStrategy<'a> for ChunkStrategy<'a> {
    async fn handle(
        &self,
        file_name: &'a str,
        chunk_data: &'a [u8],
        file_id: &'a str,
        chunk_index: usize,
        total_chunks: usize,
    ) -> ChunkStrategyResult {
        if !Path::new(&self.upload_dir).exists() {
            fs::create_dir_all(&self.upload_dir)
                .map_err(|e| ChunkStrategyError::CreateDirectory(e.to_string()))?;
        }
        let chunk_path: String = self.get_chunk_path(file_id, chunk_index);
        self.save_chunk(&chunk_path, &chunk_data).await?;
        let chunks_status = UPLOADING_FILES
            .entry(file_id.to_owned())
            .or_insert_with(|| RwLock::new(vec![false; total_chunks]));
        let mut chunks_status: RwLockWriteGuard<'_, Vec<bool>> = chunks_status.write().await;
        if chunks_status.len() != total_chunks {
            *chunks_status = vec![false; total_chunks];
        }
        chunks_status[chunk_index] = true;
        let all_chunks_uploaded: bool = chunks_status.iter().all(|&status| status);
        drop(chunks_status);
        if all_chunks_uploaded {
            return self.merge_chunks(&file_id, &file_name, total_chunks).await;
        }
        Ok(())
    }
}
