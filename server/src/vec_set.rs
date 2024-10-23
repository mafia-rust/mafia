use macros::vec_set;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecSet<T> {
    vec: Vec<T>,
}

impl<T> Default for VecSet<T> {
    fn default() -> Self {
        VecSet::new()
    }
}

impl<T> FromIterator<T> for VecSet<T> where T: PartialEq{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = VecSet::new();
        for value in iter {
            set.insert(value);
        }
        set
    }
}

impl<T> VecSet<T> {
    pub fn new() -> Self {
        VecSet {
            vec: Vec::new(),
        }
    }
    pub fn new_one(value: T) -> Self where T: PartialEq{
        vec_set!(value)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        VecSet {
            vec: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, value: T) where T: PartialEq{
        if !self.vec.contains(&value) {
            self.vec.push(value);
        }
    }

    pub fn intersection(&self, other: &VecSet<T>) -> VecSet<T> where T: PartialEq + Clone{
        let mut new_set = VecSet::new();
        for value in &self.vec {
            if other.contains(value) {
                new_set.insert(value.clone());
            }
        }
        new_set
    }

    pub fn union(&self, other: &VecSet<T>) -> VecSet<T> where T: PartialEq + Clone{
        let mut new_set = VecSet::new();
        for value in &self.vec {
            new_set.insert(value.clone());
        }
        for value in &other.vec {
            new_set.insert(value.clone());
        }
        new_set
    }

    pub fn difference(&self, other: &VecSet<T>) -> VecSet<T> where T: PartialEq + Clone{
        let mut new_set = VecSet::new();
        for value in &self.vec {
            if !other.contains(value) {
                new_set.insert(value.clone());
            }
        }
        new_set
    }

    pub fn symmetric_difference(&self, other: &VecSet<T>) -> VecSet<T> where T: PartialEq + Clone{
        let mut new_set = VecSet::new();
        for value in &self.vec {
            if !other.contains(value) {
                new_set.insert(value.clone());
            }
        }
        for value in &other.vec {
            if !self.contains(value) {
                new_set.insert(value.clone());
            }
        }
        new_set
    }

    pub fn is_disjoint(&self, other: &VecSet<T>) -> bool where T: PartialEq{
        for value in &self.vec {
            if other.contains(value) {
                return false;
            }
        }
        true
    }

    pub fn is_subset(&self, other: &VecSet<T>) -> bool where T: PartialEq{
        for value in &self.vec {
            if !other.contains(value) {
                return false;
            }
        }
        true
    }

    pub fn is_superset(&self, other: &VecSet<T>) -> bool where T: PartialEq{
        for value in &other.vec {
            if !self.contains(value) {
                return false;
            }
        }
        true
    }

    pub fn to_vec(&self) -> Vec<T> where T: Clone{
        self.vec.clone()
    }

    pub fn to_hash_set(&self) -> std::collections::HashSet<T> where T: Clone + std::hash::Hash + Eq{
        self.vec.iter().cloned().collect()
    }

    pub fn from_vec(vec: Vec<T>) -> Self where T: PartialEq{
        let mut set = VecSet::new();
        for value in vec {
            set.insert(value);
        }
        set
    }

    pub fn retain(&mut self, f: impl Fn(&T) -> bool) {
        self.vec.retain(f);
    }

    pub fn remove(&mut self, value: &T) where T: PartialEq{
        self.vec.retain(|x| x != value);
    }

    pub fn contains(&self, value: &T) -> bool where T: PartialEq{
        self.vec.contains(value)
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

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.vec.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.vec.into_iter()
    }

    pub fn dedup(&mut self) where T: PartialEq{
        self.vec.dedup();
    }
}

pub mod macros {
    macro_rules! vec_set {
        (
            $($x:expr),*
        ) => {{
            let mut set = VecSet::new();
            $(set.insert($x);)*
            set
        }};
    }
    pub(crate) use vec_set;
}