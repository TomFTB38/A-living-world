//! Climat et biomes : deux champs très lents, température et humidité. Chaque point du monde est un mélange de biomes,
//! jamais un biome pur à frontière nette : c'est ce qui rend les transitions invisibles.
use crate::det::hypot2;
use crate::noise::Noise;
#[derive(Clone, Copy, PartialEq, Debug)] pub enum Biome { Temperate = 0, Meadow = 1, DeepForest = 2, Taiga = 3, Steppe = 4, Desert = 5, Highlands = 6, Mountains = 7, Sea = 8, Ocean = 9 }
pub const COUNT: usize = 10;
pub const NAMES: [&str; COUNT] = ["Forêt tempérée", "Prairie", "Forêt profonde", "Taïga", "Steppe sèche", "Désert", "Hautes terres", "Hautes montagnes", "Mer", "Océan"];
/// Place de chaque biome sur la carte température, humidité.
const ANCHORS: [(f64, f64); COUNT] = [(0.05, 0.15), (0.32, -0.32), (0.28, 0.72), (-0.72, 0.05), (0.5, -0.56), (0.82, -0.86), (-0.75, -0.6), (-0.86, 0.72), (9.0, 9.0), (9.0, 9.0)]; // la mer et l'océan ne viennent pas du climat mais du relief des continents
/// Autour du départ, le monde reste la forêt tempérée validée.
const HOME: f64 = 420.0; const HOME_FADE: f64 = 900.0; const SOFT: f64 = 0.2;
#[derive(Clone, Copy, Debug, Default)] pub struct Mix { pub w: [f64; COUNT], pub temp: f64, pub wet: f64, /** part de mer (0 terre, 1 large) */ pub sea: f64, /** profondeur relative du large (0 mer côtière, 1 océan) */ pub deep: f64, /** approche de la côte : commence avant la mer, la terre s'y relève au-dessus du niveau de l'eau */ pub shore: f64 }
impl Mix {
    pub fn of(&self, b: Biome) -> f64 { self.w[b as usize] }
    /// Force du biome : 0 hors de sa zone, 1 dans son cœur. Le mélange reste doux, sans diluer le centre du biome.
    pub fn strong(&self, b: Biome) -> f64 { crate::noise::smooth(0.3, 0.55, self.of(b)) }
    pub fn main(&self) -> usize { let mut m = 0; for i in 1..COUNT { if self.w[i] > self.w[m] { m = i; } } m }
}
pub struct Climate { a: Noise, b: Noise, c: Noise, flat: bool }
impl Climate {
    pub fn new(seed: i32) -> Self { Climate { a: Noise::new(seed.wrapping_add(21)), b: Noise::new(seed.wrapping_add(22)), c: Noise::new(seed.wrapping_add(23)), flat: crate::gen::is_flat(seed) } }
    pub fn at(&self, x: f64, z: f64) -> Mix {
        let (wx, wz) = (x + 110.0 * self.b.n2(x / 1100.0, z / 1100.0), z + 110.0 * self.a.n2(x / 1100.0 + 30.0, z / 1100.0 - 70.0));
        let home = if self.flat { 0.0 } else { crate::noise::smooth(HOME, HOME_FADE, hypot2(x, z)) }; // monde plat : le climat doux du départ, partout
        let temp = ANCHORS[0].0 + ((self.a.fbm(wx / 3000.0, wz / 3000.0, 2) * 2.6).clamp(-1.0, 1.0) - ANCHORS[0].0) * home;
        let wet = ANCHORS[0].1 + ((self.b.fbm(wx / 3000.0 + 91.0, wz / 3000.0 - 37.0, 2) * 2.6).clamp(-1.05, 1.0) - ANCHORS[0].1) * home;
        let mut m = Mix { w: [0.0; COUNT], temp, wet, sea: 0.0, deep: 0.0, shore: 0.0 }; let mut sum = 0.0;
        // Poids mesurés depuis le biome le plus proche : aucun point du monde ne reste sans biome.
        let d: Vec<f64> = ANCHORS.iter().map(|a| (temp - a.0) * (temp - a.0) + (wet - a.1) * (wet - a.1)).collect(); let near = d.iter().cloned().fold(f64::MAX, f64::min);
        for i in 0..COUNT { m.w[i] = (-(d[i] - near) / SOFT).exp(); sum += m.w[i]; }
        for v in &mut m.w { *v /= sum.max(1e-12); }
        // Continents et mers : un champ encore plus lent que le climat. Là où il plonge, la terre cède à la mer côtière, puis au grand large.
        // Les bassins font plusieurs milliers de blocs ; le départ reste sur la terre ferme.
        let cont = self.c.fbm(wx / 7500.0 + 13.0, wz / 7500.0 - 41.0, 3) + 0.12 * self.c.n2(wx / 1600.0 - 7.0, wz / 1600.0 + 19.0);
        let sea = crate::noise::smooth(0.12, 0.26, cont) * home; let deep = crate::noise::smooth(0.3, 0.46, cont);
        if sea > 0.0 { for v in &mut m.w { *v *= 1.0 - sea; } m.w[Biome::Sea as usize] = sea * (1.0 - deep); m.w[Biome::Ocean as usize] = sea * deep; }
        m.sea = sea; m.deep = deep; m.shore = crate::noise::smooth(0.03, 0.12, cont) * home; m
    }
}
