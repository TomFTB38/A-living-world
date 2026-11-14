//! Coupes verticales et plans de la salle la plus proche de chaque biome souterrain : pour juger formes et matières.
use alw_core::{blocks::*, gen::WorldGen, karst::*};
fn col(b: Id) -> [u8; 3] {
    let n = BLOCKS[b as usize].name;
    if b == 0 { return [16, 16, 22]; } if is_water(b) { return [40, 120, 190]; } if b == B::LAVA { return [255, 110, 20]; } if b == B::MAGMA { return [170, 50, 20]; }
    for (k, c) in [("crystal_violet", [170, 90, 240]), ("crystal_blue", [90, 170, 250]), ("crystal_pink", [240, 120, 190]), ("crystal_white", [230, 240, 255]), ("glow", [60, 240, 210]), ("mushroom_cap_red", [210, 40, 30]), ("mushroom_cap_violet", [120, 50, 170]), ("mushroom_cap_blue", [40, 90, 200]), ("mushroom_gills", [255, 190, 80]), ("mushroom_stem", [230, 220, 200]), ("mycelium", [120, 100, 125]),
        ("abyss", [40, 46, 64]), ("basalt", [60, 62, 70]), ("obsidian", [40, 20, 60]), ("ice", [150, 200, 240]), ("snow", [245, 248, 255]), ("sandstone", [215, 185, 120]), ("red_sandstone", [190, 100, 60]), ("sand", [235, 215, 160]), ("coral", [240, 110, 120]), ("kelp", [40, 110, 50]), ("seagrass", [60, 140, 60]),
        ("flowstone", [235, 210, 170]), ("dripstone", [255, 235, 140]), ("limestone", [175, 165, 140]), ("mossy", [80, 130, 60]), ("grass", [90, 170, 60]), ("fern", [60, 150, 60]), ("vine", [50, 120, 40]), ("roots", [110, 75, 40]), ("ore", [200, 120, 60]),
        ("lantern", [255, 230, 120]), ("torch", [255, 200, 90]), ("rail", [150, 150, 150]), ("oak_beam", [130, 90, 50]), ("planks", [160, 120, 70]), ("glowcap", [200, 90, 255]), ("gravel", [130, 125, 120]), ("dirt", [110, 80, 55])] { if n.contains(k) { return c; } }
    [100, 98, 95]
}
fn main() {
    let a: Vec<String> = std::env::args().collect(); let seed: i32 = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(20261104);
    let w = WorldGen::new(seed); let (cw, ch) = (128usize, 100usize + 128); let mut img = vec![[0u8; 3]; cw * 6 * ch * 2];
    let mut chunks = std::collections::HashMap::new();
    for (k, kind) in UNDERS.iter().enumerate() {
        let Some(h) = w.nearest_hall_of(0.0, 0.0, 14, Some(*kind)) else { continue };
        let mut get = |x: i32, y: i32, z: i32| -> Id { let kk = (x.div_euclid(32), z.div_euclid(32)); let v = chunks.entry(kk).or_insert_with(|| w.gen_chunk(kk.0, kk.1)); v[idx(x.rem_euclid(32) as usize, y as usize, z.rem_euclid(32) as usize)] };
        let (ox, oy) = ((k % 6) * cw, (k / 6) * ch); let (x0, z0) = (h.x as i32 - 64, h.z as i32 - 64);
        for i in 0..cw { for y in 0..100 { img[(oy + 99 - y) * cw * 6 + ox + i] = col(get(x0 + i as i32, y as i32, h.z as i32)); } }
        for j in 0..128 { for i in 0..cw { img[(oy + 100 + j) * cw * 6 + ox + i] = col(get(x0 + i as i32, h.floor + 3, z0 + j as i32)); } }
        println!("{} : {:.0} {:.0}, accès {:?}", kind.key(), h.x, h.z, h.way.map(|Way::Path { mine, .. }| if mine { "mine" } else { "bouche" }));
    }
    let (wi, hi) = (cw * 6, ch * 2); let mut out = format!("P6 {wi} {hi} 255\n").into_bytes(); for p in &img { out.extend_from_slice(p); } std::fs::write(&a[1], out).unwrap();
}
