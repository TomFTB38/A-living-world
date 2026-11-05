//! Scène réelle pour le banc WebGPU : textures et maillages du moteur, écrits dans un fichier.
use alw_core::{stream::*, textures::*};
use std::io::Write;
fn main() {
    let a: Vec<String> = std::env::args().collect(); let out = &a[1]; let (x, z): (f64, f64) = (a[2].parse().unwrap(), a[3].parse().unwrap());
    let t = build_textures(128); let mut s = Streamer::new(20261104, &Preset { radius: 3, lod1: 1.5, lod2: 2.5 }, 8); let mut all = Vec::new();
    loop { let (b, _) = s.update(x, z); all.extend(b); if s.waiting() == 0 { break; } std::thread::sleep(std::time::Duration::from_millis(5)); }
    let mut f = std::io::BufWriter::new(std::fs::File::create(out).unwrap());
    f.write_all(&(t.layers).to_le_bytes()).unwrap(); f.write_all(&(all.len() as u32).to_le_bytes()).unwrap();
    f.write_all(&t.albedo).unwrap(); f.write_all(&t.nrm).unwrap();
    for b in &all { for v in [b.key.0, b.key.1, b.mesh.n_o as i32, b.mesh.n_d as i32, b.mesh.n_w as i32] { f.write_all(&v.to_le_bytes()).unwrap(); } f.write_all(&b.mesh.opaque).unwrap(); f.write_all(&b.mesh.deco).unwrap(); f.write_all(&b.mesh.water).unwrap(); }
    println!("{} chunks", all.len());
}
