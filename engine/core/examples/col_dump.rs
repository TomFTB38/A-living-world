use alw_core::blocks::*;
fn main() { let a: Vec<i32> = std::env::args().skip(1).map(|s| s.parse().unwrap()).collect(); let g = alw_core::gen::WorldGen::new(20261104); let c = CX as i32;
  for (x, z) in [(a[0], a[1]), (a[0] + 3, a[1] + 5), (a[0] - 7, a[1] + 2)] { let v = g.gen_chunk(x.div_euclid(c), z.div_euclid(c)); let col = g.column(x as f64, z as f64); print!("({x},{z}) h={:.1} water={} sea={:.2} temp={:.2}: ", col.h, col.water, col.mix.sea, col.mix.temp);
  for y in 30..56 { let b = v[idx(x.rem_euclid(c) as usize, y, z.rem_euclid(c) as usize)]; if b != 0 { print!("{y}:{} ", BLOCKS[b as usize].name); } } println!(); } }
