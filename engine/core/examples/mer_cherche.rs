fn main() { let g = alw_core::gen::WorldGen::new(20261104); let mut out = vec![]; 
  for r in 20..800 { for k in 0..64 { let a = k as f64 / 64.0 * std::f64::consts::TAU; let (x, z) = (a.cos() * r as f64 * 24.0, a.sin() * r as f64 * 24.0); let c = g.column(x, z);
    let tag = if c.mix.sea > 0.6 && c.mix.temp > 0.45 && c.water > 0.0 && c.water - c.h < 8.0 && c.water - c.h > 3.0 { "recif" } else if c.mix.sea > 0.6 && c.water == 0.0 && c.h > 52.0 { "ile" } else if c.mix.sea > 0.02 && c.mix.sea < 0.2 && c.water == 0.0 && c.h < 50.0 && c.mix.temp > 0.3 { "plage_chaude" } else { continue };
    if out.iter().filter(|(t, _, _)| *t == tag).count() < 2 { println!("{tag} {x:.0}:{z:.0} h {:.0}", c.h); out.push((tag, x, z)); } if out.len() >= 6 { return; } } } }
