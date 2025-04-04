use std::collections::HashMap;
use std::sync::LazyLock;
use futures_util::task::AtomicWaker;
use rand_seeder::Seeder;
use rand::rngs::SmallRng;
use rand::{rng, Rng, SeedableRng};
use crate::vec_map::VecMap;
use std::sync::Mutex;
pub static mut RNG_HANDLER: Mutex<RngHandler> = Mutex::new(RngHandler::new());
pub struct RngHandler {
    pub rngs: HashMap<u64, (SmallRng, String)>,
    pub default: SmallRng,
}


impl RngHandler {
    fn new() -> RngHandler {
        RngHandler {rngs: HashMap::new(), default: SmallRng::from_os_rng()}
    }
    pub fn new_rng(key: u64, seed: String) -> Option<(SmallRng, String)> {
        unsafe {RNG_HANDLER.rngs.insert(key, (Seeder::from(seed).into_rng(), seed))}
    }
    pub fn get_seed(key: u64) -> Option<String>{
        unsafe {RNG_HANDLER.rngs.get(&key)}.map(|s|s.1.clone())
    }
    pub fn get_rng<'a>(key: Option<u64>) -> &'a mut SmallRng {
        match key {
            Some(key) => match unsafe {RNG_HANDLER.rngs.get_mut(&key)} {
                Some(rng) => &mut rng.0,
                None => unsafe {&mut RNG_HANDLER.default},
            },
            None => unsafe {&mut RNG_HANDLER.default},
        }
    }
}