use alw_core::gen::WorldGen; use std::time::Instant;
fn main(){ let g=WorldGen::new(20261104);
 for near in [480, 672] { let t=Instant::now(); let (d,n)=alw_core::mesher::far_mesh(&g,0,0,near); println!("far_mesh near {near}: {n} verts {} MB {:.0} ms", d.len()/1000000, t.elapsed().as_secs_f64()*1e3); }
 let t=Instant::now(); let mut tot=0; for k in 0..8 { let (_,n)=alw_core::mesher::far_trees_mesh(&g,k-4,2); tot+=n; } println!("8 tree tiles {tot} verts {:.0} ms", t.elapsed().as_secs_f64()*1e3);
}
