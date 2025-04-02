use std::ops::{Add, AddAssign};

use rand::Rng;

use crate::vec_map::VecMap;


#[derive(Clone, Debug, Default)]
pub struct WeightMap<K> where K: Eq + Clone {
    pub weight_map: VecMap<K, u8>,
}
///All addition is saturating
impl<K> WeightMap<K> where K: Eq + Clone {
    ///Only returns none if this is empty
    pub fn choose(&self) -> Option<K>{
        if self.is_empty() {return None}
        
        let mut max = 0u32;
        for weight in self.weight_map.iter() {
            max = max.saturating_add(*weight.1 as u32);
        }
        let mut rng = rand::rng();
        let mut choice_index = rng.random_range(0..max);

        for weight in self.weight_map.iter() {
            if *weight.1 as u32 > choice_index {
                return Some(weight.0.clone());
            }
            choice_index = choice_index.saturating_sub(*weight.1 as u32);
        }
        unreachable!("Managed to pick a value that is greater than the sum of the weights");
    }

    pub fn choose_remove(&mut self) -> Option<K>{
        let choice = self.choose();
        if let Some(choice) = &choice {
            self.weight_map.remove(choice);
        }
        choice
    }
    /// Strictly slower than choose_multiple remove
    pub fn choose_multiple(&self, count: usize) -> Vec<Option<K>>{
        self.clone().choose_multiple_remove(count)
    }

    pub fn choose_multiple_remove(&mut self, count: usize) -> Vec<Option<K>>{
        let mut choices = Vec::with_capacity(count);
        for _ in 0..count {
            choices.push(self.choose_remove())
        }
        choices
    }

    pub fn is_empty(&self) -> bool{
        self.weight_map.is_empty()
    }

    /// returns the old value if the key already exists
    pub fn insert(&mut self, key: K, weight: u8) -> Option<(K, u8)>{
        self.weight_map.insert(key, weight)
    }

    /// adds weight to the weight of key, if the key does not have a weight it is added with the specified weight
    /// returns the old value if it exists
    pub fn add(&mut self, key: K, weight: u8) -> Option<(K, u8)>{
        self.weight_map.add(key, weight)
    }

    /// adds weight to the weight of key, if the key does not have a weight nothing happens
    /// returns the old value if it exists
    pub fn add_no_insert(&mut self, key: K, weight: u8) -> Option<(K, u8)>{
        self.weight_map.add_no_insert(key, weight)
    }
}

impl<K> PartialEq for WeightMap<K> where K: Eq + Clone {
    fn eq(&self, other: &Self) -> bool {
        self.weight_map == other.weight_map
    }
}
impl<K> Eq for WeightMap<K> where K: Eq + Clone {}

impl<K> Add<WeightMap<K>> for WeightMap<K> where K: Eq + Clone {
    type Output = WeightMap<K>;
    fn add(self, rhs: WeightMap<K>) -> Self::Output {
        if self.weight_map.len() >= rhs.weight_map.len() {
            let mut sum: VecMap<K, u8> = self.weight_map.clone();
            for map in rhs.weight_map.iter(){
                sum.add(map.0.clone(), *map.1);
            }
            WeightMap{weight_map: sum}
        } else {
            let mut sum: VecMap<K, u8> = rhs.weight_map.clone();
            for map in self.weight_map.iter(){
                sum.add(map.0.clone(), *map.1);
            }
            WeightMap{weight_map: sum}
        }
    }
}
impl<K> AddAssign<WeightMap<K>> for WeightMap<K> where K: Eq + Clone {
    fn add_assign(&mut self, rhs: WeightMap<K>) {
        for map in rhs.weight_map.iter(){
            self.weight_map.add(map.0.clone(), *map.1);
        }
    }
}

impl<K> Add<(K, u8)> for WeightMap<K> where K: Eq + Clone {
    type Output = WeightMap<K>;
    fn add(self, rhs: (K, u8)) -> Self::Output {
        let mut clone = self.clone();
        clone.weight_map.add(rhs.0, rhs.1);
        clone
    }
}
impl<K> AddAssign<(K, u8)> for WeightMap<K> where K: Eq + Clone {
    fn add_assign(&mut self, rhs: (K, u8)) {
        self.weight_map.add(rhs.0, rhs.1);
    }
}

impl<K> From<Vec<K>> for WeightMap<K> where K: Eq + Clone {
    fn from(value: Vec<K>) -> Self {
        WeightMap{weight_map: VecMap::from_iter(
            value.iter().map(|k| (k.clone(), 1u8))
        )}
    }
}

impl<K> From<VecMap<K, u8>> for WeightMap<K> where K: Eq + Clone {
    fn from(value: VecMap<K, u8>) -> Self {
        WeightMap{weight_map: value}
    }
}