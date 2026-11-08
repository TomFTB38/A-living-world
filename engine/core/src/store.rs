//! Voxels générés, plus les modifications du joueur. La génération n'écrase jamais une modification.
use crate::{blocks::CX, gen::WorldGen};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

pub struct ChunkStore { pub gen: WorldGen, cache: HashMap<(i32, i32), Arc<Vec<u8>>>, order: VecDeque<(i32, i32)>, edits: HashMap<(i32, i32), HashMap<u32, u8>>, limit: usize }
impl ChunkStore {
    pub fn new(seed: i32, limit: usize) -> Self { ChunkStore { gen: WorldGen::new(seed), cache: HashMap::new(), order: VecDeque::new(), edits: HashMap::new(), limit } }
    pub fn edit(&mut self, cx: i32, cz: i32, i: u32, id: u8) {
        self.edits.entry((cx, cz)).or_default().insert(i, id);
        if let Some(c) = self.cache.get_mut(&(cx, cz)) { Arc::make_mut(c)[i as usize] = id; }
    }
    pub fn get(&mut self, cx: i32, cz: i32) -> Arc<Vec<u8>> {
        if let Some(c) = self.cache.get(&(cx, cz)) { return c.clone(); }
        let mut v = self.gen.gen_world(cx, cz);
        if let Some(e) = self.edits.get(&(cx, cz)) { for (i, id) in e { v[*i as usize] = *id; } }
        let c = Arc::new(v); self.cache.insert((cx, cz), c.clone()); self.order.push_back((cx, cz));
        while self.cache.len() > self.limit { if let Some(k) = self.order.pop_front() { self.cache.remove(&k); } }
        c
    }
    /// Bloc en coordonnées du monde.
    pub fn block(&mut self, x: i32, y: i32, z: i32) -> u8 {
        if !(0..crate::blocks::CH as i32).contains(&y) { return 0; }
        let n = CX as i32; let (cx, cz) = (x.div_euclid(n), z.div_euclid(n));
        self.get(cx, cz)[crate::blocks::idx(x.rem_euclid(n) as usize, y as usize, z.rem_euclid(n) as usize)]
    }
}
