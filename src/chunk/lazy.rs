use crate::*;

/// Global map tracking upload status of file chunks.
///
/// Uses DashMap for concurrent access and RwLock for tracking chunk status.
pub static UPLOADING_FILES: Lazy<
    Arc<DashMap<String, RwLock<Vec<bool>>, BuildHasherDefault<XxHash3_64>>>,
> = Lazy::new(|| Arc::new(DashMap::with_hasher(BuildHasherDefault::default())));
