use std::ops::{Add, AddAssign};

use rand::Rng;

use crate::vec_map::VecMap;


#[derive(Clone, Debug)]
pub struct WeightMap<K> where K: Eq {
    pub weight_maps: VecMap<K, u8>,
}

impl<K> WeightMap<K> where K: Eq + Copy {
    ///Only returns none if this is empty
    pub fn choose(&self) -> Option<K>{
        if self.is_empty() {return None;}
        
        let mut max = 0u32;
        for weight in self.weight_maps.iter() {
            max += *weight.1 as u32;
        }
        let mut rng = rand::rng();
        let mut choice_index = rng.random_range(0..max);

        for weight in self.weight_maps.iter() {
            if *weight.1 as u32 > choice_index {
                return Some(weight.0.clone());
            }
            choice_index -= *weight.1 as u32;
        }
        panic!("Managed to pick a value that is greater than the sum of the weights");
    }

    pub fn choose_remove(&mut self) -> Option<K>{
        if self.is_empty() {return None;}
        
        let mut max = 0u32;
        for weight in self.weight_maps.iter() {
            max += *weight.1 as u32;
        }
        let mut rng = rand::rng();
        let mut choice_index = rng.random_range(0..max);
        
        let mut key = None;

        for weight in self.weight_maps.clone().into_iter() {
            if weight.1 as u32 > choice_index {
                key = Some(weight.0);
                break;
            }
            choice_index -= weight.1 as u32;
        }
        let Some(key) = key else {panic!("Managed to pick a value that is greater than the sum of the weights");};
        return self.weight_maps.remove(&key).map(|x|x.0);
    }

    pub fn choose_multiple(&self, count: usize) -> Vec<Option<K>>{
        return self.clone().choose_multiple_remove(count);
    }

    pub fn choose_multiple_remove(&mut self, count: usize) -> Vec<Option<K>>{
        let mut choices = Vec::with_capacity(count);
        if self.is_empty() {
            choices.fill(None);
            return choices;
        }
        if count < self.weight_maps.len(){
            for _ in 0..self.weight_maps.len() {
                choices.push(self.choose_remove())
            }
            choices.fill(None);
            return choices;
        }
        for _ in 0..self.weight_maps.len() {
            choices.push(self.choose_remove())
        }
        return choices;
    }

    pub fn is_empty(&self) -> bool{
        return self.weight_maps.is_empty();
    }

    /// returns the old value if the key already exists
    pub fn insert(&mut self, key: K, weight: u8) -> Option<(K, u8)>{
        self.weight_maps.insert(key, weight)
    }

    /// adds weight to the weight of key, if the key does not have a weight it is added with the specified weight
    /// returns the old value if it exists
    pub fn insert_add(&mut self, key: K, weight: u8) -> Option<(K, u8)>{
        self.weight_maps.insert_add(key, weight)
    }
    
    /// adds weight to the weight of key, if the key does not have a weight nothing happens
    /// returns the old value if it exists
    pub fn add(&mut self, key: K, weight: u8) -> Option<(K, u8)>{
        self.weight_maps.add(key, weight)
    }
}
impl<K> PartialEq for WeightMap<K> where K: Eq {
    fn eq(&self, other: &Self) -> bool {
        self.weight_maps == other.weight_maps
    }
}
impl<K> Eq for WeightMap<K> where K: Eq {}

impl<K> Add<WeightMap<K>> for WeightMap<K> where K: Eq + Copy {
    type Output = WeightMap<K>;
    fn add(self, rhs: WeightMap<K>) -> Self::Output {
        let mut sum: VecMap<K, u8> = VecMap::with_capacity(self.weight_maps.len().max(rhs.weight_maps.len()));
        
        for map in self.weight_maps.iter(){
            sum.insert(*map.0, *map.1);
        }

        for map in rhs.weight_maps.iter(){
            sum.insert_add(*map.0, *map.1);
        }

        return WeightMap{weight_maps: sum};
    }
}

impl<K> AddAssign<WeightMap<K>> for WeightMap<K> where K: Eq + Copy {
    fn add_assign(&mut self, rhs: WeightMap<K>) {
        for map in rhs.weight_maps.iter(){
            self.weight_maps.insert_add(*map.0, *map.1);
        }
    }
}

impl<K> From<Vec<K>> for WeightMap<K> where K: Eq + Copy {
    fn from(value: Vec<K>) -> Self {
        return WeightMap{weight_maps: VecMap::from_iter(
            value.iter().map(|k| (*k, 1u8))
        )}
    }
}

impl<K> From<VecMap<K, u8>> for WeightMap<K> where K: Eq + Copy {
    fn from(value: VecMap<K, u8>) -> Self {
        return WeightMap{weight_maps: value}
    }
}
