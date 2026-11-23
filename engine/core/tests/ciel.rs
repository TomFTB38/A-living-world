//! Le ciel : la lune change de phase au fil des jours (nouvelle, croissante, pleine, décroissante, en huit jours), la nuit
//! est plus claire sous la pleine lune ; l'arc-en-ciel ne paraît qu'avec une averse, un soleil bas, et une trouée dans les nuages.
use alw_core::{view, weather::{self, State}};

#[test]
fn la_lune_et_ses_phases() {
    for h in [0.0, 6.0, 12.0, 18.0, 23.0] { let s = view::sky_at(h); println!("{h:>4} : dir {:?} col {:?} night {:.2}", s.dir, s.col, s.night); }
    let ph: Vec<f32> = (0..9).map(|d| view::moon_phase(d as f64 * 24.0)).collect(); println!("phases jour après jour : {ph:?}");
    assert!((view::moon_phase(0.0) - 0.5).abs() < 1e-6, "on commence à la pleine lune");
    assert!((view::moon_phase(4.0 * 24.0)).abs() < 1e-6 || (view::moon_phase(4.0 * 24.0) - 1.0).abs() < 1e-6, "nouvelle lune quatre jours plus tard");
    assert!((view::moon_phase(8.0 * 24.0) - 0.5).abs() < 1e-5, "un cycle en huit jours");
    for w in ph.windows(2) { let d = (w[1] - w[0]).rem_euclid(1.0); assert!((d - 0.125).abs() < 1e-5, "elle avance d'un huitième par jour : {w:?}"); }
    assert!((view::moon_lit(0.5) - 1.0).abs() < 1e-6 && view::moon_lit(0.0) < 1e-6 && (view::moon_lit(0.25) - 0.5).abs() < 1e-5, "pleine, nouvelle, quartier");
}

#[test]
fn l_arc_en_ciel() {
    let shower = State { cloud: 0.6, rain: 0.3, fog: 0.0 };
    assert!(weather::rainbow(shower, 0.3) > 0.9, "averse, soleil bas, éclaircie : {}", weather::rainbow(shower, 0.3));
    assert_eq!(weather::rainbow(State { cloud: 0.42, rain: 0.0, fog: 0.0 }, 0.3), 0.0, "pas de pluie, pas d'arc");
    assert_eq!(weather::rainbow(State { cloud: 1.0, rain: 1.0, fog: 0.35 }, 0.3), 0.0, "sous un ciel tout couvert et la grosse pluie, pas d'arc");
    assert_eq!(weather::rainbow(shower, 0.9), 0.0, "le soleil trop haut : l'arc serait sous l'horizon");
    assert_eq!(weather::rainbow(shower, -0.2), 0.0, "la nuit");
    let mut sky = view::sky_at(16.5); let e = weather::apply(&mut sky, shower, false); assert!(e.rainbow > 0.5, "en jeu, l'après-midi d'averse : {}", e.rainbow);
}
