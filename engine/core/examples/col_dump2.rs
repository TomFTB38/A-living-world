use alw_core::blocks::*;
fn main() { let a: Vec<i32> = std::env::args().skip(1).map(|s| s.parse().unwrap()).collect(); let g = alw_core::gen::WorldGen::new(20261104); let c = CX as i32; let (x0, z0) = (a[0], a[1]);
  let mut n = std::collections::HashMap::new();
  for dz in -2..=2 { for dx in -2..=2 { for b in g.gen_chunk(x0.div_euclid(c) + dx, z0.div_euclid(c) + dz) { if b >= B::CORAL_BLOCK_RED { *n.entry(BLOCKS[b as usize].name).or_insert(0) += 1; } } } }
  println!("{n:?}");
  // vue de dessus : bloc le plus haut non-eau, non-déco
  for z in (z0 - 14)..(z0 + 14) { let mut l = String::new(); for x in (x0 - 24)..(x0 + 24) { let v = g.gen_chunk(x.div_euclid(c), z.div_euclid(c)); let mut ch = '.'; for y in (1..60).rev() { let b = v[idx(x.rem_euclid(c) as usize, y, z.rem_euclid(c) as usize)]; if b == 0 || is_wet(b) { continue; } ch = match BLOCKS[b as usize].name { "wreck_planks" => '#', "spruce_log" => '|', "chest" => 'C', "barrel" => 'o', "wool_white" => 'w', n if n.starts_with("coral_block") => '*', "sand" => '.', "gravel" => ':', "clay" => ',', _ => '?' }; break; } l.push(ch); } println!("{l}"); } }
