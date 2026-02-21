//! LRU缓存实现

use std::collections::HashMap;
use std::hash::Hash;
use parking_lot::Mutex;

/// LRU缓存
pub struct LRUCache<K, V> {
    capacity: usize,
    cache: Mutex<(Vec<K>, HashMap<K, V>)>,
}

impl<K: Clone + Hash + Eq, V: Clone> LRUCache<K, V> {
    /// 创建新的LRU缓存
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cache: Mutex::new((Vec::new(), HashMap::new())),
        }
    }
    
    /// 获取缓存
    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock();
        if let Some(value) = cache.1.get(key).cloned() {
            let key = key.clone();
            cache.0.retain(|k| k != &key);
            cache.0.push(key);
            Some(value)
        } else {
            None
        }
    }
    
    /// 插入缓存
    pub fn put(&self, key: K, value: V) {
        let mut cache = self.cache.lock();
        
        if cache.1.contains_key(&key) {
            cache.0.retain(|k| k != &key);
            cache.0.push(key.clone());
            cache.1.insert(key, value);
            return;
        }
        
        if cache.0.len() >= self.capacity {
            if let Some(old_key) = cache.0.first().cloned() {
                cache.0.remove(0);
                cache.1.remove(&old_key);
            }
        }
        
        cache.0.push(key.clone());
        cache.1.insert(key, value);
    }
    
    /// 移除缓存
    pub fn remove(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock();
        cache.0.retain(|k| k != key);
        cache.1.remove(key)
    }
    
    /// 清空缓存
    pub fn clear(&self) {
        let mut cache = self.cache.lock();
        cache.0.clear();
        cache.1.clear();
    }
    
    /// 获取缓存大小
    pub fn len(&self) -> usize {
        self.cache.lock().0.len()
    }
    
    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
