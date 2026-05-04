
use std::sync::{Arc};

use dashmap::DashMap;
use once_cell::sync::OnceCell;


pub static COUNTER_CACHE: OnceCell<Arc<CounterCache>> = OnceCell::new();

#[derive(Clone, Debug)]
pub struct CounterCache {
    pub counters: DashMap<String, i64>,
    pub peak_counters: DashMap<String, i64>,
}

impl CounterCache {
    pub fn new() -> Self {
        Self {
            counters: {
                let map = DashMap::new();
                map.insert("tph".to_string(), 0);
                map.insert("tps".to_string(), 0);
                map
            },
            peak_counters: {
                let map = DashMap::new();
                map.insert("tps".to_string(), 0);
                map
            },
        }
    }
}