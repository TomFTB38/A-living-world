//! Grands animaux marins : dauphin, requin, baleine. Des corps galbés, taillés en tranches le long de leur axe
//! (dos sombre, flancs, ventre clair : la contre-ombre des animaux du large), une peau en pixels qui donne du relief,
//! et une queue séparée qui bat : de haut en bas pour les cétacés, de gauche à droite pour le requin.
use crate::hostile::skin;
use crate::mesher::VBuf;

type C = [f64; 3];
/// Échelle d'affichage de la baleine (le modèle mesure environ 2,8 blocs).
pub const WHALE: f64 = 4.0;
/// Le pivot de la queue, sur l'axe du corps (en unités du modèle).
pub const PIVOT_DOLPHIN: f64 = -0.35; pub const PIVOT_SHARK: f64 = -0.4; pub const PIVOT_WHALE: f64 = -0.6;

fn mix(a: C, b: C, t: f64) -> C { [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t] }
/// Corps galbé : entre deux tranches (z, demi-largeur, dessus, dessous), quatre boîtes empilées — un dos étroit, deux flancs
/// pleins et un ventre étroit — qui arrondissent la section et dégradent la couleur du dos sombre au ventre clair.
fn loft(q: &mut VBuf, prof: &[(f64, f64, f64, f64)], back: C, side: C, belly: C, k: u8) {
    for w in prof.windows(2) {
        let (z0, z1) = (w[0].0.min(w[1].0), w[0].0.max(w[1].0)); let hw = (w[0].1 + w[1].1) * 0.5; let top = (w[0].2 + w[1].2) * 0.5; let bot = (w[0].3 + w[1].3) * 0.5;
        let hh = top - bot; let (y1, y2, y3) = (bot + hh * 0.22, bot + hh * 0.52, top - hh * 0.2);
        skin(q, [-hw * 0.62, y3, z0, hw * 0.62, top, z1], back, k);
        skin(q, [-hw, y2, z0, hw, y3, z1], mix(back, side, 0.45), k);
        skin(q, [-hw, y1, z0, hw, y2, z1], side, k);
        skin(q, [-hw * 0.66, bot, z0, hw * 0.66, y1, z1], belly, k);
    }
}
/// Une nageoire en marches : `n` lames qui glissent de `a` vers `b` en s'amincissant.
fn fin(q: &mut VBuf, a: [f64; 3], b: [f64; 3], w0: f64, w1: f64, thick: f64, vertical: bool, c: C, k: u8) {
    let n = 7;
    for i in 0..n { let t = i as f64 / (n - 1) as f64; let p = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]; let w = w0 + (w1 - w0) * t;
        let box_ = if vertical { [p[0] - thick, p[1] - 0.03, p[2] - w * 0.5, p[0] + thick, p[1] + 0.03, p[2] + w * 0.5] } else { [p[0] - 0.03, p[1] - thick, p[2] - w * 0.5, p[0] + 0.03, p[1] + thick, p[2] + w * 0.5] };
        skin(q, box_, c, k); }
}
/// Une nageoire plate et large (pectorale, caudale) : des plaques qui s'écartent de `a` vers `b`.
fn blade(q: &mut VBuf, a: [f64; 3], b: [f64; 3], w0: f64, w1: f64, thick: f64, c: C, k: u8) {
    let n = 7;
    for i in 0..n { let t = i as f64 / (n - 1) as f64; let p = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]; let w = (w0 + (w1 - w0) * t) * 0.5; let d = (b[0] - a[0]).abs() / n as f64 * 0.75 + 0.02;
        skin(q, [p[0] - d, p[1] - thick, p[2] - w, p[0] + d, p[1] + thick, p[2] + w], c, k); }
}
const EYE: C = [0.04, 0.04, 0.05]; const MOUTH: C = [0.14, 0.15, 0.17];

pub fn parts() -> Vec<(&'static str, VBuf)> {
    // Dauphin : dos gris ardoise, flancs plus clairs, ventre crème ; melon bombé, rostre court, aileron en faux, pectorales.
    let (back, side, belly) = ([0.3, 0.35, 0.43], [0.5, 0.56, 0.63], [0.88, 0.87, 0.86]);
    let mut db = VBuf::new(4000); { let q = &mut db; let p = PIVOT_DOLPHIN;
        loft(q, &[(p, 0.16, 0.16, -0.16), (-0.1, 0.21, 0.21, -0.2), (0.15, 0.23, 0.23, -0.22), (0.4, 0.22, 0.22, -0.2), (0.6, 0.18, 0.2, -0.16), (0.72, 0.15, 0.19, -0.12), (0.82, 0.1, 0.11, -0.08)], back, side, belly, 0);
        loft(q, &[(0.82, 0.07, 0.03, -0.08), (0.95, 0.055, 0.015, -0.07), (1.05, 0.04, 0.0, -0.055)], mix(back, side, 0.5), side, belly, 0); // rostre
        for sx in [-1.0f64, 1.0] { skin(q, [sx * 0.15 - 0.015, 0.03, 0.6, sx * 0.15 + 0.015, 0.07, 0.65], EYE, 3); skin(q, [sx * 0.065 - 0.008, -0.045, 0.82, sx * 0.065 + 0.008, -0.03, 1.02], MOUTH, 3); }
        skin(q, [-0.03, 0.2, 0.47, 0.03, 0.225, 0.52], EYE, 3); // évent
        fin(q, [0.0, 0.24, 0.05], [0.0, 0.52, -0.22], 0.32, 0.06, 0.03, true, back, 0); // aileron en faux, rejeté en arrière
        for sx in [-1.0f64, 1.0] { blade(q, [sx * 0.2, -0.1, 0.42], [sx * 0.44, -0.24, 0.24], 0.16, 0.06, 0.022, mix(back, side, 0.3), 0); } }
    let mut dt = VBuf::new(2000); { let q = &mut dt;
        loft(q, &[(0.0, 0.16, 0.16, -0.16), (-0.25, 0.11, 0.12, -0.11), (-0.5, 0.07, 0.08, -0.07), (-0.72, 0.045, 0.055, -0.045)], back, side, mix(side, belly, 0.5), 0);
        for sx in [-1.0f64, 1.0] { blade(q, [sx * 0.03, 0.0, -0.74], [sx * 0.42, 0.0, -0.92], 0.18, 0.07, 0.02, back, 0); } } // caudale horizontale
    // Requin : gris bleuté, ventre blanc tranché net, museau pointu, haut aileron, cinq fentes branchiales, queue en croissant asymétrique.
    let (back, side, belly) = ([0.33, 0.39, 0.46], [0.48, 0.54, 0.6], [0.9, 0.9, 0.88]);
    let mut sb = VBuf::new(4000); { let q = &mut sb; let p = PIVOT_SHARK;
        loft(q, &[(p, 0.15, 0.16, -0.15), (-0.1, 0.2, 0.22, -0.2), (0.2, 0.22, 0.25, -0.22), (0.5, 0.2, 0.23, -0.2), (0.75, 0.16, 0.18, -0.15), (0.95, 0.1, 0.11, -0.09), (1.1, 0.05, 0.05, -0.04), (1.18, 0.02, 0.02, -0.02)], back, side, belly, 1);
        for sx in [-1.0f64, 1.0] { skin(q, [sx * 0.135 - 0.012, 0.05, 0.84, sx * 0.135 + 0.012, 0.085, 0.88], EYE, 3);
            for g in 0..5 { let z = 0.55 + g as f64 * 0.04; skin(q, [sx * (0.19 - g as f64 * 0.006) - 0.008, -0.08, z, sx * (0.19 - g as f64 * 0.006) + 0.008, 0.08, z + 0.012], MOUTH, 3); } } // fentes branchiales
        skin(q, [-0.08, -0.12, 0.88, 0.08, -0.09, 1.02], MOUTH, 3); for t in 0..5 { let x = -0.06 + t as f64 * 0.03; skin(q, [x - 0.007, -0.105, 0.97, x + 0.007, -0.08, 0.985], [0.95, 0.94, 0.9], 3); } // gueule et dents
        fin(q, [0.0, 0.25, 0.32], [0.0, 0.64, 0.08], 0.42, 0.05, 0.03, true, back, 1); // grand aileron dorsal
        for sx in [-1.0f64, 1.0] { blade(q, [sx * 0.2, -0.14, 0.5], [sx * 0.62, -0.32, 0.22], 0.26, 0.07, 0.025, mix(back, side, 0.3), 1); } // pectorales en ailes
        fin(q, [0.0, -0.22, 0.05], [0.0, -0.34, -0.05], 0.18, 0.05, 0.02, true, side, 1); }
    let mut st = VBuf::new(2000); { let q = &mut st;
        loft(q, &[(0.0, 0.15, 0.16, -0.15), (-0.3, 0.09, 0.11, -0.08), (-0.6, 0.05, 0.07, -0.045)], back, side, mix(side, belly, 0.4), 1);
        fin(q, [0.0, 0.04, -0.58], [0.0, 0.62, -0.98], 0.2, 0.06, 0.025, true, back, 1); fin(q, [0.0, -0.02, -0.6], [0.0, -0.34, -0.82], 0.16, 0.05, 0.025, true, back, 1); // lobes inégaux
        fin(q, [0.0, 0.14, -0.28], [0.0, 0.22, -0.36], 0.08, 0.03, 0.015, true, back, 1); }
    // Baleine à bosse : bleu-noir, gorge plissée claire, très longues pectorales blanches, petite bosse dorsale, bernacles sur la tête.
    let (back, side, belly) = ([0.17, 0.2, 0.25], [0.25, 0.28, 0.34], [0.78, 0.78, 0.8]);
    let mut wb = VBuf::new(6000); { let q = &mut wb; let p = PIVOT_WHALE;
        loft(q, &[(p, 0.3, 0.3, -0.33), (-0.2, 0.42, 0.38, -0.45), (0.2, 0.45, 0.4, -0.5), (0.6, 0.42, 0.36, -0.47)], back, side, belly, 0);
        loft(q, &[(0.6, 0.42, 0.36, -0.47), (0.95, 0.36, 0.28, -0.42), (1.2, 0.28, 0.18, -0.34), (1.35, 0.18, 0.08, -0.22)], back, side, belly, 2);
        for i in 0..9 { let z = 0.25 + i as f64 * 0.12; let w = 0.3 - i as f64 * 0.012; skin(q, [-w, -0.52 + i as f64 * 0.012, z, w, -0.49 + i as f64 * 0.012, z + 0.05], [0.52, 0.52, 0.56], 3); } // plis de la gorge
        for sx in [-1.0f64, 1.0] { skin(q, [sx * 0.37 - 0.02, -0.06, 0.98, sx * 0.37 + 0.02, -0.01, 1.04], EYE, 3); skin(q, [sx * 0.3 - 0.012, -0.24, 0.7, sx * 0.3 + 0.012, -0.2, 1.3], MOUTH, 3); }
        for k in 0..7 { let z = 0.95 + k as f64 * 0.06; for sx in [-1.0f64, 1.0] { skin(q, [sx * 0.09 - 0.03, 0.24 - k as f64 * 0.018, z, sx * 0.09 + 0.03, 0.29 - k as f64 * 0.018, z + 0.04], [0.36, 0.38, 0.42], 2); } } // tubercules
        fin(q, [0.0, 0.33, -0.3], [0.0, 0.44, -0.45], 0.2, 0.06, 0.03, true, back, 0); // petite bosse dorsale
        for sx in [-1.0f64, 1.0] { blade(q, [sx * 0.42, -0.25, 0.7], [sx * 1.25, -0.55, 0.15], 0.3, 0.12, 0.03, [0.74, 0.74, 0.76], 2); } } // pectorales géantes
    let mut wt = VBuf::new(3000); { let q = &mut wt;
        loft(q, &[(0.0, 0.3, 0.3, -0.33), (-0.4, 0.2, 0.22, -0.2), (-0.8, 0.1, 0.12, -0.1), (-1.0, 0.07, 0.08, -0.07)], back, side, mix(side, belly, 0.4), 0);
        for sx in [-1.0f64, 1.0] { blade(q, [sx * 0.05, 0.0, -1.0], [sx * 0.75, 0.02, -1.25], 0.34, 0.14, 0.03, back, 0); skin(q, [sx * 0.75 - 0.05, -0.04, -1.32, sx * 0.75 + 0.05, -0.01, -1.18], belly, 0); } } // caudale, dessous blanc
    vec![("dolphinBody", db), ("dolphinTail", dt), ("sharkBody", sb), ("sharkTail", st), ("whaleBody", wb), ("whaleTail", wt)]
}
