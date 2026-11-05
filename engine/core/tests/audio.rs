use alw_core::audio::*;
fn bank() -> Bank { Bank::parse(include_bytes!("../../game/assets/bank.bin")).expect("banque") }
fn stat(b: &[f32]) -> (f32, f32, f32) { let (mut s, mut p, mut j) = (0f32, 0f32, 0f32); for k in 0..b.len() { s += b[k] * b[k]; p = p.max(b[k].abs()); if k >= 2 { j = j.max((b[k] - b[k - 2]).abs()); } } ((s / b.len() as f32).sqrt(), p, j) }
#[test]
fn banque_complete() {
    let b = bank(); println!("{} familles de sons", b.set_count());
    for (n, m) in SOUNDS.iter() { for l in m.step.iter().chain(m.hit).chain(m.brk).chain(m.put) { assert!(b.has(l.set), "{n} : {}", l.set); } }
    for t in TOOLS.iter() { if let Some(l) = t.2 { assert!(b.has(l.set), "{}", l.set); } }
    for s in ["blade", "whoosh", "puff", "snort", "hurt", "splash", "soft"] { assert!(b.has(s), "{s}"); }
    let _ = sound_of("oak_log_x"); assert_eq!(sound_of("oak_log_x").m, "wood");
}
#[test]
fn humeurs() {
    let base = Situation { food: 100.0, water: 100.0, ..Default::default() };
    assert_eq!(pick_mood(base), Mood::Calme); assert_eq!(pick_mood(Situation { night: true, ..base }), Mood::Nuit); assert_eq!(pick_mood(Situation { building: true, night: true, ..base }), Mood::Construction);
    assert_eq!(pick_mood(Situation { underground: true, building: true, ..base }), Mood::Suspense); assert_eq!(pick_mood(Situation { hurt: true, underground: true, ..base }), Mood::Danger); assert_eq!(pick_mood(Situation { food: 10.0, ..base }), Mood::Danger);
}
#[test]
fn signal_de_chaque_famille() {
    let sr = 48000.0; let render = |f: &dyn Fn(&SoundScape), secs: f32, mood: Option<Mood>| { let (mut m, tx) = Mixer::new(sr, bank()); let mut s = SoundScape::new(Some(tx)); s.ear = [0.0; 3]; s.volumes(0.7, 0.7, if mood.is_some() { 0.5 } else { 0.0 }); s.force(mood); s.mood(Mood::Calme); s.yaw = 0.0; f(&s); let mut o = vec![0f32; (secs * sr) as usize * 2]; m.render(&mut o); (o, m.voices()) };
    let (o, _) = render(&|s| s.break_block("stone", Some("pickaxe")), 0.6, None); let a = stat(&o); println!("pierre {a:?}"); assert!(a.1 > 0.02 && a.1 < 0.9);
    let (o, _) = render(&|s| s.env(0.42, 0.0, false, false, None, None), 1.0, None); assert!(stat(&o).1 < 1e-4, "ciel clair : aucun vent");
    let (o, _) = render(&|s| s.env(1.0, 1.0, false, false, None, None), 4.0, None); let r = stat(&o[96000..]); println!("pluie {r:?}"); assert!(r.0 > 0.01 && r.1 < 0.8);
    let (o, _) = render(&|s| s.env(0.42, 0.0, false, false, None, Some([6.0, 0.0, 0.0])), 4.0, None); let c = stat(&o[96000..]); let (o2, _) = render(&|s| s.env(0.42, 0.0, false, false, Some([6.0, 0.0, 0.0]), None), 4.0, None); let f = stat(&o2[96000..]); println!("chute {c:?}, ruisseau {f:?}"); assert!(c.0 > f.0 && f.0 > 0.0005);
    for m in [Mood::Calme, Mood::Nuit, Mood::Construction, Mood::Suspense, Mood::Danger] { let (o, v) = render(&|_| {}, 14.0, Some(m)); let a = stat(&o); println!("{m:?} {a:?}, {v} voix"); assert!(a.0 > 0.001, "{m:?} sonne"); assert!(a.1 < 0.95, "{m:?} sans saturer"); }
}
