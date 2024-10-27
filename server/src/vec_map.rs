use serde::{ser::SerializeMap, Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VecMap<K, V> where K: Eq {
    vec: Vec<(K, V)>,
}
impl<K, V> Default for VecMap<K, V> where K: Eq {
    fn default() -> Self {
        VecMap::new()
    }
}

impl<K, V> VecMap<K, V> where K: Eq {
    pub fn new() -> Self {
        VecMap { vec: Vec::new() }
    }

    /// returns the old value if the key already exists
    pub fn insert(&mut self, key: K, value: V) -> Option<(K, V)>{

        if let Some((old_key, old_val)) = self.vec.iter_mut().find(|(k, _)| *k == key) {
            Some((
                std::mem::replace(old_key, key), 
                std::mem::replace(old_val, value)
            ))
        }else{
            self.vec.push((key, value));
            None
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_kvp(key).map(|(_, v)| v)
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_kvp_mut(key).map(|(_, v)| v)
    }

    pub fn get_kvp(&self, key: &K) -> Option<(&K, &V)> {
        for (k, v) in &self.vec {
            if k == key {
                return Some((k, v));
            }
        }
        None
    }

    pub fn get_kvp_mut(&mut self, key: &K) -> Option<(&K, &mut V)> {
        for (k, v) in &mut self.vec {
            if k == key {
                return Some((k ,v));
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        let mut index = None;
        for (i, (k, _)) in self.vec.iter().enumerate() {
            if k == key {
                index = Some(i);
                break;
            }
        }
        if let Some(i) = index {
            Some(self.vec.remove(i))
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.vec.iter().map(|(k, v)| (k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.vec.iter_mut().map(|(k, v)| (k as &K, v))
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

    pub fn retain<F>(&mut self, mut f: F) where F: FnMut(&K, &V) -> bool {
        self.vec.retain(|(k, v)| f(k, v));
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.vec.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.vec.iter().map(|(_, v)| v)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.vec.iter().any(|(k, _)| k == key)
    }
}
impl<K, V> IntoIterator for VecMap<K, V> where K: Eq {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}
impl<K,  V> FromIterator<(K, V)> for VecMap<K, V> where K: Eq {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for (k, v) in iter {
            vec.push((k, v));
        }
        VecMap { vec }
    }
}

impl<K, V> Serialize for VecMap<K, V> where K: Eq, K: Serialize, V: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl<'de, K, V> Deserialize<'de> for VecMap<K, V> where K: Eq, K: Deserialize<'de>, V: Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct MapVisitor<K, V> where K: Eq {
            marker: std::marker::PhantomData<VecMap<K, V>>,
        }

        impl <'de, K, V> serde::de::Visitor<'de> for MapVisitor<K, V> where K: Eq, K: Deserialize<'de>, V: Deserialize<'de> {
            type Value = VecMap<K, V>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut vec = Vec::new();
                while let Some((k, v)) = map.next_entry()? {
                    vec.push((k, v));
                }
                Ok(VecMap { vec })
            }
        }

        deserializer.deserialize_map(MapVisitor { marker: std::marker::PhantomData })
    }
}