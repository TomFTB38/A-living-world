//! Les bêtes, sculptées : cerf, oiseaux, renard, lièvres, corbeau, loup, sanglier, ours, bouquetin, lézard, poissons,
//! crabe, goéland. Des corps galbés (dos sombre, flancs, ventre clair), des muscles aux épaules et aux cuisses, des
//! membres effilés, des têtes modelées (crâne, joues, museau, truffe, oreilles), et une peau en pixels : poil, plume, écaille.
//! Chaque modèle garde les pivots et l'encombrement de l'ancien : le corps à hauteur de hanche, les pattes qui pendent
//! depuis la hanche, la tête depuis la base du cou, tout tourné vers +z.
use crate::hostile::Mat;
use crate::mesher::VBuf;
use crate::sculpt::{dk, mix, Pen, C};

const BLACK: C = [0.08, 0.07, 0.07]; const WHITE: C = [0.94, 0.93, 0.9]; const NOSE: C = [0.1, 0.08, 0.08];
/// Un œil : un globe sombre et un reflet.
fn eye(p: &mut Pen, x: f64, y: f64, z: f64, r: f64, iris: C) {
    let s = x.signum(); p.bm([x - r, y - r, z - r, x + r, y + r, z + r], iris, Mat::Plain);
    let g = r * 0.45; p.bm([x + s * r * 0.2 - g * 0.5, y + r * 0.2, z + r * 0.35, x + s * r * 0.2 + g * 0.5, y + r * 0.2 + g, z + r * 1.02], [0.9, 0.9, 0.86], Mat::Plain);
}
/// Une oreille dressée en pointe : des plaques qui rétrécissent en montant, penchées vers l'extérieur.
#[allow(clippy::too_many_arguments)]
fn ear(p: &mut Pen, x: f64, y: f64, z: f64, w: f64, h: f64, out: f64, c: C, tip: C, inner: C) {
    let n = 5; let t = w * 0.35;
    for k in 0..n { let f = k as f64 / n as f64; let ww = w * (1.0 - f * 0.8); let cx = x + x.signum() * out * f; let y0 = y + h * f; let y1 = y + h * (f + 1.0 / n as f64);
        p.bx([cx - ww * 0.5, y0, z - t, cx + ww * 0.5, y1, z + t], if k == n - 1 { tip } else { c });
        if k < n - 2 { p.bm([cx - ww * 0.3, y0, z + t, cx + ww * 0.3, y1, z + t * 1.25], inner, Mat::Plain); } }
}

/// Le cerf, la biche et l'oiseau des bois (avec son aile).
pub fn deer() -> Vec<(&'static str, VBuf)> {
    const FUR: C = [0.68, 0.45, 0.26]; const FUR2: C = [0.5, 0.32, 0.18]; const CREAM: C = [0.92, 0.84, 0.7]; const ANT: C = [0.74, 0.62, 0.45];
    let mut body = VBuf::new(8000); { let mut p = Pen::new(&mut body, Mat::Fur, 0.05);
        p.body(&[(-0.6, 0.1, 0.38, 0.16), (-0.52, 0.17, 0.42, 0.05), (-0.3, 0.2, 0.43, -0.02), (0.0, 0.18, 0.4, -0.05), (0.25, 0.19, 0.42, -0.02), (0.45, 0.17, 0.46, 0.04), (0.58, 0.12, 0.43, 0.14)], FUR2, FUR, CREAM);
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.12, 0.2, -0.36], [0.09, 0.18, 0.17], FUR, dk(FUR, 0.92)); p.blob([sx * 0.12, 0.2, 0.36], [0.08, 0.16, 0.12], FUR, dk(FUR, 0.92)); } // cuisses et épaules
        p.blob([0.0, 0.3, -0.58], [0.11, 0.1, 0.05], CREAM, CREAM); // miroir blanc de la croupe
        p.tube([0.0, 0.4, -0.6], [0.0, 0.3, -0.7], [0.045, 0.03], [0.035, 0.025], FUR2, CREAM); } // queue
    let mut head = VBuf::new(8000); { let mut p = Pen::new(&mut head, Mat::Fur, 0.04);
        p.tube([0.0, -0.02, -0.02], [0.0, 0.42, 0.03], [0.095, 0.12], [0.065, 0.08], FUR, FUR); // cou
        p.bx([-0.05, 0.02, 0.06, 0.05, 0.36, 0.1], CREAM); // gorge claire
        p.blob([0.0, 0.48, 0.04], [0.1, 0.09, 0.12], FUR, FUR2);
        p.tube([0.0, 0.47, 0.1], [0.0, 0.44, 0.3], [0.068, 0.065], [0.045, 0.042], FUR2, dk(FUR2, 0.9)); // chanfrein
        p.bx([-0.04, 0.39, 0.16, 0.04, 0.43, 0.29], CREAM); // dessous du museau
        p.bm([-0.045, 0.43, 0.29, 0.045, 0.49, 0.325], NOSE, Mat::Plain);
        for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.095, 0.5, 0.1, 0.018, [0.12, 0.08, 0.05]); p.bx([sx * 0.085 - 0.02, 0.47, 0.13, sx * 0.085 + 0.02, 0.485, 0.2], dk(FUR2, 0.8)); // larmier
            p.tube([sx * 0.08, 0.55, 0.0], [sx * 0.24, 0.6, -0.01], [0.045, 0.016], [0.03, 0.012], FUR2, FUR); p.bm([sx * 0.13 - 0.04, 0.565, 0.006, sx * 0.13 + 0.04, 0.59, 0.016], CREAM, Mat::Plain); // oreilles
            // bois : merrain qui monte et s'ouvre, trois andouillers
            let a = |x: f64, y: f64, z: f64| [sx * x, y, z];
            p.tube_m(a(0.06, 0.56, -0.0), a(0.1, 0.72, -0.03), [0.024, 0.024], [0.02, 0.02], ANT, ANT, Mat::Bone);
            p.tube_m(a(0.1, 0.72, -0.03), a(0.17, 0.88, -0.05), [0.02, 0.02], [0.017, 0.017], ANT, ANT, Mat::Bone);
            p.tube_m(a(0.17, 0.88, -0.05), a(0.22, 1.04, -0.02), [0.017, 0.017], [0.012, 0.012], ANT, mix(ANT, WHITE, 0.4), Mat::Bone);
            p.tube_m(a(0.08, 0.66, -0.01), a(0.1, 0.7, 0.14), [0.014, 0.014], [0.01, 0.01], ANT, mix(ANT, WHITE, 0.4), Mat::Bone);
            p.tube_m(a(0.15, 0.84, -0.04), a(0.15, 0.92, 0.09), [0.013, 0.013], [0.009, 0.009], ANT, mix(ANT, WHITE, 0.4), Mat::Bone);
            p.tube_m(a(0.19, 0.95, -0.04), a(0.31, 0.99, -0.06), [0.012, 0.012], [0.008, 0.008], ANT, mix(ANT, WHITE, 0.4), Mat::Bone); } }
    let mut leg = VBuf::new(3000); { let mut p = Pen::new(&mut leg, Mat::Fur, 0.035);
        p.tube([0.0, 0.04, 0.0], [0.0, -0.3, 0.005], [0.055, 0.075], [0.038, 0.045], FUR, FUR2);
        p.blob([0.0, -0.31, 0.0], [0.042, 0.04, 0.045], FUR2, FUR2); // genou
        p.tube([0.0, -0.32, 0.0], [0.0, -0.6, 0.012], [0.032, 0.035], [0.028, 0.032], mix(FUR2, CREAM, 0.3), mix(FUR2, CREAM, 0.6));
        p.bm([-0.042, -0.7, -0.04, 0.042, -0.6, 0.06], [0.14, 0.1, 0.08], Mat::Bone); p.bm([-0.004, -0.7, 0.0, 0.004, -0.64, 0.065], [0.05, 0.04, 0.03], Mat::Plain); } // sabot fendu
    let (bird, wing) = bird_parts(crate::fauna::BIRD, [0.22, 0.5, 0.86], [0.95, 0.9, 0.82], [0.14, 0.3, 0.6], [0.92, 0.65, 0.15], [0.15, 0.36, 0.72], None);
    vec![("deerBody", body), ("deerHead", head), ("deerLeg", leg), ("bird", bird), ("wing", wing)]
}

/// Un oiseau (corps posé sur ses pattes, origine au sol) et son aile (de l'épaule vers +x), à l'échelle `k`.
fn bird_parts(k: f64, back: C, belly: C, tail: C, beak: C, wingc: C, tip: Option<C>) -> (VBuf, VBuf) {
    let s = |v: [f64; 3]| v.map(|x| x * k); let s6 = |v: [f64; 6]| v.map(|x| x * k);
    let mut o = VBuf::new(6000); { let mut p = Pen::new(&mut o, Mat::Feather, 0.016 * k);
        p.blob(s([0.0, 0.055, -0.005]), s([0.055, 0.05, 0.1]), back, belly);
        p.blob(s([0.0, 0.12, 0.1]), s([0.046, 0.045, 0.048]), back, mix(back, belly, 0.5)); // tête
        p.tube_m(s([0.0, 0.12, 0.14]), s([0.0, 0.112, 0.2]), [0.016 * k, 0.012 * k], [0.004 * k, 0.003 * k], beak, dk(beak, 0.8), Mat::Bone);
        for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.04 * k, 0.13 * k, 0.12 * k, 0.008 * k, BLACK);
            p.tube_m(s([sx * 0.025, 0.02, 0.0]), s([sx * 0.025, -0.055, 0.008]), [0.006 * k, 0.006 * k], [0.005 * k, 0.005 * k], [0.3, 0.24, 0.2], [0.3, 0.24, 0.2], Mat::Plain);
            p.bm(s6([sx * 0.025 - 0.012, -0.06, -0.005, sx * 0.025 + 0.012, -0.053, 0.03]), [0.3, 0.24, 0.2], Mat::Plain); }
        for i in 0..4 { let z = -0.1 - i as f64 * 0.035; let w = 0.03 + i as f64 * 0.006; p.bx(s6([-w, 0.05 - i as f64 * 0.006, z - 0.03, w, 0.065 - i as f64 * 0.006, z + 0.005]), if i == 3 { dk(tail, 0.8) } else { tail }); } } // queue étagée
    let mut w = VBuf::new(2000); { let mut p = Pen::new(&mut w, Mat::Feather, 0.016 * k);
        // L'aile : un bras épais près de l'épaule, puis des rémiges qui s'allongent vers l'arrière au bout.
        for i in 0..6 { let x0 = i as f64 * 0.03; let f = i as f64 / 5.0; let z0 = -0.06 - f * 0.05; let z1 = 0.075 - f * 0.05; let th = 0.012 * (1.0 - f * 0.5);
            let c = match tip { Some(t) if i >= 4 => t, _ => mix(wingc, dk(wingc, 0.8), f) };
            p.bx(s6([x0, -th * 0.5, z0, x0 + 0.032, th * 0.5 + 0.004, z1]), c); }
        p.bx(s6([0.0, 0.004, 0.01, 0.07, 0.014, 0.08]), mix(wingc, WHITE, 0.15)); } // couvertures
    (o, w)
}

/// Renard, lièvres, corbeau, loup, sanglier, ours, bouquetin, lézard.
pub fn land(crow_k: f64) -> Vec<(&'static str, VBuf)> {
    let mut out: Vec<(&'static str, VBuf)> = Vec::new();
    // ---- Renard roux : élancé, queue en panache à bout blanc, bas des pattes noir, joues blanches, oreilles noires au bout.
    const ORANGE: C = [0.8, 0.38, 0.12]; const ORANGE2: C = [0.62, 0.27, 0.09]; const SOCK: C = [0.16, 0.1, 0.07];
    let mut fox = VBuf::new(6000); { let mut p = Pen::new(&mut fox, Mat::Fur, 0.03);
        p.body(&[(-0.32, 0.07, 0.19, 0.07), (-0.25, 0.1, 0.22, 0.01), (0.0, 0.095, 0.21, 0.0), (0.2, 0.1, 0.23, 0.0), (0.3, 0.08, 0.21, 0.05)], ORANGE2, ORANGE, WHITE);
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.06, 0.12, -0.21], [0.05, 0.1, 0.1], ORANGE, ORANGE); }
        p.bx([-0.05, 0.03, 0.18, 0.05, 0.2, 0.31], WHITE); // poitrail
        p.body(&[(-0.28, 0.05, 0.18, 0.09), (-0.4, 0.08, 0.19, 0.01), (-0.56, 0.085, 0.16, -0.02), (-0.66, 0.07, 0.13, 0.0)], ORANGE2, ORANGE, mix(ORANGE, WHITE, 0.3));
        p.blob([0.0, 0.065, -0.72], [0.06, 0.06, 0.065], WHITE, WHITE); }
    let mut fox_head = VBuf::new(5000); { let mut p = Pen::new(&mut fox_head, Mat::Fur, 0.025);
        p.blob([0.0, 0.07, 0.04], [0.1, 0.085, 0.09], ORANGE, ORANGE);
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.065, 0.025, 0.07], [0.05, 0.045, 0.06], WHITE, WHITE); } // joues
        p.tube([0.0, 0.05, 0.1], [0.0, 0.035, 0.28], [0.045, 0.035], [0.02, 0.018], ORANGE, ORANGE2);
        p.tube([0.0, 0.015, 0.1], [0.0, 0.012, 0.26], [0.04, 0.025], [0.016, 0.012], WHITE, WHITE);
        p.blob_m([0.0, 0.04, 0.29], [0.02, 0.018, 0.018], NOSE, NOSE, Mat::Plain);
        for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.055, 0.1, 0.11, 0.014, [0.7, 0.5, 0.1]); ear(&mut p, sx * 0.065, 0.13, 0.0, 0.07, 0.14, 0.02, ORANGE, BLACK, WHITE); } }
    let mut fox_leg = VBuf::new(1500); { let mut p = Pen::new(&mut fox_leg, Mat::Fur, 0.025);
        p.tube([0.0, 0.03, 0.0], [0.0, -0.16, 0.0], [0.04, 0.05], [0.027, 0.03], ORANGE, ORANGE2); p.tube([0.0, -0.16, 0.0], [0.0, -0.27, 0.01], [0.025, 0.028], [0.022, 0.025], SOCK, SOCK);
        p.bx([-0.028, -0.29, -0.025, 0.028, -0.26, 0.04], SOCK); }
    out.extend([("fox", fox), ("foxHead", fox_head), ("foxLeg", fox_leg)]);
    // ---- Lièvres et lapins : brun, gris, blanc d'hiver ou noir ; dos rond, cuisses puissantes, longues oreilles.
    let hare = |fur: C, belly: C, tip: C| { let mut o = VBuf::new(8000); { let mut p = Pen::new(&mut o, Mat::Fur, 0.028);
        p.blob([0.0, 0.12, -0.02], [0.095, 0.1, 0.15], fur, belly);
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.07, 0.085, -0.07], [0.045, 0.075, 0.09], fur, dk(fur, 0.92));
            p.bx([sx * 0.07 - 0.025, 0.0, -0.13, sx * 0.07 + 0.025, 0.025, 0.04], dk(fur, 0.85)); // grands pieds
            p.tube([sx * 0.04, 0.08, 0.1], [sx * 0.04, 0.0, 0.13], [0.018, 0.02], [0.015, 0.016], fur, belly); }
        p.blob([0.0, 0.2, 0.16], [0.065, 0.065, 0.075], fur, mix(fur, belly, 0.4));
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.03, 0.17, 0.22], [0.032, 0.03, 0.032], mix(fur, belly, 0.5), belly); } // babines
        p.bm([-0.012, 0.185, 0.245, 0.012, 0.2, 0.26], [0.75, 0.5, 0.5], Mat::Plain);
        for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.052, 0.215, 0.18, 0.013, [0.12, 0.06, 0.04]);
            p.tube([sx * 0.032, 0.25, 0.13], [sx * 0.05, 0.45, 0.09], [0.022, 0.012], [0.017, 0.009], fur, fur);
            p.bm([sx * 0.05 - 0.016, 0.42, 0.083, sx * 0.05 + 0.016, 0.465, 0.1], tip, Mat::Plain);
            p.bm([sx * 0.04 - 0.01, 0.28, 0.14, sx * 0.04 + 0.01, 0.41, 0.145], [0.8, 0.6, 0.58], Mat::Plain); }
        p.blob([0.0, 0.13, -0.18], [0.035, 0.035, 0.03], belly, belly); } o };
    out.push(("hare0", hare([0.52, 0.37, 0.24], [0.86, 0.8, 0.7], BLACK))); out.push(("hare1", hare([0.56, 0.55, 0.53], [0.82, 0.81, 0.79], [0.25, 0.24, 0.23])));
    out.push(("hare2", hare(WHITE, [0.9, 0.9, 0.9], BLACK))); out.push(("hare3", hare([0.13, 0.12, 0.12], [0.3, 0.28, 0.27], [0.08, 0.08, 0.08])));
    // ---- Corbeau : noir bleuté, gros bec.
    let (crow, crow_wing) = bird_parts(crow_k, [0.09, 0.09, 0.11], [0.12, 0.12, 0.14], [0.1, 0.1, 0.12], [0.16, 0.16, 0.18], [0.1, 0.1, 0.13], None);
    out.extend([("crow", crow), ("crowWing", crow_wing)]);
    // ---- Loup : gris, collerette épaisse, ventre pâle, masque clair, yeux d'ambre, queue droite au bout sombre.
    const GREY: C = [0.42, 0.41, 0.39]; const GREY2: C = [0.28, 0.27, 0.26]; const PALE: C = [0.72, 0.7, 0.66];
    let mut wolf = VBuf::new(8000); { let mut p = Pen::new(&mut wolf, Mat::Fur, 0.03);
        p.body(&[(-0.34, 0.08, 0.22, 0.07), (-0.26, 0.12, 0.25, 0.0), (0.0, 0.115, 0.24, 0.0), (0.16, 0.13, 0.28, -0.01), (0.3, 0.11, 0.27, 0.05)], GREY2, GREY, PALE);
        p.blob([0.0, 0.17, 0.22], [0.14, 0.13, 0.1], GREY, PALE); // collerette
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.075, 0.13, -0.22], [0.06, 0.11, 0.11], GREY, GREY); }
        p.body(&[(-0.3, 0.05, 0.21, 0.11), (-0.44, 0.065, 0.18, 0.04), (-0.6, 0.055, 0.11, -0.01), (-0.7, 0.035, 0.07, 0.0)], GREY2, GREY, PALE);
        p.blob([0.0, 0.035, -0.69], [0.035, 0.035, 0.035], BLACK, GREY2); }
    let mut wolf_head = VBuf::new(6000); { let mut p = Pen::new(&mut wolf_head, Mat::Fur, 0.025);
        p.blob([0.0, 0.06, 0.05], [0.105, 0.09, 0.1], GREY, GREY);
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.085, 0.01, 0.03], [0.05, 0.065, 0.07], PALE, PALE); } // joues
        p.tube([0.0, 0.05, 0.12], [0.0, 0.035, 0.3], [0.05, 0.04], [0.03, 0.026], GREY, GREY2); p.tube([0.0, 0.0, 0.12], [0.0, 0.002, 0.28], [0.045, 0.03], [0.026, 0.018], PALE, PALE);
        p.blob_m([0.0, 0.045, 0.305], [0.024, 0.02, 0.02], NOSE, NOSE, Mat::Plain);
        for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.058, 0.09, 0.13, 0.015, [0.85, 0.68, 0.18]); p.bx([sx * 0.06 - 0.03, 0.115, 0.115, sx * 0.06 + 0.03, 0.13, 0.15], PALE); ear(&mut p, sx * 0.06, 0.13, 0.0, 0.065, 0.11, 0.015, GREY, GREY2, PALE); } }
    let mut wolf_leg = VBuf::new(1500); { let mut p = Pen::new(&mut wolf_leg, Mat::Fur, 0.025);
        p.tube([0.0, 0.03, 0.0], [0.0, -0.2, 0.0], [0.045, 0.06], [0.03, 0.034], GREY, GREY); p.tube([0.0, -0.2, 0.0], [0.0, -0.3, 0.01], [0.028, 0.03], [0.026, 0.028], PALE, PALE);
        p.bx([-0.032, -0.33, -0.03, 0.032, -0.29, 0.05], mix(PALE, GREY2, 0.4)); }
    out.extend([("wolf", wolf), ("wolfHead", wolf_head), ("wolfLeg", wolf_leg)]);
    // ---- Sanglier : massif et bas, avant puissant, crête de soies, groin, défenses recourbées.
    const BR: C = [0.24, 0.17, 0.12]; const BR2: C = [0.14, 0.1, 0.08]; const BRB: C = [0.32, 0.24, 0.18]; const TUSK: C = [0.92, 0.88, 0.78];
    let mut boar = VBuf::new(8000); { let mut p = Pen::new(&mut boar, Mat::Fur, 0.035);
        p.body(&[(-0.38, 0.1, 0.27, 0.07), (-0.3, 0.155, 0.31, 0.0), (0.0, 0.17, 0.34, -0.02), (0.18, 0.18, 0.38, 0.0), (0.32, 0.15, 0.34, 0.05)], BR2, BR, BRB);
        for i in 0..10 { let z = -0.28 + i as f64 * 0.06; let h = 0.04 + 0.05 * (1.0 - ((i as f64 - 6.0) / 6.0).abs()); let top = 0.33 + (z + 0.3) * 0.08; p.bm([-0.025, top - 0.02, z - 0.03, 0.025, top + h, z + 0.03], [0.1, 0.08, 0.06], Mat::Hair); } // crête
        p.tube([0.0, 0.25, -0.38], [0.0, 0.12, -0.43], [0.015, 0.015], [0.012, 0.012], BR2, BR2); p.bm([-0.02, 0.06, -0.45, 0.02, 0.13, -0.41], [0.1, 0.08, 0.06], Mat::Hair); }
    let mut boar_head = VBuf::new(6000); { let mut p = Pen::new(&mut boar_head, Mat::Fur, 0.03);
        p.body(&[(-0.05, 0.13, 0.2, -0.09), (0.08, 0.125, 0.18, -0.08), (0.2, 0.09, 0.1, -0.065), (0.31, 0.065, 0.05, -0.05)], BR2, BR, BRB);
        p.bm([-0.06, -0.055, 0.31, 0.06, 0.055, 0.345], [0.5, 0.36, 0.32], Mat::Flesh); for sx in [-1.0f64, 1.0] { p.bm([sx * 0.025 - 0.012, -0.01, 0.345, sx * 0.025 + 0.012, 0.015, 0.35], NOSE, Mat::Plain); } // groin
        for sx in [-1.0f64, 1.0] { p.tube_m([sx * 0.07, -0.04, 0.22], [sx * 0.085, 0.03, 0.25], [0.014, 0.014], [0.009, 0.009], TUSK, TUSK, Mat::Bone); p.tube_m([sx * 0.085, 0.03, 0.25], [sx * 0.075, 0.06, 0.22], [0.009, 0.009], [0.006, 0.006], TUSK, WHITE, Mat::Bone);
            eye(&mut p, sx * 0.1, 0.09, 0.1, 0.012, [0.1, 0.06, 0.04]); ear(&mut p, sx * 0.08, 0.17, -0.02, 0.06, 0.1, 0.04, BR, BR2, [0.4, 0.28, 0.24]); } }
    let mut boar_leg = VBuf::new(1500); { let mut p = Pen::new(&mut boar_leg, Mat::Fur, 0.03);
        p.tube([0.0, 0.03, 0.0], [0.0, -0.15, 0.0], [0.055, 0.07], [0.035, 0.04], BR, BR2); p.bm([-0.036, -0.2, -0.035, 0.036, -0.15, 0.045], [0.12, 0.1, 0.08], Mat::Bone); }
    out.extend([("boar", boar), ("boarHead", boar_head), ("boarLeg", boar_leg)]);
    // ---- Ours brun : bosse au garrot, pattes larges aux griffes claires, museau clair, oreilles rondes.
    const BEAR: C = [0.32, 0.21, 0.13]; const BEAR2: C = [0.22, 0.14, 0.08]; const MUZ: C = [0.5, 0.38, 0.26];
    let mut bear = VBuf::new(9000); { let mut p = Pen::new(&mut bear, Mat::Fur, 0.038);
        p.body(&[(-0.34, 0.12, 0.27, 0.06), (-0.25, 0.17, 0.31, -0.01), (0.0, 0.18, 0.32, -0.02), (0.12, 0.18, 0.37, -0.01), (0.25, 0.16, 0.34, 0.02), (0.33, 0.12, 0.28, 0.06)], BEAR2, BEAR, BEAR2);
        p.blob([0.0, 0.31, 0.12], [0.13, 0.08, 0.13], mix(BEAR, MUZ, 0.25), BEAR); // bosse du garrot, plus claire
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.11, 0.15, -0.2], [0.08, 0.13, 0.12], BEAR, BEAR2); p.blob([sx * 0.11, 0.17, 0.2], [0.08, 0.13, 0.11], BEAR, BEAR2); }
        p.blob([0.0, 0.2, -0.36], [0.035, 0.035, 0.03], BEAR, BEAR); }
    let mut bear_head = VBuf::new(6000); { let mut p = Pen::new(&mut bear_head, Mat::Fur, 0.025);
        p.blob([0.0, 0.05, 0.04], [0.125, 0.105, 0.1], BEAR, BEAR2);
        p.tube([0.0, 0.01, 0.1], [0.0, -0.005, 0.25], [0.065, 0.055], [0.048, 0.042], MUZ, MUZ);
        p.blob_m([0.0, 0.015, 0.255], [0.03, 0.022, 0.018], NOSE, NOSE, Mat::Plain); p.bm([-0.035, -0.045, 0.16, 0.035, -0.035, 0.24], [0.2, 0.12, 0.1], Mat::Plain);
        for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.065, 0.07, 0.12, 0.013, [0.08, 0.05, 0.03]); p.blob([sx * 0.095, 0.15, -0.0], [0.04, 0.04, 0.025], BEAR, BEAR2); p.bm([sx * 0.095 - 0.02, 0.13, 0.024, sx * 0.095 + 0.02, 0.165, 0.03], BEAR2, Mat::Plain); } }
    let mut bear_leg = VBuf::new(2000); { let mut p = Pen::new(&mut bear_leg, Mat::Fur, 0.025);
        p.tube([0.0, 0.04, 0.0], [0.0, -0.2, 0.01], [0.075, 0.085], [0.06, 0.065], BEAR, BEAR2);
        p.blob([0.0, -0.225, 0.02], [0.066, 0.035, 0.08], BEAR2, BEAR2);
        for k in 0..4 { let x = -0.045 + k as f64 * 0.03; p.bm([x - 0.007, -0.255, 0.085, x + 0.007, -0.24, 0.115], [0.78, 0.74, 0.62], Mat::Plain); } }
    out.extend([("bear", bear), ("bearHead", bear_head), ("bearLeg", bear_leg)]);
    // ---- Bouquetin : élancé, robe fauve, ventre crème, barbiche, grandes cornes noueuses en arc.
    const IB: C = [0.6, 0.52, 0.4]; const IB2: C = [0.45, 0.38, 0.29]; const IBB: C = [0.85, 0.8, 0.7]; const HORN: C = [0.45, 0.4, 0.33];
    let mut ibex = VBuf::new(6000); { let mut p = Pen::new(&mut ibex, Mat::Fur, 0.03);
        p.body(&[(-0.28, 0.07, 0.18, 0.05), (-0.2, 0.1, 0.21, -0.0), (0.05, 0.095, 0.21, -0.01), (0.2, 0.095, 0.23, 0.01), (0.27, 0.07, 0.2, 0.06)], IB2, IB, IBB);
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.06, 0.11, -0.18], [0.045, 0.09, 0.09], IB, IB2); p.blob([sx * 0.06, 0.12, 0.17], [0.045, 0.08, 0.07], IB, IB2); }
        p.tube([0.0, 0.17, -0.28], [0.0, 0.14, -0.33], [0.025, 0.02], [0.02, 0.015], dk(IB2, 0.6), dk(IB2, 0.6)); }
    let mut ibex_head = VBuf::new(6000); { let mut p = Pen::new(&mut ibex_head, Mat::Fur, 0.022);
        p.tube([0.0, -0.03, -0.03], [0.0, 0.08, 0.03], [0.05, 0.06], [0.042, 0.05], IB, IB);
        p.blob([0.0, 0.08, 0.06], [0.055, 0.058, 0.07], IB, IB2);
        p.tube([0.0, 0.06, 0.1], [0.0, 0.025, 0.2], [0.04, 0.04], [0.028, 0.026], IB2, IB2); p.bm([-0.02, 0.01, 0.2, 0.02, 0.04, 0.21], NOSE, Mat::Plain);
        p.tube_m([0.0, 0.01, 0.15], [0.0, -0.08, 0.14], [0.018, 0.012], [0.008, 0.006], [0.3, 0.25, 0.2], [0.25, 0.2, 0.16], Mat::Hair); // barbiche
        for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.052, 0.1, 0.09, 0.011, [0.55, 0.42, 0.12]);
            p.tube([sx * 0.05, 0.1, 0.0], [sx * 0.11, 0.09, -0.02], [0.016, 0.024], [0.01, 0.016], IB, IB2); // oreilles
            // corne en arc rejetée vers l'arrière, avec ses nœuds
            let (r, a0, a1): (f64, f64, f64) = (0.16, 0.25, 3.0); let (cz, cy) = (0.02 - r * a0.cos(), 0.13 - r * a0.sin()); let n = 12;
            for k in 0..n { let (t0, t1) = (k as f64 / n as f64, (k + 1) as f64 / n as f64); let (b0, b1) = (a0 + (a1 - a0) * t0, a0 + (a1 - a0) * t1);
                let x0 = sx * (0.03 + 0.035 * t0); let x1 = sx * (0.03 + 0.035 * t1); let rr = 0.026 * (1.0 - t0 * 0.7);
                let c = if k % 2 == 0 { HORN } else { dk(HORN, 1.15) };
                p.tube_m([x0, cy + r * b0.sin(), cz + r * b0.cos()], [x1, cy + r * b1.sin(), cz + r * b1.cos()], [rr, rr], [rr * 0.92, rr * 0.92], c, c, Mat::Bone); } } }
    let mut ibex_leg = VBuf::new(1500); { let mut p = Pen::new(&mut ibex_leg, Mat::Fur, 0.022);
        p.tube([0.0, 0.03, 0.0], [0.0, -0.16, 0.0], [0.035, 0.045], [0.022, 0.026], IB, IB2); p.tube([0.0, -0.16, 0.0], [0.0, -0.3, 0.005], [0.02, 0.022], [0.018, 0.02], dk(IB2, 0.7), dk(IB2, 0.6));
        p.bm([-0.024, -0.345, -0.02, 0.024, -0.3, 0.035], [0.12, 0.1, 0.08], Mat::Bone); }
    out.extend([("ibex", ibex), ("ibexHead", ibex_head), ("ibexLeg", ibex_leg)]);
    // ---- Lézard ocellé : vert olive moucheté, écailles, pattes écartées aux doigts fins.
    const SC: C = [0.5, 0.52, 0.3]; const SC2: C = [0.36, 0.4, 0.2]; const SCB: C = [0.72, 0.7, 0.48];
    let mut lizard = VBuf::new(6000); { let mut p = Pen::new(&mut lizard, Mat::Scale, 0.014);
        p.body(&[(-0.1, 0.03, 0.03, 0.0), (-0.04, 0.045, 0.042, 0.0), (0.06, 0.042, 0.04, 0.0), (0.11, 0.03, 0.034, 0.002), (0.13, 0.025, 0.032, 0.004)], SC2, SC, SCB);
        p.body(&[(0.12, 0.03, 0.035, 0.004), (0.16, 0.032, 0.036, 0.004), (0.2, 0.02, 0.024, 0.006)], SC2, SC, SCB); // tête
        p.body(&[(-0.1, 0.028, 0.03, 0.0), (-0.18, 0.018, 0.022, 0.002), (-0.26, 0.009, 0.013, 0.002), (-0.3, 0.004, 0.008, 0.002)], SC2, SC, SCB); // queue
        for (i, z) in [-0.06, -0.02, 0.02, 0.06].iter().enumerate() { for sx in [-1.0f64, 1.0] { if (i + (sx > 0.0) as usize) % 2 == 0 { p.bm([sx * 0.02 - 0.008, 0.04, z - 0.008, sx * 0.02 + 0.008, 0.046, z + 0.008], [0.22, 0.42, 0.55], Mat::Plain); } } } // ocelles bleues
        for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.024, 0.026, 0.165, 0.006, BLACK);
            for z in [0.07, -0.07] { p.tube([sx * 0.035, 0.018, z], [sx * 0.075, 0.012, z + 0.02], [0.008, 0.008], [0.006, 0.006], SC, SC2); p.tube([sx * 0.075, 0.012, z + 0.02], [sx * 0.085, 0.0, z + 0.028], [0.006, 0.006], [0.005, 0.005], SC2, SC2);
                for t in -1..=1 { let a = t as f64 * 0.5; p.bm([sx * 0.085 + a.sin() * 0.012 - 0.002, 0.0, z + 0.028 + a.cos() * 0.01, sx * 0.085 + a.sin() * 0.012 + 0.002, 0.003, z + 0.04 + a.cos() * 0.01], SC2, Mat::Plain); } } } }
    out.push(("lizard", lizard));
    out
}

/// Maquereau, poissons de récif, crabe, goéland (et son aile).
pub fn sea(gull_k: f64) -> Vec<(&'static str, VBuf)> {
    // Un poisson fuselé : profil du dos et du ventre, bandes éventuelles, opercule, œil, nageoires fines, caudale fourchue.
    let fish = |len: f64, hgt: f64, back: C, belly: C, band: Option<C>, tail: C, fin: C| { let mut o = VBuf::new(5000); { let mut p = Pen::new(&mut o, Mat::Scale, 0.017);
        let (l, h, wd) = (len * 0.5, hgt * 0.5, hgt * 0.3);
        p.body(&[(-l, wd * 0.25, h * 0.22, -h * 0.22), (-l * 0.6, wd * 0.6, h * 0.62, -h * 0.6), (-l * 0.1, wd, h, -h), (l * 0.45, wd * 0.9, h * 0.88, -h * 0.85), (l * 0.8, wd * 0.6, h * 0.55, -h * 0.5), (l, wd * 0.3, h * 0.2, -h * 0.15)], dk(back, 0.85), mix(back, belly, 0.3), belly);
        if let Some(c) = band { for zc in [-0.3, 0.2] { let z = l * zc; p.bx([-wd * 1.02, -h * 0.9, z, wd * 1.02, h * 0.95, z + len * 0.09], c); } }
        for sx in [-1.0f64, 1.0] { p.bm([sx * wd * 0.95 - 0.003, -h * 0.5, l * 0.45, sx * wd * 0.95 + 0.003, h * 0.5, l * 0.5], dk(back, 0.7), Mat::Plain); // opercule
            eye(&mut p, sx * wd * 0.75, h * 0.3, l * 0.68, (h * 0.18).max(0.008), BLACK); p.bm([sx * wd * 0.9 - 0.004, h * 0.18, l * 0.6, sx * wd * 0.9 + 0.004, h * 0.42, l * 0.76], [0.95, 0.9, 0.7], Mat::Plain);
            p.bx([sx * wd * 0.9, -h * 0.35, l * 0.2, sx * (wd * 1.7), -h * 0.15, l * 0.4], fin); } // pectorales
        for k in 0..5 { let z = -l * 0.45 + k as f64 * l * 0.16; let hh = h * (0.35 + 0.15 * (2.0 - (k as f64 - 2.0).abs())); p.bx([-0.004, h * 0.8, z, 0.004, h * 0.8 + hh, z + l * 0.16], fin); } // dorsale en rayons
        p.bx([-0.004, -h * 0.85 - h * 0.35, -l * 0.5, 0.004, -h * 0.75, -l * 0.15], fin);
        for (s, k) in [(1.0, 0), (-1.0, 1)] { let _ = k; p.tube([0.0, 0.0, -l * 0.95], [0.0, s * h * 0.95, -l * 1.55], [0.005, h * 0.18], [0.004, h * 0.12], tail, dk(tail, 0.8)); } } o };
    let mackerel = { let mut o = fish(0.42, 0.11, [0.2, 0.42, 0.42], [0.86, 0.88, 0.9], None, [0.24, 0.32, 0.34], [0.3, 0.4, 0.42]); let mut p = Pen::new(&mut o, Mat::Scale, 0.017);
        for k in 0..7 { let z = -0.17 + k as f64 * 0.05; let w = 0.02 + 0.008 * (3.0 - (k as f64 - 3.0).abs()); p.bm([-w, 0.035, z, w, 0.058, z + 0.018], [0.06, 0.14, 0.16], Mat::Plain); } o }; // zébrures sombres
    let clown = fish(0.26, 0.13, [0.98, 0.45, 0.1], [0.98, 0.55, 0.2], Some(WHITE), [0.98, 0.45, 0.1], [0.98, 0.5, 0.15]);
    let tang = fish(0.3, 0.17, [0.12, 0.3, 0.85], [0.18, 0.38, 0.9], None, [0.98, 0.84, 0.15], [0.06, 0.1, 0.3]);
    let butterfly = fish(0.24, 0.22, [0.98, 0.86, 0.2], [0.98, 0.92, 0.6], Some([0.1, 0.08, 0.06]), [0.98, 0.86, 0.2], [0.95, 0.7, 0.15]);
    // Crabe : carapace bombée et grenue, yeux sur pédoncules, deux pinces, six pattes articulées.
    let mut crab = VBuf::new(8000); { let mut p = Pen::new(&mut crab, Mat::Chitin, 0.022); let (sh, dk_, cl) = ([0.82, 0.32, 0.14], [0.6, 0.2, 0.1], [0.9, 0.42, 0.2]);
        p.blob([0.0, 0.09, 0.0], [0.15, 0.05, 0.11], cl, [0.9, 0.78, 0.6]); p.blob([0.0, 0.12, -0.01], [0.11, 0.025, 0.08], sh, sh);
        for sx in [-1.0f64, 1.0] { p.tube([sx * 0.04, 0.12, 0.08], [sx * 0.05, 0.2, 0.09], [0.01, 0.01], [0.009, 0.009], dk_, dk_); p.blob_m([sx * 0.05, 0.21, 0.09], [0.017, 0.017, 0.017], BLACK, BLACK, Mat::Plain);
            p.tube([sx * 0.11, 0.08, 0.08], [sx * 0.17, 0.08, 0.15], [0.022, 0.02], [0.02, 0.018], dk_, sh);
            p.blob([sx * 0.17, 0.09, 0.2], [0.04, 0.035, 0.06], cl, sh); p.tube([sx * 0.155, 0.1, 0.24], [sx * 0.14, 0.12, 0.3], [0.012, 0.012], [0.007, 0.007], dk_, BLACK); p.tube([sx * 0.19, 0.08, 0.24], [sx * 0.18, 0.07, 0.29], [0.011, 0.011], [0.007, 0.007], dk_, BLACK);
            for k in 0..3 { let z = -0.07 + k as f64 * 0.06; p.tube([sx * 0.12, 0.08, z], [sx * 0.24, 0.11, z - 0.01], [0.013, 0.013], [0.011, 0.011], sh, dk_); p.tube([sx * 0.24, 0.11, z - 0.01], [sx * 0.29, 0.0, z - 0.02], [0.01, 0.01], [0.006, 0.006], dk_, dk(dk_, 0.7)); } } }
    // Goéland : blanc, manteau gris, bec jaune à tache rouge, bouts d'ailes noirs.
    let (mut gull, gull_wing) = bird_parts(gull_k, [0.62, 0.65, 0.7], WHITE, WHITE, [0.95, 0.8, 0.2], [0.64, 0.67, 0.72], Some(BLACK));
    { let mut p = Pen::new(&mut gull, Mat::Feather, 0.016 * gull_k); let k = gull_k; p.blob([0.0, 0.12 * k, 0.1 * k], [0.047 * k, 0.046 * k, 0.049 * k], WHITE, WHITE); for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.042 * k, 0.13 * k, 0.12 * k, 0.007 * k, BLACK); }
        p.bm([-0.005 * k, 0.1 * k, 0.175 * k, 0.005 * k, 0.108 * k, 0.185 * k], [0.85, 0.2, 0.15], Mat::Plain); }
    vec![("mackerel", mackerel), ("reef0", clown), ("reef1", tang), ("reef2", butterfly), ("crab", crab), ("gull", gull), ("gullWing", gull_wing)]
}

/// Les bêtes de ferme, la selle et le collier. Mêmes conventions que les autres : le corps à hauteur de hanche, les
/// pattes qui pendent depuis la hanche, la tête depuis la base du cou, tout tourné vers +z ; trois robes par espèce.
pub fn farm() -> Vec<(&'static str, VBuf)> {
    let mut out = Vec::new();
    const PINK: C = [0.86, 0.6, 0.58]; const HOOF: C = [0.16, 0.13, 0.11]; const HORN: C = [0.86, 0.82, 0.7];
    // ---- Vache : pie noire, rousse, brune ; mamelle rose, cornes courtes, mufle clair, queue à touffe.
    for (k, (base, patch)) in [(WHITE, BLACK), ([0.62, 0.34, 0.18], WHITE), ([0.4, 0.27, 0.18], [0.3, 0.2, 0.13])].into_iter().enumerate() {
        let mut cow = VBuf::new(9000); { let mut p = Pen::new(&mut cow, Mat::Fur, 0.035);
            p.body(&[(-0.4, 0.13, 0.3, 0.02), (-0.3, 0.17, 0.33, -0.02), (0.0, 0.185, 0.34, -0.05), (0.2, 0.175, 0.34, -0.03), (0.36, 0.14, 0.33, 0.05)], dk(base, 0.9), base, base);
            for (x, y, z, r) in [(0.17, 0.2, -0.15, 0.1), (-0.17, 0.16, 0.12, 0.11), (0.16, 0.12, 0.2, 0.08), (-0.15, 0.24, -0.3, 0.08), (0.0, 0.33, 0.05, 0.09)] { p.blob([x, y, z], [r * 0.5, r, r], patch, patch); } // les taches
            p.blob_m([0.0, -0.05, -0.22], [0.08, 0.05, 0.08], PINK, PINK, Mat::Flesh); // la mamelle
            p.tube([0.0, 0.3, -0.4], [0.0, -0.05, -0.47], [0.018, 0.018], [0.014, 0.014], base, base); p.blob([0.0, -0.08, -0.47], [0.03, 0.05, 0.03], patch, patch); }
        let mut head = VBuf::new(6000); { let mut p = Pen::new(&mut head, Mat::Fur, 0.025);
            p.blob([0.0, 0.05, 0.06], [0.1, 0.11, 0.13], base, base); p.blob([0.04, 0.09, 0.1], [0.05, 0.05, 0.05], patch, patch);
            p.blob_m([0.0, -0.03, 0.2], [0.075, 0.06, 0.06], PINK, PINK, Mat::Flesh); for sx in [-1.0f64, 1.0] { p.bm([sx * 0.03 - 0.01, -0.02, 0.255, sx * 0.03 + 0.01, 0.0, 0.262], NOSE, Mat::Plain); }
            for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.085, 0.09, 0.12, 0.015, [0.1, 0.06, 0.04]); p.tube_m([sx * 0.07, 0.16, 0.04], [sx * 0.14, 0.2, 0.06], [0.016, 0.016], [0.008, 0.008], HORN, HORN, Mat::Bone);
                p.blob([sx * 0.13, 0.1, 0.0], [0.05, 0.025, 0.035], base, base); } }
        let mut leg = VBuf::new(1500); { let mut p = Pen::new(&mut leg, Mat::Fur, 0.025); p.tube([0.0, 0.04, 0.0], [0.0, -0.27, 0.0], [0.055, 0.065], [0.035, 0.038], base, mix(base, patch, 0.3)); p.bm([-0.038, -0.33, -0.035, 0.038, -0.27, 0.045], HOOF, Mat::Bone); }
        out.extend([(["cow0", "cow1", "cow2"][k], cow), (["cowHead0", "cowHead1", "cowHead2"][k], head), (["cowLeg0", "cowLeg1", "cowLeg2"][k], leg)]);
    }
    // ---- Mouton : une toison bouclée (blanche, crème, brune), tondu une peau rase ; la tête et les pattes sombres ou claires.
    for (k, (wool, face)) in [([0.93, 0.91, 0.86], [0.12, 0.1, 0.09]), ([0.86, 0.8, 0.68], [0.82, 0.76, 0.66]), ([0.42, 0.32, 0.24], [0.2, 0.15, 0.12])].into_iter().enumerate() {
        let mut sheep = VBuf::new(9000); { let mut p = Pen::new(&mut sheep, Mat::Wool, 0.03);
            p.body(&[(-0.3, 0.14, 0.3, 0.0), (-0.2, 0.18, 0.34, -0.04), (0.0, 0.19, 0.35, -0.06), (0.18, 0.18, 0.34, -0.04), (0.3, 0.14, 0.3, 0.02)], wool, wool, dk(wool, 0.85));
            for (x, y, z) in [(0.12, 0.3, -0.1), (-0.12, 0.28, 0.1), (0.0, 0.34, 0.0), (0.1, 0.2, 0.22), (-0.1, 0.22, -0.22)] { p.blob([x, y, z], [0.08, 0.07, 0.08], wool, wool); } }
        let mut shorn = VBuf::new(6000); { let mut p = Pen::new(&mut shorn, Mat::Skin, 0.03);
            let skin = mix(wool, PINK, 0.6); p.body(&[(-0.27, 0.09, 0.24, 0.04), (-0.18, 0.12, 0.27, 0.0), (0.0, 0.125, 0.27, -0.01), (0.16, 0.12, 0.28, 0.0), (0.27, 0.09, 0.25, 0.05)], skin, skin, skin); }
        let mut head = VBuf::new(4000); { let mut p = Pen::new(&mut head, Mat::Fur, 0.022);
            p.blob([0.0, 0.04, 0.06], [0.07, 0.08, 0.11], face, face); p.blob_m([0.0, 0.1, 0.0], [0.08, 0.05, 0.07], wool, wool, Mat::Wool);
            for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.055, 0.06, 0.1, 0.012, [0.5, 0.4, 0.1]); p.tube([sx * 0.06, 0.07, 0.02], [sx * 0.15, 0.05, 0.0], [0.03, 0.012], [0.025, 0.01], face, face); } }
        let mut leg = VBuf::new(1200); { let mut p = Pen::new(&mut leg, Mat::Fur, 0.02); p.tube([0.0, 0.0, 0.0], [0.0, -0.28, 0.0], [0.03, 0.03], [0.024, 0.024], face, face); p.bm([-0.026, -0.31, -0.024, 0.026, -0.27, 0.034], HOOF, Mat::Bone); }
        out.extend([(["sheep0", "sheep1", "sheep2"][k], sheep), (["sheepShorn0", "sheepShorn1", "sheepShorn2"][k], shorn), (["sheepHead0", "sheepHead1", "sheepHead2"][k], head), (["sheepLeg0", "sheepLeg1", "sheepLeg2"][k], leg)]);
    }
    // ---- Cochon : rose, tacheté de noir ou brun ; groin plat, oreilles tombantes, queue en tire-bouchon.
    for (k, (skin, spot)) in [([0.9, 0.66, 0.62], [0.9, 0.66, 0.62]), ([0.88, 0.7, 0.64], [0.15, 0.12, 0.12]), ([0.5, 0.34, 0.26], [0.4, 0.26, 0.2])].into_iter().enumerate() {
        let mut pig = VBuf::new(7000); { let mut p = Pen::new(&mut pig, Mat::Skin, 0.03);
            p.body(&[(-0.32, 0.12, 0.27, 0.02), (-0.24, 0.17, 0.31, -0.02), (0.0, 0.18, 0.32, -0.04), (0.18, 0.17, 0.31, -0.02), (0.3, 0.13, 0.28, 0.03)], dk(skin, 0.92), skin, mix(skin, WHITE, 0.15));
            for (x, y, z) in [(0.16, 0.18, -0.1), (-0.15, 0.2, 0.1), (0.0, 0.3, -0.2)] { p.blob([x, y, z], [0.05, 0.07, 0.08], spot, spot); }
            for i in 0..6 { let a = i as f64 * 1.1; p.bm([0.025 * a.cos() - 0.012, 0.24 + i as f64 * 0.012, -0.35 - 0.025 * a.sin() - 0.012, 0.025 * a.cos() + 0.012, 0.25 + i as f64 * 0.012, -0.35 - 0.025 * a.sin() + 0.012], skin, Mat::Skin); } } // la queue en tire-bouchon
        let mut head = VBuf::new(4000); { let mut p = Pen::new(&mut head, Mat::Skin, 0.022);
            p.blob([0.0, 0.03, 0.06], [0.1, 0.1, 0.1], skin, skin); p.tube([0.0, 0.0, 0.12], [0.0, -0.01, 0.2], [0.055, 0.045], [0.05, 0.042], skin, skin);
            p.bm([-0.05, -0.05, 0.2, 0.05, 0.035, 0.215], mix(skin, [0.8, 0.45, 0.45], 0.5), Mat::Flesh); for sx in [-1.0f64, 1.0] { p.bm([sx * 0.02 - 0.009, -0.015, 0.215, sx * 0.02 + 0.009, 0.005, 0.22], NOSE, Mat::Plain);
                eye(&mut p, sx * 0.06, 0.07, 0.13, 0.011, [0.08, 0.05, 0.04]); p.bm([sx * 0.07 - 0.035, 0.08, 0.04, sx * 0.07 + 0.035, 0.1, 0.12], dk(skin, 0.9), Mat::Skin); } } // oreilles tombantes
        let mut leg = VBuf::new(1000); { let mut p = Pen::new(&mut leg, Mat::Skin, 0.02); p.tube([0.0, 0.03, 0.0], [0.0, -0.17, 0.0], [0.045, 0.05], [0.035, 0.038], skin, skin); p.bm([-0.036, -0.2, -0.032, 0.036, -0.17, 0.04], HOOF, Mat::Bone); }
        out.extend([(["pig0", "pig1", "pig2"][k], pig), (["pigHead0", "pigHead1", "pigHead2"][k], head), (["pigLeg0", "pigLeg1", "pigLeg2"][k], leg)]);
    }
    // ---- Cheval : bai, alezan, gris pommelé ; longue encolure et tête, crinière et queue de crins, balzanes et sabots.
    for (k, (coat, hair)) in [([0.4, 0.22, 0.12], [0.08, 0.06, 0.05]), ([0.62, 0.33, 0.15], [0.7, 0.42, 0.2]), ([0.72, 0.72, 0.7], [0.3, 0.3, 0.3])].into_iter().enumerate() {
        let mut horse = VBuf::new(10000); { let mut p = Pen::new(&mut horse, Mat::Fur, 0.035);
            p.body(&[(-0.42, 0.12, 0.3, 0.04), (-0.32, 0.165, 0.33, 0.0), (0.0, 0.165, 0.32, -0.01), (0.22, 0.17, 0.34, 0.0), (0.4, 0.13, 0.33, 0.06)], dk(coat, 0.85), coat, mix(coat, WHITE, 0.1));
            if k == 2 { for (x, y, z) in [(0.15, 0.2, -0.2), (-0.15, 0.22, 0.05), (0.14, 0.15, 0.25), (-0.12, 0.3, -0.3)] { p.blob([x, y, z], [0.03, 0.06, 0.06], dk(coat, 0.7), dk(coat, 0.7)); } } // pommelures
            p.tube_m([0.0, 0.3, -0.43], [0.0, 0.0, -0.52], [0.04, 0.03], [0.03, 0.02], hair, hair, Mat::Hair); p.tube_m([0.0, 0.0, -0.52], [0.0, -0.18, -0.5], [0.03, 0.02], [0.015, 0.012], hair, hair, Mat::Hair); }
        let mut head = VBuf::new(7000); { let mut p = Pen::new(&mut head, Mat::Fur, 0.025);
            p.tube([0.0, 0.0, 0.0], [0.0, 0.32, 0.16], [0.09, 0.1], [0.06, 0.07], coat, coat); // l'encolure, qui monte vers l'avant
            p.tube([0.0, 0.34, 0.12], [0.0, 0.22, 0.38], [0.065, 0.075], [0.04, 0.05], coat, dk(coat, 0.85)); p.blob_m([0.0, 0.22, 0.38], [0.045, 0.045, 0.03], dk(coat, 0.6), dk(coat, 0.6), Mat::Plain);
            for i in 0..8 { let t = i as f64 / 7.0; p.bm([-0.025, 0.06 + 0.3 * t, -0.03 + 0.18 * t - 0.03, 0.025, 0.12 + 0.3 * t, -0.03 + 0.18 * t + 0.03], hair, Mat::Hair); } // la crinière
            if k != 2 { p.bm([-0.02, 0.3, 0.22, 0.02, 0.33, 0.33], WHITE, Mat::Plain); } // la liste blanche
            for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.06, 0.33, 0.2, 0.014, [0.08, 0.05, 0.04]); ear(&mut p, sx * 0.04, 0.4, 0.12, 0.035, 0.08, 0.01, coat, dk(coat, 0.7), dk(coat, 0.7)); } }
        let mut leg = VBuf::new(1500); { let mut p = Pen::new(&mut leg, Mat::Fur, 0.025); let sock = if k == 0 { WHITE } else { coat };
            p.tube([0.0, 0.04, 0.0], [0.0, -0.22, 0.0], [0.055, 0.065], [0.03, 0.034], coat, coat); p.tube([0.0, -0.22, 0.0], [0.0, -0.4, 0.01], [0.03, 0.032], [0.028, 0.03], coat, sock); p.bm([-0.036, -0.45, -0.032, 0.036, -0.4, 0.045], HOOF, Mat::Bone); }
        out.extend([(["horse0", "horse1", "horse2"][k], horse), (["horseHead0", "horseHead1", "horseHead2"][k], head), (["horseLeg0", "horseLeg1", "horseLeg2"][k], leg)]);
    }
    // ---- Poule : blanche, rousse, noire ; crête et barbillons rouges, bec jaune, queue relevée, pattes jaunes.
    for (k, plume) in [[0.94, 0.92, 0.86], [0.62, 0.32, 0.14], [0.12, 0.12, 0.14]].into_iter().enumerate() {
        let mut hen = VBuf::new(5000); { let mut p = Pen::new(&mut hen, Mat::Feather, 0.018);
            p.blob([0.0, 0.14, 0.0], [0.08, 0.08, 0.11], plume, mix(plume, WHITE, 0.2)); p.blob([0.0, 0.22, 0.08], [0.05, 0.06, 0.05], plume, plume);
            for (dz, dy, w) in [(-0.1, 0.2, 0.035), (-0.12, 0.24, 0.03)] { p.bm([-w, 0.14 + dy * 0.2, dz - 0.05, w, 0.14 + dy, dz + 0.02], dk(plume, 0.8), Mat::Feather); } // la queue
            p.bm([-0.012, 0.29, 0.06, 0.012, 0.33, 0.12], [0.8, 0.1, 0.08], Mat::Flesh); p.bm([-0.008, 0.18, 0.12, 0.008, 0.22, 0.135], [0.8, 0.1, 0.08], Mat::Flesh); // crête, barbillons
            p.bm([-0.012, 0.22, 0.12, 0.012, 0.24, 0.16], [0.9, 0.72, 0.2], Mat::Plain); // le bec
            for sx in [-1.0f64, 1.0] { eye(&mut p, sx * 0.035, 0.25, 0.1, 0.008, [0.1, 0.06, 0.02]); p.bm([sx * 0.04 - 0.005, 0.0, -0.005, sx * 0.04 + 0.005, 0.08, 0.005], [0.9, 0.72, 0.2], Mat::Plain); p.bm([sx * 0.04 - 0.012, 0.0, -0.01, sx * 0.04 + 0.012, 0.008, 0.04], [0.9, 0.72, 0.2], Mat::Plain); } }
        out.push((["hen0", "hen1", "hen2"][k], hen));
    }
    // ---- La selle de cuir, ses quartiers, sa sangle et ses étriers ; le collier rouge du loup apprivoisé.
    let mut saddle = VBuf::new(3000); { let mut p = Pen::new(&mut saddle, Mat::Leather, 0.02); let (lth, dark) = ([0.42, 0.24, 0.12], [0.24, 0.13, 0.06]);
        p.bx([-0.12, 0.0, -0.13, 0.12, 0.04, 0.13], lth); p.bx([-0.05, 0.04, 0.09, 0.05, 0.1, 0.13], dark); p.bx([-0.07, 0.04, -0.13, 0.07, 0.08, -0.1], dark);
        for sx in [-1.0f64, 1.0] { p.bx([sx * 0.12 - 0.012, -0.16, -0.08, sx * 0.12 + 0.012, 0.02, 0.08], lth); p.bm([sx * 0.13 - 0.005, -0.36, -0.005, sx * 0.13 + 0.005, -0.16, 0.005], dark, Mat::Leather); p.bm([sx * 0.13 - 0.02, -0.38, -0.03, sx * 0.13 + 0.02, -0.36, 0.03], [0.55, 0.55, 0.58], Mat::Metal); } }
    let mut collar = VBuf::new(800); { let mut p = Pen::new(&mut collar, Mat::Leather, 0.015); for (x0, x1, z0, z1) in [(-0.09, 0.09, 0.07, 0.09), (-0.09, 0.09, -0.09, -0.07), (-0.09, -0.07, -0.07, 0.07), (0.07, 0.09, -0.07, 0.07)] { p.bx([x0, -0.02, z0, x1, 0.02, z1], [0.75, 0.12, 0.1]); } p.bm([-0.015, -0.05, 0.085, 0.015, -0.02, 0.1], [0.85, 0.75, 0.3], Mat::Metal); }
    out.extend([("saddle", saddle), ("collar", collar)]);
    out
}
