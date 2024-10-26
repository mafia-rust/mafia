use serde::{Deserialize, Serialize};

use crate::vec_map::VecMap;

#[derive(Clone, Debug)]
pub struct VecSet<K> where K: Eq {
    vec: VecMap<K, ()>,
}
impl<K> Default for VecSet<K> where K: Eq {
    fn default() -> Self {
        VecSet::new()
    }
}

impl <K> VecSet<K> where K: Eq {
    pub fn new() -> Self {
        VecSet { vec: VecMap::new() }
    }

    pub fn insert(&mut self, key: K) -> bool {
        self.vec.insert(key, ()).is_none()
    }

    pub fn extend(&mut self, keys: impl IntoIterator<Item = K>) {
        for key in keys {
            self.insert(key);
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.vec.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<K> {
        self.vec.remove(key).map(|(k, _)| k)
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.vec.iter().map(|(k, _)| k)
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    pub fn retain<F>(&mut self, mut f: F) where F: FnMut(&K) -> bool {
        self.vec.retain(|k, _| f(k));
    }

    pub fn elements(&self) -> impl Iterator<Item = &K> {
        self.vec.keys()
    }
}
impl<K> IntoIterator for VecSet<K> where K: Eq {
    type Item = K;
    type IntoIter = std::iter::Map<std::vec::IntoIter<(K, ())>, fn((K, ())) -> K>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter().map(|(k, _)| k)
    }
}
impl<K> FromIterator<K> for VecSet<K> where K: Eq {
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let mut set = VecSet::new();
        for key in iter {
            set.insert(key);
        }
        set
    }
}

impl<K> Serialize for VecSet<K> where K: Eq, K: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.vec.keys().collect::<Vec<_>>().serialize(serializer)
    }
}
impl<'de, K> Deserialize<'de> for VecSet<K> where K: Eq, K: Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let vec: Vec<K> = Deserialize::deserialize(deserializer)?;
        Ok(vec.into_iter().collect())
    }
}