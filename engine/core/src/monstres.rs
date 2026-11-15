//! Les créatures des profondeurs, sculptées en volumes galbés (la méthode des dauphins) : araignée des cavernes, golem de
//! cristal, rampant de magma, braise, scarabée lumineux. Chaque pièce est modelée autour de son point d'attache.
use crate::hostile::Mat;
use crate::mesher::VBuf;
use crate::sculpt::{dk, Pen, C};

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
    out
}
