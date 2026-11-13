//! Le chat des commandes et les œufs d'apparition.
use alw_core::{commande::EGGS, session::*, stream::Preset};
fn session() -> Session { Session::new(20261104, &Preset { radius: 1, lod1: 1.5, lod2: 2.5 }, 1, [0.5, 120.0, 0.5]) }

#[test]
fn les_commandes_du_chat() {
    let mut s = session(); s.hour = 10.0;
    s.command("/nuit"); assert!((s.hour - 22.0).abs() < 1e-9); s.command("jour"); assert!((s.hour - 7.0).abs() < 1e-9);
    s.command("/heure 18"); assert!((s.hour - 18.0).abs() < 1e-9); let h0 = s.hours; s.command("/avance 3"); assert!((s.hour - 21.0).abs() < 1e-9 && s.hours - h0 > 2.9);
    s.command("/vitesse 20"); assert_eq!(s.time_speed, 20.0); s.command("/figer"); assert!(s.time_frozen); s.command("/figer"); assert!(!s.time_frozen);
    assert!(s.command("/vol").contains("créatif")); assert!(!s.player.fly, "pas de vol en survie");
    s.command("/créatif"); assert!(s.creative); s.command("/vol"); assert!(s.player.fly); s.command("/vol"); assert!(!s.player.fly);
    s.command("/vol"); s.command("/SURVIE"); assert!(!s.creative && !s.player.fly, "la survie retire le vol");
    assert!(s.command("/n'importe quoi").contains("inconnue")); assert!(s.command("/aide").contains("/vitesse"));
}

#[test]
fn un_oeuf_pour_chaque_creature() {
    let mut s = session(); s.wild.all.clear(); s.horde.foes.clear(); s.fauna.deer.clear();
    for (egg, _) in EGGS { assert!(item(egg).is_some(), "objet {egg}"); assert!(s.hatch(egg, [3.5, 60.0, 3.5]).is_some(), "{egg}"); }
    assert_eq!(s.horde.foes.len(), 4, "les quatre hostiles"); assert_eq!(s.fauna.deer.len(), 1, "le cerf"); assert_eq!(s.wild.all.len(), EGGS.len() - 5, "les autres bêtes");
    assert!(s.hatch("bread", [0.0; 3]).is_none());
}
