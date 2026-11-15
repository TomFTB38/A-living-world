//! Les créatures des profondeurs, sculptées en volumes galbés (la méthode des dauphins) : araignée des cavernes, golem de
//! cristal, rampant de magma, braise, scarabée lumineux. Chaque pièce est modelée autour de son point d'attache.
use crate::hostile::Mat;
use crate::mesher::VBuf;
use crate::sculpt::{dk, Pen, C};
#[allow(unused_imports)] use crate::mesher::VSIZE;

const PLAIN: Mat = Mat::Plain;
/// Araignée : poil brun-gris, croix pâle sur l'abdomen, yeux rouges. Golem : roche violacée, cristaux clairs, cœur qui luit.
/// Rampant de magma : croûte de basalte, fissures et ventre incandescents.
const SPD: C = [0.2, 0.17, 0.15]; const SPD2: C = [0.33, 0.28, 0.24]; const SPDB: C = [0.42, 0.37, 0.32]; const SPDM: C = [0.78, 0.74, 0.62];
const ROCK: C = [0.34, 0.32, 0.4]; const ROCK2: C = [0.24, 0.22, 0.3]; const CRY: C = [0.78, 0.62, 0.98]; const CRY2: C = [0.95, 0.88, 1.0]; const CORE: C = [1.0, 0.95, 1.0];
const BAS: C = [0.13, 0.11, 0.11]; const BAS2: C = [0.24, 0.13, 0.09]; const FIRE: C = [1.0, 0.5, 0.1]; const FIRE2: C = [1.0, 0.82, 0.3];

pub fn parts() -> Vec<(&'static str, VBuf)> {
    let mut out: Vec<(&'static str, VBuf)> = Vec::new();
    let mut m = |n: &'static str, mat: Mat, pix: f64, f: &dyn Fn(&mut Pen)| { let mut v = VBuf::new(4000); { let mut p = Pen::new(&mut v, mat, pix); f(&mut p); } out.push((n, v)); };

    // ---- Araignée des cavernes : céphalothorax bas, gros abdomen velu marqué d'une croix, huit yeux, deux crochets.
    m("spdBody", Mat::Fur, 0.03, &|p| {
        p.blob([0.0, 0.02, -0.34], [0.3, 0.25, 0.38], SPD, SPDB); p.blob([0.0, 0.0, 0.16], [0.21, 0.14, 0.24], SPD2, SPDB);
        p.bm([-0.03, 0.245, -0.56, 0.03, 0.275, -0.14], SPDM, PLAIN); p.bm([-0.14, 0.235, -0.4, 0.14, 0.265, -0.34], SPDM, PLAIN); // la croix
        for (x, y, s) in [(-0.1, 0.07, 0.035), (0.065, 0.07, 0.035), (-0.045, 0.1, 0.03), (0.015, 0.1, 0.03), (-0.15, 0.03, 0.025), (0.125, 0.03, 0.025)] { p.bm([x, y, 0.37, x + s, y + s, 0.4], [0.85, 0.1, 0.08], PLAIN); }
        for sx in [-1.0f64, 1.0] { p.tube([sx * 0.07, -0.02, 0.36], [sx * 0.08, -0.1, 0.46], [0.04, 0.04], [0.03, 0.03], SPD2, SPD); p.tube([sx * 0.08, -0.1, 0.46], [sx * 0.04, -0.2, 0.44], [0.025, 0.025], [0.01, 0.01], [0.1, 0.07, 0.06], [0.05, 0.03, 0.03]); }
    });
    // Une patte, vers +x depuis la hanche : elle monte jusqu'au genou, puis plonge jusqu'au sol, fine et annelée.
    m("spdLeg", Mat::Fur, 0.03, &|p| {
        p.tube([0.0, 0.0, 0.0], [0.34, 0.24, 0.0], [0.05, 0.05], [0.04, 0.04], SPD2, SPD); p.blob([0.35, 0.25, 0.0], [0.055, 0.055, 0.055], SPDB, SPD2);
        p.tube([0.36, 0.24, 0.0], [0.62, -0.08, 0.0], [0.038, 0.038], [0.028, 0.028], SPD, SPD2); p.tube([0.62, -0.08, 0.0], [0.7, -0.3, 0.0], [0.026, 0.026], [0.012, 0.012], SPD2, [0.08, 0.06, 0.05]);
    });

    // ---- Golem de cristal : un torse de roche en tonneau, le cœur de cristal qui luit dans la poitrine, des aiguilles sur le dos et les épaules.
    m("glmTorso", Mat::Grain, 0.045, &|p| {
        p.column_m(&[(0.0, 0.3, 0.23), (0.3, 0.4, 0.28), (0.6, 0.52, 0.32), (0.8, 0.5, 0.3), (0.9, 0.3, 0.2)], 0.0, ROCK2, ROCK, 0.7, Mat::Grain);
        p.bm([-0.1, 0.42, 0.27, 0.1, 0.66, 0.34], CRY, PLAIN); p.bm([-0.055, 0.47, 0.33, 0.055, 0.61, 0.36], CORE, PLAIN); // le cœur
        for (x, z, h, r) in [(-0.3, -0.2, 0.5, 0.07), (0.28, -0.22, 0.42, 0.06), (0.0, -0.28, 0.62, 0.08), (-0.46, 0.0, 0.3, 0.06), (0.46, 0.02, 0.34, 0.06), (0.16, -0.18, 0.3, 0.05)] {
            p.tube_m([x, 0.72, z], [x * 1.25, 0.72 + h, z * 1.5], [r, r], [0.012, 0.012], CRY, CRY2, PLAIN); }
        for (x, y) in [(-0.3, 0.2), (0.22, 0.3), (-0.12, 0.12)] { p.bx([x, y, 0.2, x + 0.14, y + 0.1, 0.3], dk(ROCK, 0.8)); } // des éclats de roche
    });
    m("glmHead", Mat::Grain, 0.04, &|p| {
        p.rbox([-0.17, 0.0, -0.15, 0.17, 0.27, 0.16], ROCK); p.bx([-0.19, 0.17, -0.12, 0.19, 0.24, 0.17], ROCK2); // l'arcade
        for sx in [-1.0f64, 1.0] { p.bm([sx * 0.085 - 0.04, 0.09, 0.15, sx * 0.085 + 0.04, 0.15, 0.175], CORE, PLAIN); }
        p.tube_m([0.0, 0.25, -0.04], [0.02, 0.5, -0.1], [0.06, 0.06], [0.01, 0.01], CRY, CRY2, PLAIN); p.tube_m([-0.1, 0.24, -0.02], [-0.18, 0.4, -0.08], [0.04, 0.04], [0.008, 0.008], CRY, CRY2, PLAIN);
    });
    // Un bras, depuis l'épaule vers le bas : le haut, puis un avant-bras énorme terminé par un poing hérissé de cristaux.
    m("glmArm", Mat::Grain, 0.045, &|p| {
        p.blob([0.0, 0.0, 0.0], [0.17, 0.15, 0.17], ROCK, ROCK2); p.tube([0.0, -0.05, 0.0], [0.0, -0.5, 0.0], [0.11, 0.11], [0.13, 0.13], ROCK, ROCK2);
        p.blob([0.0, -0.78, 0.02], [0.19, 0.3, 0.2], ROCK, ROCK2);
        for (x, z) in [(-0.09, 0.12), (0.03, 0.15), (0.11, 0.08)] { p.tube_m([x, -0.95, z], [x * 1.3, -1.12, z * 1.6], [0.04, 0.04], [0.008, 0.008], CRY, CRY2, PLAIN); }
    });
    m("glmLeg", Mat::Grain, 0.045, &|p| {
        p.tube([0.0, 0.0, 0.0], [0.0, -0.5, 0.0], [0.15, 0.15], [0.17, 0.17], ROCK, ROCK2); p.rbox([-0.19, -0.66, -0.16, 0.19, -0.48, 0.26], ROCK2);
        p.bm([-0.05, -0.3, 0.15, 0.05, -0.16, 0.19], CRY, PLAIN);
    });

    // ---- Rampant de magma : la silhouette du rampant, en croûte de basalte fendue ; le ventre et les fissures rougeoient.
    m("mgmBody", Mat::Grain, 0.035, &|p| {
        p.body(&[(-0.66, 0.1, 0.16, 0.05), (-0.46, 0.27, 0.38, 0.0), (-0.1, 0.38, 0.5, 0.0), (0.22, 0.34, 0.44, 0.0), (0.38, 0.22, 0.3, 0.04)], BAS, BAS2, FIRE);
        for z in [-0.4, -0.2, 0.0, 0.18] { p.bm([-0.3, 0.3, z, 0.3, 0.52, z + 0.03], FIRE, PLAIN); } // les fissures en travers
        p.bm([-0.02, 0.47, -0.44, 0.02, 0.53, 0.24], FIRE2, PLAIN);
        for (x, z) in [(-0.2, -0.3), (0.14, -0.1), (-0.12, 0.1), (0.2, 0.26)] { p.bx([x, 0.4, z, x + 0.12, 0.5, z + 0.12], dk(BAS, 1.4)); } // des plaques
    });
    m("mgmHead", Mat::Grain, 0.035, &|p| {
        p.blob([0.0, 0.1, 0.1], [0.22, 0.14, 0.16], BAS, BAS2);
        for (x, y) in [(-0.15, 0.1), (0.09, 0.1), (-0.08, 0.15), (0.03, 0.15)] { p.bm([x, y, 0.22, x + 0.06, y + 0.05, 0.25], FIRE2, PLAIN); }
        for sx in [-1.0f64, 1.0] { p.tube([sx * 0.1, 0.02, 0.2], [sx * 0.11, -0.08, 0.3], [0.035, 0.035], [0.025, 0.025], BAS2, BAS); p.tube_m([sx * 0.11, -0.08, 0.3], [sx * 0.06, -0.17, 0.26], [0.022, 0.022], [0.008, 0.008], FIRE, FIRE2, PLAIN); }
    });
    m("mgmLeg", Mat::Grain, 0.035, &|p| {
        p.tube([0.0, 0.0, 0.0], [0.28, 0.1, 0.0], [0.045, 0.045], [0.04, 0.04], BAS2, BAS); p.blob_m([0.3, 0.11, 0.0], [0.05, 0.05, 0.05], FIRE2, FIRE, PLAIN);
        p.tube([0.31, 0.1, 0.0], [0.43, -0.08, 0.0], [0.04, 0.04], [0.03, 0.03], BAS, BAS2); p.tube([0.43, -0.08, 0.0], [0.36, -0.28, 0.0], [0.028, 0.028], [0.012, 0.012], BAS2, BAS);
    });
    // Une braise au sol : une flaque incandescente, son cœur plus clair.
    m("ember", PLAIN, 0.05, &|p| { p.bx([-0.2, 0.0, -0.2, 0.2, 0.03, 0.2], FIRE); p.bx([-0.11, 0.02, -0.11, 0.11, 0.05, 0.11], FIRE2); p.bx([-0.28, 0.0, -0.06, -0.2, 0.02, 0.08], dk(FIRE, 0.8)); p.bx([0.2, 0.0, -0.1, 0.27, 0.02, 0.02], dk(FIRE, 0.8)); });

    // ---- Scarabée lumineux : élytres sombres à reflet bleu-vert, abdomen qui luit, six pattes, deux antennes.
    m("glowbug", Mat::Chitin, 0.014, &|p| {
        let (sh, sh2) = ([0.06, 0.22, 0.22], [0.03, 0.1, 0.12]); let glow = [0.55, 1.0, 0.9];
        p.blob([0.0, 0.07, -0.02], [0.085, 0.06, 0.12], sh, sh2); p.blob([0.0, 0.065, 0.11], [0.055, 0.04, 0.05], sh2, sh2);
        p.bm([-0.004, 0.125, -0.13, 0.004, 0.132, 0.08], sh2, PLAIN); // la suture des élytres
        p.blob_m([0.0, 0.06, -0.14], [0.065, 0.045, 0.05], glow, [0.3, 0.9, 0.8], PLAIN); p.bm([-0.05, 0.02, -0.1, 0.05, 0.035, 0.02], glow, PLAIN);
        for sx in [-1.0f64, 1.0] { for z in [-0.07, 0.01, 0.08] { p.tube([sx * 0.06, 0.05, z], [sx * 0.13, 0.0, z + 0.02 * sx.abs()], [0.01, 0.01], [0.006, 0.006], sh2, sh2); }
            p.tube([sx * 0.02, 0.09, 0.15], [sx * 0.06, 0.13, 0.21], [0.005, 0.005], [0.004, 0.004], sh2, sh2); }
    });
    // ---- Chauve-souris : petit corps velu, grandes oreilles, museau court ; l'aile est une main : un bras, trois doigts, la peau tendue entre eux.
    let k = crate::wild::BAT; let (fur, fur2, skin) = ([0.2, 0.15, 0.12], [0.34, 0.27, 0.22], [0.13, 0.09, 0.09]);
    m("bat", Mat::Fur, 0.016, &|p| {
        p.blob([0.0, 0.07 * k, 0.0], [0.06 * k, 0.06 * k, 0.1 * k], fur, fur2); p.blob([0.0, 0.085 * k, 0.11 * k], [0.05 * k, 0.045 * k, 0.05 * k], fur, fur2);
        p.bm([-0.02 * k, 0.065 * k, 0.15 * k, 0.02 * k, 0.09 * k, 0.175 * k], skin, PLAIN); // le museau
        for sx in [-1.0f64, 1.0] { p.tube_m([sx * 0.03 * k, 0.11 * k, 0.1 * k], [sx * 0.055 * k, 0.19 * k, 0.09 * k], [0.02 * k, 0.012 * k], [0.006 * k, 0.004 * k], skin, fur, PLAIN);
            p.bm([sx * 0.025 * k - 0.007 * k, 0.092 * k, 0.152 * k, sx * 0.025 * k + 0.007 * k, 0.106 * k, 0.16 * k], [0.02, 0.02, 0.02], PLAIN);
            p.bm([sx * 0.02 * k - 0.006 * k, 0.0, -0.06 * k, sx * 0.02 * k + 0.006 * k, 0.02 * k, -0.04 * k], skin, PLAIN); }
    });
    m("batWing", PLAIN, 0.02, &|p| {
        p.tube([0.0, 0.0, 0.02 * k], [0.13 * k, 0.012 * k, 0.05 * k], [0.012 * k, 0.012 * k], [0.009 * k, 0.009 * k], fur, skin); // le bras
        for (x1, z1) in [(0.36 * k, 0.02 * k), (0.3 * k, -0.07 * k), (0.2 * k, -0.12 * k)] { p.tube([0.13 * k, 0.012 * k, 0.05 * k], [x1, 0.0, z1], [0.006 * k, 0.006 * k], [0.004 * k, 0.004 * k], skin, skin); } // les doigts
        // la peau : des bandes de plus en plus courtes vers le bout, le bord arrière en festons
        for (x0, x1, z0, z1) in [(0.0, 0.07, -0.1, 0.05), (0.07, 0.14, -0.13, 0.06), (0.14, 0.21, -0.12, 0.05), (0.21, 0.28, -0.08, 0.04), (0.28, 0.34, -0.035, 0.03)] {
            p.bx([x0 * k, 0.0, z0 * k, x1 * k, 0.006 * k, z1 * k], dk(skin, 1.5)); }
    });
    surface(&mut out);
    out
}

/// Les créatures de surface : sylvain (un être d'écorce), noyé, archer décharné, fantôme, ver des sables, harpie.
/// Les quatre premiers partagent une charpente d'homme : torse depuis les hanches, tête, bras et jambe d'une pièce.
fn surface(out: &mut Vec<(&'static str, VBuf)>) {
    let mut m = |n: &'static str, mat: Mat, pix: f64, f: &dyn Fn(&mut Pen)| { let mut v = VBuf::new(4000); { let mut p = Pen::new(&mut v, mat, pix); f(&mut p); } out.push((n, v)); };
    let amber = [1.0, 0.75, 0.2];

    // ---- Sylvain : un tronc noueux pour torse, à l'écorce sillonnée ; un visage creusé dans le bois aux yeux d'ambre, des nœuds,
    // des champignons en console, de la mousse au pied ; des bras de branches aux doigts de brindilles, des jambes de racines.
    let (bark, bark2, moss, leaf) = ([0.4, 0.31, 0.22], [0.27, 0.2, 0.14], [0.3, 0.42, 0.16], [0.24, 0.5, 0.16]); let (heart, shroom) = ([0.62, 0.47, 0.3], [0.86, 0.74, 0.55]); let _ = (bark, bark2);
    // un nœud : un bourrelet d'écorce autour d'un cœur sombre
    let knot = |p: &mut Pen, c: [f64; 3], r: f64| { p.blob_m([c[0], c[1], c[2]], [r, r, r * 0.6], bark, bark2, Mat::Bark); p.bm([c[0] - r * 0.45, c[1] - r * 0.45, c[2] + r * 0.45, c[0] + r * 0.45, c[1] + r * 0.45, c[2] + r * 0.62], [0.09, 0.06, 0.04], PLAIN); };
    m("sylTorso", Mat::Bark, 0.026, &|p| {
        p.column_m(&[(0.0, 0.25, 0.2), (0.12, 0.29, 0.23), (0.4, 0.3, 0.23), (0.62, 0.34, 0.25), (0.86, 0.38, 0.26), (1.02, 0.32, 0.23), (1.15, 0.2, 0.16)], 0.0, bark2, bark, 0.68, Mat::Bark);
        // des contreforts d'écorce le long du tronc, comme des côtes
        for (x, z, y0, y1, w) in [(-0.2, 0.2, 0.05, 0.75, 0.06), (0.16, 0.21, 0.2, 0.95, 0.05), (0.0, 0.23, 0.0, 0.5, 0.05), (-0.26, -0.12, 0.1, 0.9, 0.06), (0.24, -0.14, 0.0, 0.7, 0.06), (0.0, -0.22, 0.2, 1.0, 0.07)] { p.tube([x, y0, z], [x * 1.12, y1, z * 1.08], [w, w * 0.7], [w * 0.7, w * 0.5], bark2, bark); }
        knot(p, [0.14, 0.42, 0.2], 0.075); knot(p, [-0.17, 0.78, 0.2], 0.06);
        for (x, y, w) in [(-0.32, 0.5, 0.11), (-0.3, 0.38, 0.14)] { p.bm([x - w, y, 0.02, x + 0.02, y + 0.03, 0.02 + w * 1.4], shroom, PLAIN); p.bm([x - w * 0.8, y - 0.015, 0.04, x, y, w * 1.2], dk(shroom, 0.7), PLAIN); } // des polypores
        for (x, z) in [(-0.2, 0.19), (0.05, 0.22), (0.2, 0.12), (-0.1, -0.2), (0.22, -0.16)] { p.bm([x - 0.06, 0.0, z - 0.03, x + 0.07, 0.09 + 0.05 * (x * 9.0).sin().abs(), z + 0.04], moss, Mat::Wool); } // la mousse au pied
        p.tube([0.22, 1.0, -0.05], [0.44, 1.36, -0.12], [0.05, 0.05], [0.02, 0.02], bark, bark2); p.blob_m([0.46, 1.42, -0.12], [0.15, 0.11, 0.15], leaf, dk(leaf, 0.65), Mat::Wool); p.blob_m([0.36, 1.5, -0.04], [0.1, 0.08, 0.1], [0.3, 0.56, 0.18], dk(leaf, 0.7), Mat::Wool);
        p.tube([-0.2, 1.02, -0.08], [-0.32, 1.22, -0.16], [0.04, 0.04], [0.012, 0.012], bark, bark2); // un chicot sec
    });
    m("sylHead", Mat::Bark, 0.024, &|p| {
        p.column_m(&[(0.0, 0.16, 0.15), (0.1, 0.2, 0.18), (0.24, 0.2, 0.18), (0.36, 0.14, 0.13)], 0.0, bark2, bark, 0.68, Mat::Bark);
        // le visage : des orbites creusées sous une arcade d'écorce, un nez en nœud, une bouche fendue qui montre le bois clair
        p.bm([-0.15, 0.2, 0.15, 0.15, 0.255, 0.2], bark2, Mat::Bark);
        for sx in [-1.0f64, 1.0] { p.bm([sx * 0.085 - 0.045, 0.13, 0.165, sx * 0.085 + 0.045, 0.2, 0.185], [0.06, 0.04, 0.03], PLAIN); p.bm([sx * 0.085 - 0.025, 0.145, 0.18, sx * 0.085 + 0.025, 0.185, 0.195], amber, PLAIN); }
        p.tube([0.0, 0.16, 0.17], [0.0, 0.08, 0.22], [0.025, 0.025], [0.035, 0.03], bark, bark2);
        p.bm([-0.09, 0.03, 0.165, 0.09, 0.065, 0.19], [0.07, 0.05, 0.03], PLAIN); for x in [-0.06, -0.02, 0.025] { p.bm([x, 0.045, 0.185, x + 0.025, 0.065, 0.195], heart, PLAIN); }
        for (x, z, h, a) in [(-0.1, -0.02, 0.3, -0.12), (0.06, 0.0, 0.36, 0.1), (0.0, -0.08, 0.26, 0.0)] { p.tube([x, 0.34, z], [x + a, 0.34 + h, z - 0.04], [0.03, 0.03], [0.01, 0.01], bark, bark2);
            p.tube([x + a * 0.5, 0.34 + h * 0.5, z - 0.02], [x + a * 0.5 + 0.1 * a.signum(), 0.34 + h * 0.75, z + 0.05], [0.014, 0.014], [0.006, 0.006], bark, bark2);
            p.blob_m([x + a, 0.38 + h, z - 0.04], [0.1, 0.075, 0.1], leaf, dk(leaf, 0.65), Mat::Wool); }
    });
    m("sylArm", Mat::Bark, 0.024, &|p| {
        p.blob([0.0, 0.0, 0.0], [0.11, 0.1, 0.11], bark, bark2); p.tube([0.0, 0.0, 0.0], [0.0, -0.55, 0.03], [0.09, 0.09], [0.07, 0.07], bark, bark2); p.blob([0.0, -0.56, 0.03], [0.085, 0.07, 0.085], bark, bark2);
        p.tube([0.0, -0.55, 0.03], [0.0, -1.0, 0.1], [0.07, 0.07], [0.05, 0.05], bark2, bark);
        for (x, z, l) in [(-0.08, 0.1, 0.24), (0.0, 0.17, 0.3), (0.08, 0.1, 0.24), (-0.03, 0.0, 0.16)] { p.tube([x * 0.5, -1.0, 0.1], [x, -1.0 - l, z + 0.06], [0.026, 0.026], [0.007, 0.007], bark, bark2); } // des doigts de brindilles
        p.tube([0.05, -0.3, 0.0], [0.2, -0.16, -0.04], [0.025, 0.025], [0.008, 0.008], bark, bark2); p.blob_m([0.22, -0.13, -0.04], [0.07, 0.05, 0.07], leaf, dk(leaf, 0.65), Mat::Wool); // un rameau
        p.bm([-0.1, -0.3, -0.06, -0.02, -0.18, 0.06], moss, Mat::Wool);
    });
    m("sylLeg", Mat::Bark, 0.026, &|p| {
        p.tube([0.0, 0.0, 0.0], [0.0, -0.7, 0.0], [0.12, 0.12], [0.1, 0.1], bark2, bark); p.blob([0.0, -0.36, 0.03], [0.11, 0.08, 0.12], bark, bark2); // le genou, un bourrelet
        for (x, z) in [(-0.16, 0.14), (0.13, 0.18), (0.0, -0.16), (0.18, -0.06), (-0.14, -0.08)] { p.tube([0.0, -0.66, 0.0], [x, -0.92, z], [0.065, 0.065], [0.022, 0.022], bark, bark2); } // les racines
        p.bm([-0.08, -0.9, 0.05, 0.06, -0.84, 0.16], moss, Mat::Wool);
    });

    // ---- La charpente d'homme, dans trois peaux : noyé (chair verdâtre gonflée, algues), archer (os et haillons), fantôme (linceul pâle).
    let human = |m: &mut dyn FnMut(&'static str, Mat, f64, &dyn Fn(&mut Pen)), names: [&'static str; 4], mat: Mat, body: C, body2: C, limb: C, eye: C, kind: u8| {
        m(names[0], mat, 0.035, &|p| {
            p.column_m(&[(0.0, 0.17, 0.11), (0.25, 0.19, 0.12), (0.5, 0.23, 0.13), (0.6, 0.17, 0.1)], 0.0, body2, body, 0.72, mat);
            match kind {
                0 => { for (x, y) in [(-0.16, 0.3), (0.05, 0.42), (-0.04, 0.12)] { p.bm([x, y, 0.1, x + 0.1, y + 0.05, 0.14], [0.16, 0.34, 0.2], PLAIN); } p.tube_m([0.12, 0.58, 0.0], [0.2, 0.2, 0.1], [0.03, 0.02], [0.015, 0.01], [0.14, 0.36, 0.2], [0.1, 0.26, 0.16], PLAIN); } // des algues
                1 => { for y in [0.3, 0.38, 0.46] { p.bm([-0.2, y, 0.105, 0.2, y + 0.03, 0.13], [0.9, 0.87, 0.78], PLAIN); } p.bm([-0.2, 0.0, -0.12, 0.2, 0.2, 0.13], [0.3, 0.2, 0.13], Mat::Cloth); p.bm([-0.05, 0.1, -0.2, 0.09, 0.62, -0.12], [0.36, 0.24, 0.14], Mat::Leather); } // côtes, pagne, carquois
                _ => { p.column_m(&[(-0.75, 0.04, 0.03), (-0.4, 0.13, 0.09), (0.0, 0.2, 0.13)], 0.0, dk(body2, 0.85), body2, 0.7, mat); } // le linceul qui s'effile : pas de jambes
            }
        });
        m(names[1], mat, 0.03, &|p| {
            p.column_m(&[(0.0, 0.1, 0.1), (0.1, 0.13, 0.13), (0.22, 0.12, 0.12), (0.28, 0.07, 0.07)], 0.0, body, body, 0.72, if kind == 1 { Mat::Bone } else { mat });
            for sx in [-1.0f64, 1.0] { p.bm([sx * 0.055 - 0.028, 0.12, 0.115, sx * 0.055 + 0.028, 0.17, 0.135], eye, PLAIN); }
            match kind { 0 => { p.bm([-0.05, 0.03, 0.105, 0.05, 0.06, 0.125], [0.05, 0.08, 0.07], PLAIN); p.tube_m([-0.1, 0.26, 0.0], [-0.16, 0.0, 0.04], [0.03, 0.02], [0.012, 0.01], [0.14, 0.36, 0.2], [0.1, 0.26, 0.16], PLAIN); }
                1 => { p.bm([-0.06, 0.02, 0.11, 0.06, 0.05, 0.13], [0.1, 0.08, 0.07], PLAIN); p.bm([-0.14, 0.2, -0.14, 0.14, 0.31, 0.1], [0.34, 0.14, 0.12], Mat::Cloth); } // un capuchon en lambeaux
                _ => { p.bm([-0.04, 0.02, 0.11, 0.04, 0.09, 0.13], [0.05, 0.07, 0.1], PLAIN); } }
        });
        m(names[2], mat, 0.03, &|p| { p.tube([0.0, 0.0, 0.0], [0.0, -0.3, 0.0], [0.055, 0.055], [0.045, 0.045], limb, body2); p.tube([0.0, -0.3, 0.0], [0.0, -0.58, 0.02], [0.045, 0.045], [0.035, 0.035], body2, limb); p.blob([0.0, -0.63, 0.03], [0.045, 0.06, 0.05], limb, body2); });
        m(names[3], mat, 0.035, &|p| { p.tube([0.0, 0.0, 0.0], [0.0, -0.42, 0.0], [0.075, 0.075], [0.06, 0.06], limb, body2); p.tube([0.0, -0.42, 0.0], [0.0, -0.8, 0.0], [0.06, 0.06], [0.045, 0.045], body2, limb); p.rbox([-0.06, -0.86, -0.07, 0.06, -0.78, 0.13], dk(limb, 0.8)); });
    };
    human(&mut m, ["noyTorso", "noyHead", "noyArm", "noyLeg"], Mat::Flesh, [0.36, 0.5, 0.44], [0.26, 0.38, 0.36], [0.4, 0.54, 0.46], [0.85, 0.95, 0.8], 0);
    human(&mut m, ["arcTorso", "arcHead", "arcArm", "arcLeg"], Mat::Bone, [0.82, 0.78, 0.68], [0.6, 0.56, 0.48], [0.86, 0.82, 0.72], [0.95, 0.3, 0.1], 1);
    human(&mut m, ["fanTorso", "fanHead", "fanArm", "fanLeg"], Mat::Cloth, [0.86, 0.93, 0.98], [0.62, 0.76, 0.9], [0.8, 0.9, 0.97], [0.1, 0.5, 0.9], 2);
    // L'arc de l'archer : tenu debout au bout du bras, la corde derrière.
    m("arcBow", Mat::Grain, 0.025, &|p| { let w = [0.4, 0.26, 0.14];
        p.tube([0.0, 0.0, 0.03], [0.0, 0.3, -0.03], [0.018, 0.018], [0.012, 0.012], w, dk(w, 0.8)); p.tube([0.0, 0.3, -0.03], [0.0, 0.5, -0.14], [0.012, 0.012], [0.006, 0.006], w, dk(w, 0.8));
        p.tube([0.0, 0.0, 0.03], [0.0, -0.3, -0.03], [0.018, 0.018], [0.012, 0.012], w, dk(w, 0.8)); p.tube([0.0, -0.3, -0.03], [0.0, -0.5, -0.14], [0.012, 0.012], [0.006, 0.006], w, dk(w, 0.8));
        p.bm([-0.003, -0.5, -0.145, 0.003, 0.5, -0.139], [0.85, 0.82, 0.74], PLAIN); });

    // ---- Ver des sables : des anneaux cuirassés couleur de dune, une gueule ronde garnie de crocs ; sous le sable, on n'en voit qu'un sillon.
    let (sand, sand2, belly) = ([0.72, 0.6, 0.4], [0.5, 0.4, 0.26], [0.86, 0.78, 0.6]);
    m("verSeg", Mat::Chitin, 0.045, &|p| { p.blob([0.0, 0.22, 0.0], [0.34, 0.26, 0.34], sand, sand2); p.bm([-0.3, 0.36, -0.3, 0.3, 0.42, 0.3], sand2, Mat::Chitin); p.bm([-0.18, 0.1, 0.26, 0.18, 0.34, 0.35], belly, PLAIN); });
    m("verHead", Mat::Chitin, 0.04, &|p| {
        p.column_m(&[(0.0, 0.34, 0.34), (0.3, 0.3, 0.3), (0.5, 0.2, 0.2)], 0.0, sand2, sand, 0.7, Mat::Chitin); p.bm([-0.14, 0.46, -0.14, 0.14, 0.52, 0.14], [0.3, 0.08, 0.08], PLAIN); // la gueule
        for k in 0..8 { let a = k as f64 * std::f64::consts::FRAC_PI_4; let (c, s) = (a.cos() * 0.17, a.sin() * 0.17); p.tube_m([c, 0.48, s], [c * 0.6, 0.66, s * 0.6], [0.035, 0.035], [0.008, 0.008], [0.95, 0.92, 0.82], [1.0, 1.0, 0.95], PLAIN); }
    });
    m("verMound", Mat::Grain, 0.05, &|p| { p.blob([0.0, 0.02, 0.0], [0.5, 0.16, 0.5], sand, sand2); p.blob([0.3, 0.0, -0.36], [0.26, 0.1, 0.26], sand, sand2); p.blob([-0.32, 0.0, -0.3], [0.22, 0.08, 0.22], sand, sand2); });

    // ---- Harpie : un corps d'oiseau de proie à tête de femme échevelée, des serres, deux grandes ailes sombres.
    let (fe, fe2, skin, hair) = ([0.3, 0.24, 0.3], [0.5, 0.42, 0.4], [0.78, 0.66, 0.58], [0.16, 0.1, 0.14]);
    m("hrpBody", Mat::Feather, 0.035, &|p| {
        p.body(&[(-0.5, 0.05, 0.1, 0.02), (-0.25, 0.18, 0.2, -0.08), (0.05, 0.22, 0.24, -0.1), (0.25, 0.16, 0.2, -0.02)], fe, fe, fe2);
        p.blob_m([0.0, 0.28, 0.3], [0.12, 0.13, 0.12], skin, dk(skin, 0.85), Mat::Human); p.blob_m([0.0, 0.34, 0.24], [0.14, 0.12, 0.14], hair, hair, Mat::Hair); // la tête et sa tignasse
        for sx in [-1.0f64, 1.0] { p.bm([sx * 0.05 - 0.02, 0.29, 0.405, sx * 0.05 + 0.02, 0.32, 0.425], [0.95, 0.8, 0.2], PLAIN);
            p.tube_m([sx * 0.1, -0.06, -0.05], [sx * 0.11, -0.3, 0.02], [0.04, 0.04], [0.025, 0.025], fe2, [0.75, 0.6, 0.2], PLAIN);
            for t in [-0.05, 0.0, 0.05] { p.tube_m([sx * 0.11, -0.3, 0.02], [sx * 0.11 + t, -0.4, 0.1], [0.014, 0.014], [0.005, 0.005], [0.75, 0.6, 0.2], [0.1, 0.08, 0.06], PLAIN); } }
        p.bm([-0.02, 0.25, 0.41, 0.02, 0.275, 0.45], [0.75, 0.6, 0.2], PLAIN); // un bec crochu
        for (x, w) in [(-0.1, 0.07), (0.0, 0.08), (0.1, 0.07)] { p.bx([x - w * 0.5, 0.03, -0.78, x + w * 0.5, 0.06, -0.46], dk(fe, 0.8)); } // la queue
    });
    m("hrpWing", Mat::Feather, 0.04, &|p| {
        p.tube([0.0, 0.0, 0.05], [0.45, 0.03, 0.1], [0.045, 0.045], [0.03, 0.03], fe, fe2);
        for (x0, x1, z0, z1) in [(0.0, 0.18, -0.3, 0.1), (0.18, 0.36, -0.4, 0.13), (0.36, 0.54, -0.42, 0.13), (0.54, 0.72, -0.36, 0.1), (0.72, 0.9, -0.26, 0.07), (0.9, 1.05, -0.14, 0.04)] {
            p.bx([x0, 0.0, z0, x1, 0.02, z1], fe); p.bx([x0 + 0.02, -0.006, z0 - 0.08, x1 - 0.02, 0.012, z0 + 0.02], dk(fe, 0.7)); }
    });
}
