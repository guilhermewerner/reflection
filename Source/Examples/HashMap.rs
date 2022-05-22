use crate::*;
use std::collections::HashMap as InnerHashMap;
use std::hash::Hash;

#[derive(Reflect)]
pub struct HashMap<K, V>
where
    K: Reflect + Eq + Hash,
    V: Reflect,
{
    inner: InnerHashMap<K, V>,
}

impl<K, V> HashMap<K, V>
where
    K: Reflect + Eq + Hash,
    V: Reflect,
{
    #[function]
    pub fn Insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    #[function]
    pub fn Remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }
}
