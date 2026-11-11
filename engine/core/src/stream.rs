//! Chargement du monde en arrière-plan : des fils génèrent et maillent les chunks autour du joueur, les plus proches d'abord.
use crate::blocks::{Id, idx, CH, CX};
use crate::gen::WorldGen;
use crate::mesher::{far_mesh, Mesh, Mesher};
use crate::player::Voxels;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

type Key = (i32, i32);
/// Niveau spécial : l'anneau de relief lointain, centré sur ce chunk.
pub const FAR: u8 = 255;
pub struct Built { pub key: Key, pub lod: u8, pub mesh: Mesh, pub voxels: Arc<Vec<Id>>, pub ms: f64 }
struct Shared { queue: Mutex<Vec<(Key, u8)>>, wake: Condvar, voxels: Mutex<HashMap<Key, Arc<Vec<Id>>>>, edits: Mutex<HashMap<Key, HashMap<u32, Id>>>, stop: AtomicBool }
pub struct Preset { pub radius: i32, pub lod1: f64, pub lod2: f64 }
/// Réglages du PC : plus loin que le Haut du prototype, puisque la marge est là.
pub const HIGH: Preset = Preset { radius: 10, lod1: 2.3, lod2: 4.6 };
/// Pour les cartes puissantes : 512 blocs de terrain détaillé autour du joueur.
pub const ULTRA: Preset = Preset { radius: 16, lod1: 3.5, lod2: 8.0 };

pub struct Streamer { sh: Arc<Shared>, rx: Receiver<Built>, threads: Vec<JoinHandle<()>>, pub radius: i32, lods: (f64, f64), loaded: HashMap<Key, u8>, pending: HashSet<(Key, u8)>, vox: HashMap<Key, Arc<Vec<Id>>>, last: Option<Key>, far_at: Option<Key> }

fn voxels_of(sh: &Shared, gen: &WorldGen, k: Key) -> Arc<Vec<Id>> {
    if let Some(v) = sh.voxels.lock().unwrap().get(&k) { return v.clone(); }
    let mut v = gen.gen_world(k.0, k.1);
    // Les modifications du joueur priment toujours sur la génération. Même ordre de verrous que `edit`.
    let mut all = sh.voxels.lock().unwrap();
    if let Some(e) = sh.edits.lock().unwrap().get(&k) { for (i, id) in e { v[*i as usize] = *id; } }
    all.entry(k).or_insert(Arc::new(v)).clone()
}
fn worker(sh: Arc<Shared>, tx: Sender<Built>, seed: i32) {
    let gen = WorldGen::new(seed); let mut m = Mesher::new();
    loop {
        let job = { let mut q = sh.queue.lock().unwrap(); loop { if sh.stop.load(Ordering::Relaxed) { return; } if let Some(j) = q.pop() { break j; } q = sh.wake.wait(q).unwrap(); } };
        let t = std::time::Instant::now();
        let (key, lod) = job;
        if lod == FAR {
            let (opaque, n_o) = far_mesh(&gen, key.0, key.1);
            if tx.send(Built { key, lod, mesh: Mesh { opaque, n_o, ..Default::default() }, voxels: Arc::new(Vec::new()), ms: t.elapsed().as_secs_f64() * 1000.0 }).is_err() { return; }
            continue;
        }
        let mut mesh = m.mesh_chunk(&mut |x, z| voxels_of(&sh, &gen, (x, z)), key.0, key.1, lod);
        let voxels = voxels_of(&sh, &gen, key);
        if lod < 2 { crate::batisse::dress(&gen, key.0, key.1, &voxels, &mut mesh); }
        if tx.send(Built { key, lod, mesh, voxels, ms: t.elapsed().as_secs_f64() * 1000.0 }).is_err() { return; }
    }
}
impl Streamer {
    pub fn new(seed: i32, p: &Preset, threads: usize) -> Self {
        let sh = Arc::new(Shared { queue: Mutex::new(Vec::new()), wake: Condvar::new(), voxels: Mutex::new(HashMap::new()), edits: Mutex::new(HashMap::new()), stop: AtomicBool::new(false) });
        let (tx, rx) = channel();
        let threads = (0..threads.max(1)).map(|_| { let (s, t) = (sh.clone(), tx.clone()); std::thread::spawn(move || worker(s, t, seed)) }).collect();
        Streamer { sh, rx, threads, radius: p.radius, lods: (p.lod1, p.lod2), loaded: HashMap::new(), pending: HashSet::new(), vox: HashMap::new(), last: None, far_at: None }
    }
    fn lod_for(&self, d: f64) -> u8 { if d < self.lods.0 { 0 } else if d < self.lods.1 { 1 } else { 2 } }
    /// À appeler chaque image. Rend les chunks prêts, et ceux à oublier parce que trop loin.
    pub fn update(&mut self, x: f64, z: f64) -> (Vec<Built>, Vec<Key>) {
        let n = CX as f64; let c = ((x / n).floor() as i32, (z / n).floor() as i32);
        let mut gone = Vec::new();
        if self.last != Some(c) {
            self.last = Some(c); let r = self.radius; let mut jobs = Vec::new();
            for dz in -r..=r { for dx in -r..=r {
                let d = ((dx * dx + dz * dz) as f64).sqrt(); if d > r as f64 + 0.5 { continue; }
                let k = (c.0 + dx, c.1 + dz); let lod = self.lod_for(d);
                if self.loaded.get(&k) != Some(&lod) && !self.pending.contains(&(k, lod)) { jobs.push((d, k, lod)); }
            } }
            jobs.sort_by(|a, b| b.0.total_cmp(&a.0)); // le plus proche en dernier : il sort le premier
            let far = |k: &Key, m: f64| (((k.0 - c.0).pow(2) + (k.1 - c.1).pow(2)) as f64).sqrt() > m;
            let lim = r as f64 + 1.5;
            for k in self.loaded.keys().filter(|k| far(k, lim)).copied().collect::<Vec<_>>() { self.loaded.remove(&k); self.vox.remove(&k); gone.push(k); }
            self.sh.voxels.lock().unwrap().retain(|k, _| !far(k, lim + 2.5));
            let mut q = self.sh.queue.lock().unwrap(); q.clear();
            self.pending.clear();
            for (_, k, l) in jobs { q.push((k, l)); self.pending.insert((k, l)); }
            // L'horizon se recentre quand le joueur s'en éloigne. Dernier de la file : il part en premier.
            if self.far_at.is_none_or(|f| (f.0 - c.0).abs().max((f.1 - c.1).abs()) >= 4) { self.far_at = Some(c); q.push((c, FAR)); self.pending.insert((c, FAR)); }
            drop(q); self.sh.wake.notify_all();
        }
        let mut out = Vec::new();
        while let Ok(b) = self.rx.try_recv() {
            self.pending.remove(&(b.key, b.lod));
            if b.lod == FAR { out.push(b); continue; }
            let d = (((b.key.0 - c.0).pow(2) + (b.key.1 - c.1).pow(2)) as f64).sqrt();
            if d > self.radius as f64 + 1.5 { continue; }
            // Une copie déjà modifiée ne doit pas être écrasée par un maillage parti plus tôt.
            self.loaded.insert(b.key, b.lod); self.vox.entry(b.key).or_insert_with(|| b.voxels.clone()); out.push(b);
        }
        (out, gone)
    }
    /// Pose un bloc. La modification survit au déchargement, et les chunks touchés sont remaillés aussitôt.
    pub fn edit(&mut self, x: i32, y: i32, z: i32, id: Id) -> bool { self.edit_many(&[(x, y, z, id)]) == 1 }
    /// Plusieurs blocs d'un coup, chaque chunk touché n'étant remaillé qu'une fois. Rend le nombre de blocs changés.
    pub fn edit_many(&mut self, list: &[(i32, i32, i32, Id)]) -> usize {
        let n = CX as i32; let mut done = 0; let mut near = HashSet::new(); let mut main = HashSet::new();
        {
            let mut all = self.sh.voxels.lock().unwrap(); let mut edits = self.sh.edits.lock().unwrap();
            for &(x, y, z, id) in list {
                if !(0..CH as i32).contains(&y) { continue; }
                let k = (x.div_euclid(n), z.div_euclid(n)); let i = idx(x.rem_euclid(n) as usize, y as usize, z.rem_euclid(n) as usize);
                let Some(v) = self.vox.get_mut(&k) else { continue };
                edits.entry(k).or_default().insert(i as u32, id); if let Some(c) = all.get_mut(&k) { Arc::make_mut(c)[i] = id; }
                Arc::make_mut(v)[i] = id; done += 1; main.insert(k);
                // La lumière porte à 8 blocs : les voisins proches du bord changent aussi.
                let (lx, lz) = (x.rem_euclid(n), z.rem_euclid(n));
                for dz in -1..=1 { for dx in -1..=1 {
                    if (dx == -1 && lx >= 8) || (dx == 1 && lx < n - 8) || (dz == -1 && lz >= 8) || (dz == 1 && lz < n - 8) || (dx, dz) == (0, 0) { continue; }
                    near.insert((k.0 + dx, k.1 + dz));
                } }
            }
        }
        let c = main.iter().next().map(|k| self.last.unwrap_or(*k));
        let Some(c) = c else { return 0 }; let mut q = self.sh.queue.lock().unwrap();
        let mut todo: Vec<(Key, bool)> = near.into_iter().filter(|k| !main.contains(k)).map(|k| (k, false)).collect(); todo.extend(main.into_iter().map(|k| (k, true))); // les chunks modifiés sortent les premiers
        for (key, must) in todo {
            if self.loaded.remove(&key).is_none() && !must { continue; }
            let lod = self.lod_for((((key.0 - c.0).pow(2) + (key.1 - c.1).pow(2)) as f64).sqrt());
            q.push((key, lod)); self.pending.insert((key, lod));
        }
        drop(q); self.sh.wake.notify_all(); done
    }
    pub fn edits(&self) -> HashMap<Key, HashMap<u32, Id>> { self.sh.edits.lock().unwrap().clone() }
    pub fn set_edits(&mut self, e: HashMap<Key, HashMap<u32, Id>>) { *self.sh.edits.lock().unwrap() = e; }
    /// Le chunk de ce point est-il prêt ?
    pub fn voxel_ready(&self, x: f64, z: f64) -> bool { let n = CX as f64; self.vox.contains_key(&((x / n).floor() as i32, (z / n).floor() as i32)) }
    pub fn waiting(&self) -> usize { self.pending.len() }
    pub fn loaded(&self) -> usize { self.loaded.len() }
}
impl Voxels for Streamer {
    fn get(&self, x: i32, y: i32, z: i32) -> Option<Id> {
        if !(0..CH as i32).contains(&y) { return Some(0); }
        let n = CX as i32; let v = self.vox.get(&(x.div_euclid(n), z.div_euclid(n)))?;
        Some(v[idx(x.rem_euclid(n) as usize, y as usize, z.rem_euclid(n) as usize)])
    }
}
impl Drop for Streamer {
    fn drop(&mut self) { self.sh.stop.store(true, Ordering::Relaxed); self.sh.wake.notify_all(); for t in self.threads.drain(..) { let _ = t.join(); } }
}
