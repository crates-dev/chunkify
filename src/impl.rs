use crate::*;

impl<F> ChunkNaming for F
where
    F: for<'a> Fn(String, usize) -> String + Send + Sync,
{
    fn format_chunk_name(&self, file_id: String, chunk_index: usize) -> String {
        self(file_id, chunk_index)
    }
}

impl<'a> ChunkStrategy<'a> {
    pub fn new<F>(upload_dir: &'a str, file_name_func: F) -> Self
    where
        F: ChunkNaming + 'static,
    {
        Self {
            uploading_files: Arc::new(RwLock::new(HashMap::new())),
            upload_dir,
            file_name_func: Box::new(file_name_func),
        }
    }

    fn get_chunk_json_path(&self, file_id: &str, chunk_index: usize) -> String {
        self.file_name_func
            .format_chunk_name(file_id.to_owned(), chunk_index)
    }

    fn get_chunk_path(&self, file_id: &str, chunk_index: usize) -> String {
        Path::new(&self.upload_dir)
            .join(self.get_chunk_json_path(file_id, chunk_index))
            .to_string_lossy()
            .into_owned()
    }

    async fn save_chunk(&self, chunk_path: &str, chunk_data: &[u8]) -> ChunkStrategyResult {
        println!("{chunk_path}");
        async_write_to_file(chunk_path, chunk_data)
            .await
            .map_err(|e| ChunkStrategyError::WriteChunk(e.to_string()))?;
        Ok(())
    }

    async fn merge_chunks(
        &self,
        file_id: &str,
        file_name: &str,
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
            let chunk_data: Vec<u8> =
                fs::read(&chunk_path).map_err(|e| ChunkStrategyError::ReadChunk(e.to_string()))?;
            writer
                .write_all(&chunk_data)
                .map_err(|e| ChunkStrategyError::WriteOutput(e.to_string()))?;
            let _ = fs::remove_file(&chunk_path);
        }
        Ok(())
    }
}

impl<'a> HandleStrategy for ChunkStrategy<'a> {
    async fn handle(
        &self,
        file_name: &str,
        chunk_data: &[u8],
        file_id: &str,
        chunk_index: usize,
        total_chunks: usize,
    ) -> ChunkStrategyResult {
        if !Path::new(&self.upload_dir).exists() {
            fs::create_dir_all(&self.upload_dir)
                .map_err(|e| ChunkStrategyError::CreateDirectory(e.to_string()))?;
        }
        let chunk_path: String = self.get_chunk_path(file_id, chunk_index);
        self.save_chunk(&chunk_path, &chunk_data).await?;
        let mut uploading_files: RwLockWriteGuard<'_, HashMap<String, Vec<bool>>> =
            self.uploading_files.write().await;
        let chunks_status: &mut Vec<bool> = uploading_files
            .entry(file_id.to_owned())
            .or_insert_with(|| vec![false; total_chunks]);
        if chunks_status.len() != total_chunks {
            *chunks_status = vec![false; total_chunks];
        }
        if chunk_index < chunks_status.len() {
            chunks_status[chunk_index] = true;
        }
        let all_chunks_uploaded: bool = chunks_status.iter().all(|&status| status);
        if all_chunks_uploaded {
            uploading_files.remove(file_id);
            drop(uploading_files);
            return self.merge_chunks(&file_id, &file_name, total_chunks).await;
        }
        Ok(())
    }
}
