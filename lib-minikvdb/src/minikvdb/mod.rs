use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub mod kvdb_value;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MiniKVDB {
    #[cfg(feature = "key_value")]
    pub(crate) kv: Arc<RwLock<crate::kv_store::KVStore>>,
    #[cfg(feature = "list")]
    pub(crate) list: Arc<RwLock<crate::list_store::ListStore>>,
    #[cfg(feature = "map")]
    pub(crate) map: Arc<RwLock<crate::map_store::MapStore>>,
}
