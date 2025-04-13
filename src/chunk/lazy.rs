use crate::*;

pub static UPLOADING_FILES: Lazy<
    Arc<DashMap<String, RwLock<Vec<bool>>, BuildHasherDefault<XxHash3_64>>>,
> = Lazy::new(|| Arc::new(DashMap::with_hasher(BuildHasherDefault::default())));
