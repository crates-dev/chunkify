use crate::*;

impl<'a, F> ChunkNaming<'a> for F where F: Fn(&'a str, usize) -> String + Send + Sync {}

impl<'a> ChunkStrategy<'a> {
    pub fn new<F>(
        start_chunk_index: usize,
        upload_dir: &'a str,
        file_id: &'a str,
        file_name: &'a str,
        total_chunks: usize,
        file_name_func: F,
    ) -> NewChunkStrategyResult<'a>
    where
        F: ChunkNaming<'a> + 'static,
    {
        if start_chunk_index >= total_chunks {
            return Err(ChunkStrategyError::IndexOutOfBounds(
                start_chunk_index,
                total_chunks,
            ));
        }
        Ok(Self {
            upload_dir,
            start_chunk_index,
            file_id,
            file_name,
            total_chunks,
            file_name_func: Box::new(file_name_func),
        })
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

    pub async fn merge_chunks(&self) -> ChunkStrategyResult {
        let chunks_status: RefMut<'_, String, RwLock<Vec<bool>>> = UPLOADING_FILES
            .entry(self.file_id.to_owned())
            .or_insert_with(|| RwLock::new(vec![false; self.total_chunks]));
        let mut chunks_status: RwLockWriteGuard<'_, Vec<bool>> = chunks_status.write().await;
        let all_chunks_uploaded: bool = chunks_status.iter().all(|&status| status);
        if !all_chunks_uploaded {
            return Err(ChunkStrategyError::Merge);
        }
        chunks_status.clear();
        drop(chunks_status);
        let final_path: String = Path::new(&self.upload_dir)
            .join(self.file_name)
            .to_string_lossy()
            .into_owned();
        let output_file: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&final_path)
            .map_err(|e| ChunkStrategyError::CreateOutputFile(e.to_string()))?;
        let mut writer: BufWriter<File> = BufWriter::new(output_file);
        for i in self.start_chunk_index..self.total_chunks {
            let chunk_path: String = self.get_chunk_path(self.file_id, i);
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
    async fn save_chunk(&self, chunk_data: &'a [u8], chunk_index: usize) -> ChunkStrategyResult {
        if !Path::new(&self.upload_dir).exists() {
            fs::create_dir_all(&self.upload_dir)
                .map_err(|e| ChunkStrategyError::CreateDirectory(e.to_string()))?;
        }
        let chunk_path: String = self.get_chunk_path(self.file_id, chunk_index);
        self.save_chunk(&chunk_path, &chunk_data).await?;
        let chunks_status: RefMut<'_, String, RwLock<Vec<bool>>> = UPLOADING_FILES
            .entry(self.file_id.to_owned())
            .or_insert_with(|| RwLock::new(vec![false; self.total_chunks]));
        let mut chunks_status: RwLockWriteGuard<'_, Vec<bool>> = chunks_status.write().await;
        if chunks_status.len() != self.total_chunks {
            *chunks_status = vec![false; self.total_chunks];
        }
        if chunk_index >= chunks_status.len() {
            return Err(ChunkStrategyError::IndexOutOfBounds(
                chunk_index,
                self.total_chunks,
            ));
        }
        chunks_status[chunk_index] = true;
        Ok(())
    }
}
