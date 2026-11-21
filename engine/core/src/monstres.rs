//! Les créatures des profondeurs, sculptées en volumes galbés (la méthode des dauphins) : araignée des cavernes, golem de
//! cristal, rampant de magma, braise, scarabée lumineux. Chaque pièce est modelée autour de son point d'attache.
use crate::hostile::Mat;
use crate::mesher::VBuf;
use crate::sculpt::{dk, mix, Pen, C};
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

/// Les créatures de surface : sylvain (un être d'écorce), noyé, archer squelette, fantôme, ver des sables, harpie.
/// Les quatre premiers partagent une charpente d'homme : torse depuis les hanches, tête, bras et jambe d'une pièce.
fn surface(out: &mut Vec<(&'static str, VBuf)>) {
    let mut m = |n: &'static str, mat: Mat, pix: f64, f: &dyn Fn(&mut Pen)| { let mut v = VBuf::new(4000); { let mut p = Pen::new(&mut v, mat, pix); f(&mut p); } out.push((n, v)); };

    // ---- Sylvain (d'après la planche du joueur) : un colosse voûté. Un tronc tors qui s'évase en épaules énormes, une tête basse
    // et anguleuse aux yeux verts sous des bois de cerf, de longs bras aux mains griffues, des jambes courtes qui s'étalent en
    // racines-contreforts ; un manteau de mousse sur les épaules, la tête et les bras, d'où pendent des lianes.
    let (bark, bark2, bark3) = ([0.52, 0.37, 0.23], [0.34, 0.23, 0.14], [0.62, 0.46, 0.3]); let (moss, moss2, vine) = ([0.44, 0.62, 0.16], [0.26, 0.42, 0.12], [0.36, 0.56, 0.14]); let glow = [0.55, 1.0, 0.2];
    // une touffe de mousse posée sur le bois, et les lianes qui en pendent
    let tuft = |p: &mut Pen, c: [f64; 3], r: [f64; 3], vines: &[(f64, f64, f64)]| { p.blob_m(c, r, moss, moss2, Mat::Wool);
        for &(dx, dz, l) in vines { p.bm([c[0] + dx - 0.022, c[1] - l, c[2] + dz - 0.022, c[0] + dx + 0.022, c[1], c[2] + dz + 0.022], vine, Mat::Wool); p.bm([c[0] + dx - 0.035, c[1] - l - 0.05, c[2] + dz - 0.035, c[0] + dx + 0.035, c[1] - l + 0.02, c[2] + dz + 0.035], moss, Mat::Wool); } };
    m("sylTorso", Mat::Bark, 0.03, &|p| {
        // le tronc : étroit à la taille, il s'ouvre en un poitrail large et bossu
        p.column_m(&[(0.0, 0.3, 0.26), (0.2, 0.27, 0.24), (0.45, 0.34, 0.28), (0.7, 0.5, 0.34), (0.92, 0.6, 0.38), (1.08, 0.5, 0.34), (1.18, 0.3, 0.24)], 0.0, bark2, bark, 0.7, Mat::Bark);
        // des torons de bois qui s'enroulent autour du tronc
        for (x0, z0, x1, z1, y0, y1, w) in [(-0.26, 0.2, 0.3, 0.3, 0.05, 0.85, 0.075), (0.24, 0.2, -0.36, 0.3, 0.1, 0.95, 0.07), (-0.28, -0.18, 0.2, -0.3, 0.0, 0.9, 0.08), (0.3, -0.1, 0.5, 0.1, 0.3, 1.0, 0.07), (-0.3, 0.0, -0.52, 0.12, 0.35, 1.0, 0.07)] {
            p.tube([x0, y0, z0], [x1, y1, z1], [w, w * 0.8], [w * 0.8, w * 0.65], bark2, bark3); }
        p.blob([0.0, 1.02, -0.22], [0.42, 0.2, 0.2], bark, bark2); // la bosse du dos
        // le manteau de mousse des épaules et du dos, ses lianes
        tuft(p, [-0.42, 1.14, 0.02], [0.26, 0.11, 0.3], &[(-0.12, 0.26, 0.34), (0.06, 0.28, 0.5), (-0.2, -0.2, 0.4)]);
        tuft(p, [0.42, 1.14, 0.02], [0.26, 0.11, 0.3], &[(0.12, 0.26, 0.42), (-0.06, 0.28, 0.3), (0.2, -0.22, 0.5)]);
        tuft(p, [0.0, 1.16, -0.2], [0.34, 0.1, 0.22], &[(-0.16, -0.18, 0.55), (0.14, -0.2, 0.4)]);
        tuft(p, [0.0, 0.5, 0.27], [0.22, 0.09, 0.08], &[(-0.1, 0.05, 0.24), (0.1, 0.05, 0.32)]); // une jupe de mousse à la taille
        p.bm([-0.36, 0.52, 0.06, -0.24, 0.55, 0.3], [0.86, 0.74, 0.55], PLAIN); p.bm([-0.34, 0.4, 0.08, -0.2, 0.43, 0.34], [0.8, 0.68, 0.5], PLAIN); // des polypores
    });
    m("sylHead", Mat::Bark, 0.026, &|p| {
        // un coin de bois anguleux : front large, mâchoire étroite, arcades lourdes
        p.column_m(&[(0.0, 0.1, 0.12), (0.12, 0.17, 0.17), (0.26, 0.22, 0.2), (0.36, 0.2, 0.18)], 0.02, bark2, bark, 0.72, Mat::Bark);
        p.bm([-0.24, 0.24, 0.14, 0.24, 0.31, 0.25], bark2, Mat::Bark); p.tube([0.0, 0.26, 0.2], [0.0, 0.1, 0.27], [0.035, 0.03], [0.05, 0.035], bark, bark3); // l'arcade, le nez
        for sx in [-1.0f64, 1.0] { p.bm([sx * 0.11 - 0.06, 0.17, 0.2, sx * 0.11 + 0.06, 0.245, 0.225], [0.05, 0.04, 0.02], PLAIN); p.bm([sx * 0.11 - 0.04, 0.185, 0.22, sx * 0.11 + 0.04, 0.23, 0.24], glow, PLAIN); } // les yeux verts
        for (x, l) in [(-0.09, 0.09), (-0.045, 0.12), (0.0, 0.1), (0.045, 0.12), (0.09, 0.08)] { p.tube_m([x, 0.07, 0.14], [x, 0.07 - l, 0.16], [0.02, 0.02], [0.006, 0.006], bark3, bark, PLAIN); } // les crocs d'écharde
        // les bois : deux ramures qui montent, s'écartent et se divisent
        for sx in [-1.0f64, 1.0] {
            p.tube([sx * 0.14, 0.32, -0.02], [sx * 0.36, 0.62, -0.08], [0.06, 0.06], [0.045, 0.045], bark2, bark); p.tube([sx * 0.36, 0.62, -0.08], [sx * 0.62, 0.74, -0.04], [0.045, 0.045], [0.03, 0.03], bark, bark3);
            p.tube([sx * 0.62, 0.74, -0.04], [sx * 0.72, 1.0, 0.0], [0.03, 0.03], [0.014, 0.014], bark, bark3); p.tube([sx * 0.62, 0.74, -0.04], [sx * 0.9, 0.8, 0.04], [0.028, 0.028], [0.012, 0.012], bark, bark3);
            p.tube([sx * 0.36, 0.62, -0.08], [sx * 0.34, 0.98, -0.12], [0.04, 0.04], [0.016, 0.016], bark, bark3); p.tube([sx * 0.34, 0.86, -0.11], [sx * 0.5, 1.02, -0.06], [0.022, 0.022], [0.01, 0.01], bark, bark3);
            p.tube([sx * 0.2, 0.4, -0.04], [sx * 0.12, 0.72, -0.1], [0.035, 0.035], [0.012, 0.012], bark2, bark);
            tuft(p, [sx * 0.66, 0.8, -0.03], [0.12, 0.06, 0.1], &[(sx * 0.04, 0.04, 0.2)]); tuft(p, [sx * 0.36, 1.0, -0.12], [0.09, 0.05, 0.09], &[]); }
        tuft(p, [0.0, 0.39, -0.04], [0.22, 0.08, 0.2], &[(-0.17, 0.1, 0.22), (0.18, 0.08, 0.16), (0.0, -0.18, 0.3)]); // la mousse du crâne
    });
    m("sylArm", Mat::Bark, 0.028, &|p| {
        // l'épaule en boule, le bras qui s'épaissit vers une main énorme, quatre griffes recourbées
        p.blob([0.0, 0.0, 0.0], [0.19, 0.17, 0.19], bark, bark2); p.tube([0.0, -0.05, 0.0], [0.0, -0.62, 0.05], [0.12, 0.12], [0.11, 0.11], bark, bark2);
        p.tube([0.0, -0.1, -0.06], [0.03, -0.6, 0.08], [0.05, 0.05], [0.04, 0.04], bark2, bark3); // un toron
        p.blob([0.0, -0.64, 0.05], [0.13, 0.1, 0.13], bark, bark2); p.tube([0.0, -0.64, 0.05], [0.0, -1.12, 0.14], [0.11, 0.11], [0.15, 0.14], bark2, bark);
        p.blob([0.0, -1.2, 0.16], [0.19, 0.14, 0.17], bark, bark2); // la paume
        for (x, z, l) in [(-0.15, 0.1, 0.34), (-0.05, 0.16, 0.42), (0.06, 0.16, 0.42), (0.16, 0.1, 0.34)] { p.tube([x, -1.24, 0.16 + z * 0.3], [x * 1.1, -1.24 - l * 0.6, 0.2 + z], [0.055, 0.055], [0.04, 0.04], bark, bark3);
            p.tube([x * 1.1, -1.24 - l * 0.6, 0.2 + z], [x * 1.05, -1.24 - l, 0.12 + z], [0.04, 0.04], [0.01, 0.01], bark3, [0.75, 0.6, 0.42]); }
        p.tube([-0.14, -1.16, 0.12], [-0.3, -1.34, 0.22], [0.045, 0.045], [0.012, 0.012], bark, bark3); // le pouce
        tuft(p, [0.0, 0.14, 0.0], [0.2, 0.08, 0.2], &[(0.0, 0.16, 0.3), (-0.12, -0.1, 0.22)]); tuft(p, [0.0, -0.58, -0.08], [0.13, 0.06, 0.1], &[(0.0, -0.04, 0.26), (0.08, 0.0, 0.16)]);
    });
    m("sylLeg", Mat::Bark, 0.03, &|p| {
        // une jambe courte comme un tronc, qui s'étale au sol en contreforts : de grosses racines, puis des radicelles
        p.tube([0.0, 0.0, 0.0], [0.0, -0.5, 0.0], [0.17, 0.17], [0.2, 0.2], bark2, bark); p.blob([0.0, -0.22, 0.06], [0.16, 0.1, 0.15], bark, bark2);
        for (x, z, r) in [(-0.34, 0.26, 0.11), (0.3, 0.32, 0.11), (0.0, 0.46, 0.1), (0.02, -0.36, 0.11), (0.38, -0.08, 0.1), (-0.38, -0.1, 0.1)] {
            p.tube([x * 0.2, -0.38, z * 0.2], [x * 0.7, -0.66, z * 0.7], [r, r], [r * 0.75, r * 0.6], bark, bark2); p.tube([x * 0.7, -0.66, z * 0.7], [x * 1.25, -0.75, z * 1.25], [r * 0.7, r * 0.5], [0.03, 0.025], bark2, bark3); }
        tuft(p, [0.12, -0.42, 0.18], [0.12, 0.05, 0.1], &[]); p.bm([-0.2, -0.75, 0.3, -0.06, -0.7, 0.44], moss, Mat::Wool);
        // un champignon rouge à pois blancs entre les racines
        p.bm([0.22, -0.75, 0.1, 0.26, -0.66, 0.14], [0.92, 0.9, 0.84], PLAIN); p.bm([0.17, -0.66, 0.05, 0.31, -0.61, 0.19], [0.86, 0.14, 0.1], PLAIN); p.bm([0.2, -0.61, 0.08, 0.24, -0.6, 0.12], [0.96, 0.95, 0.92], PLAIN);
    });

    // ---- Trois créatures sur la charpente d'homme (torse depuis les hanches, tête, bras, jambe), d'après la planche du joueur.

    // Archer squelette : un vrai squelette — colonne, bassin, cage de côtes sur un creux sombre — sous un capuchon brun en lambeaux ;
    // le crâne aux orbites qui luisent d'un bleu pâle, un baudrier de cuir en travers, le carquois dans le dos, un pagne déchiré.
    let (bone, bone2, hollow) = ([0.9, 0.87, 0.78], [0.66, 0.62, 0.53], [0.07, 0.06, 0.06]); let (hood, hood2, strap) = ([0.24, 0.17, 0.13], [0.15, 0.11, 0.09], [0.42, 0.27, 0.15]); let pale = [0.78, 0.92, 1.0];
    m("arcTorso", Mat::Bone, 0.022, &|p| {
        p.bm([-0.15, 0.2, -0.07, 0.15, 0.54, 0.06], hollow, PLAIN); // le creux de la cage
        p.tube([0.0, 0.0, -0.03], [0.0, 0.58, -0.03], [0.032, 0.032], [0.028, 0.028], bone2, bone); // la colonne
        p.rbox([-0.15, -0.05, -0.07, 0.15, 0.07, 0.07], bone); p.bm([-0.06, -0.03, 0.05, 0.06, 0.04, 0.075], hollow, PLAIN); // le bassin
        for (k, y) in [0.24, 0.31, 0.38, 0.45].iter().enumerate() { let w = 0.15 + 0.012 * k as f64; p.bx([-w, *y, 0.055, w, y + 0.035, 0.095], bone); for sx in [-1.0f64, 1.0] { p.bx([sx * w - 0.02, *y, -0.08, sx * w + 0.02, y + 0.035, 0.09], bone2); } p.bx([-w, *y, -0.1, w, y + 0.035, -0.07], bone2); } // les côtes
        p.bx([-0.028, 0.22, 0.085, 0.028, 0.52, 0.11], bone); p.bx([-0.25, 0.52, -0.05, 0.25, 0.565, 0.05], bone); // sternum, clavicules
        for sx in [-1.0f64, 1.0] { p.blob([sx * 0.26, 0.545, 0.0], [0.05, 0.045, 0.05], bone, bone2); }
        p.tube_m([-0.21, 0.56, 0.1], [0.17, 0.1, 0.1], [0.035, 0.014], [0.035, 0.014], strap, dk(strap, 0.8), Mat::Leather); p.tube_m([-0.21, 0.56, -0.1], [0.17, 0.1, -0.1], [0.035, 0.014], [0.035, 0.014], strap, dk(strap, 0.8), Mat::Leather); // le baudrier
        p.bm([-0.17, 0.05, -0.09, 0.17, 0.11, 0.09], strap, Mat::Leather); p.bm([-0.03, 0.055, 0.085, 0.03, 0.105, 0.1], [0.75, 0.62, 0.3], Mat::Metal); // la ceinture et sa boucle
        for (x, l, z) in [(-0.16, 0.24, 0.085), (-0.09, 0.36, 0.09), (-0.02, 0.28, 0.09), (0.05, 0.4, 0.09), (0.12, 0.26, 0.085), (-0.14, 0.34, -0.09), (-0.05, 0.26, -0.095), (0.04, 0.38, -0.095), (0.11, 0.3, -0.09)] { p.bm([x, 0.05 - l, z - 0.012, x + 0.06, 0.06, z + 0.012], if (x * 50.0) as i32 % 2 == 0 { hood } else { hood2 }, Mat::Cloth); } // le pagne en lanières
        // le carquois, en biais dans le dos, et ses flèches empennées de blanc
        p.tube_m([0.12, 0.08, -0.15], [-0.06, 0.6, -0.17], [0.055, 0.055], [0.06, 0.06], strap, dk(strap, 0.75), Mat::Leather);
        for (dx, dz, l) in [(-0.03, 0.0, 0.2), (0.02, 0.02, 0.24), (0.0, -0.03, 0.18), (0.035, -0.02, 0.21)] { let (x, z) = (-0.06 + dx, -0.17 + dz); p.tube_m([x, 0.6, z], [x - l * 0.33, 0.6 + l, z - l * 0.04], [0.008, 0.008], [0.008, 0.008], [0.5, 0.36, 0.2], [0.5, 0.36, 0.2], PLAIN);
            p.bm([x - l * 0.33 - 0.02, 0.6 + l * 0.6, z - l * 0.04 - 0.004, x - l * 0.33 + 0.02, 0.6 + l, z - l * 0.04 + 0.004], [0.93, 0.91, 0.86], PLAIN); }
        // la pèlerine du capuchon sur les épaules, déchirée dans le dos
        p.bm([-0.27, 0.5, -0.13, 0.27, 0.61, 0.09], hood, Mat::Cloth); for (x, l) in [(-0.26, 0.12), (-0.16, 0.22), (-0.06, 0.14), (0.04, 0.24), (0.15, 0.13)] { p.bm([x, 0.5 - l, -0.135, x + 0.1, 0.52, -0.11], hood2, Mat::Cloth); }
    });
    m("arcHead", Mat::Bone, 0.02, &|p| {
        p.column_m(&[(0.0, 0.075, 0.085), (0.07, 0.115, 0.115), (0.17, 0.125, 0.125), (0.27, 0.105, 0.11)], 0.0, bone2, bone, 0.7, Mat::Bone); // le crâne
        p.bx([-0.075, -0.045, -0.01, 0.075, 0.01, 0.105], bone); p.bm([-0.07, 0.005, 0.1, 0.07, 0.022, 0.118], hollow, PLAIN); for x in [-0.06, -0.03, 0.0, 0.03] { p.bm([x, -0.012, 0.1, x + 0.022, 0.012, 0.117], bone, PLAIN); } // la mâchoire, les dents
        for sx in [-1.0f64, 1.0] { p.bm([sx * 0.055 - 0.036, 0.1, 0.105, sx * 0.055 + 0.036, 0.17, 0.128], hollow, PLAIN); p.bm([sx * 0.055 - 0.022, 0.115, 0.122, sx * 0.055 + 0.022, 0.155, 0.134], pale, PLAIN); p.bm([sx * 0.055 - 0.01, 0.127, 0.13, sx * 0.055 + 0.01, 0.145, 0.138], [1.0, 1.0, 1.0], PLAIN); }
        p.bm([-0.014, 0.055, 0.108, 0.014, 0.095, 0.126], hollow, PLAIN); // le nez
        // le capuchon : dos, dessus, joues, une visière qui met les orbites dans l'ombre
        p.bm([-0.155, -0.04, -0.16, 0.155, 0.31, -0.1], hood, Mat::Cloth); p.bm([-0.155, 0.27, -0.16, 0.155, 0.32, 0.13], hood, Mat::Cloth); p.bm([-0.13, 0.31, -0.14, 0.13, 0.335, 0.09], hood, Mat::Cloth); for sx in [-1.0f64, 1.0] { p.bm([sx * 0.14 - 0.02, -0.02, -0.12, sx * 0.14 + 0.02, 0.3, 0.1], hood2, Mat::Cloth); }
        p.bm([-0.155, 0.235, 0.1, 0.155, 0.3, 0.16], hood2, Mat::Cloth); p.bm([-0.1, 0.1, -0.2, 0.1, 0.3, -0.15], hood2, Mat::Cloth); // la visière, et la pointe du capuchon qui retombe derrière
    });
    m("arcArm", Mat::Bone, 0.02, &|p| {
        p.tube([0.0, 0.0, 0.0], [0.0, -0.28, 0.0], [0.028, 0.028], [0.024, 0.024], bone, bone2); p.blob([0.0, -0.3, 0.0], [0.04, 0.035, 0.04], bone, bone2); // l'humérus, le coude
        for dx in [-0.016, 0.016] { p.tube([dx, -0.31, 0.0], [dx, -0.56, 0.015], [0.014, 0.014], [0.012, 0.012], bone, bone2); } // radius et cubitus
        p.bm([-0.04, -0.52, -0.03, 0.04, -0.42, 0.045], strap, Mat::Leather); // un brassard
        p.blob([0.0, -0.6, 0.02], [0.035, 0.035, 0.03], bone, bone2); for x in [-0.03, -0.01, 0.012] { p.bx([x, -0.68, 0.02, x + 0.016, -0.62, 0.04], bone); }
    });
    m("arcLeg", Mat::Bone, 0.022, &|p| {
        p.blob([0.0, -0.01, 0.0], [0.05, 0.045, 0.05], bone, bone2); p.tube([0.0, 0.0, 0.0], [0.0, -0.4, 0.0], [0.034, 0.034], [0.03, 0.03], bone, bone2); p.blob([0.0, -0.41, 0.01], [0.05, 0.045, 0.05], bone, bone2);
        for dx in [-0.018, 0.018] { p.tube([dx, -0.42, 0.0], [dx, -0.79, 0.0], [0.017, 0.017], [0.014, 0.014], bone, bone2); }
        p.rbox([-0.045, -0.86, -0.05, 0.045, -0.79, 0.13], bone); for x in [-0.04, -0.012, 0.016] { p.bx([x, -0.86, 0.13, x + 0.024, -0.83, 0.16], bone2); }
    });

    // Fantôme (d'après la planche du joueur) : un drap de glace d'un seul tenant. Une tête cubique sans cou, deux yeux carrés et
    // une bouche sombre ; le drap tombe droit, s'élargit aux épaules en deux pans courts qui pendent sur les côtés, et finit en
    // longues franges déchiquetées, de plus en plus bleues, puis en éclats qui flottent dessous.
    let (ice, ice2, ice3, cyan) = ([0.74, 0.9, 1.0], [0.46, 0.74, 1.0], [0.2, 0.46, 0.92], [0.4, 1.0, 1.0]); let dark = [0.04, 0.1, 0.26];
    // des carreaux plus clairs ou plus sombres semés sur le drap, comme des facettes de glace
    let facets = |p: &mut Pen, spots: &[(f64, f64, f64, f64)], z: f64| { for (i, &(x, y, w, h)) in spots.iter().enumerate() { p.bm([x, y, z, x + w, y + h, z + 0.012], if i % 2 == 0 { [0.83, 0.95, 1.0] } else { mix(ice, ice2, 0.4) }, PLAIN); } };
    // une frange : trois tronçons de plus en plus étroits et de plus en plus bleus
    let fringe = |p: &mut Pen, x: f64, w: f64, top: f64, l: f64, z0: f64, z1: f64| {
        p.bm([x, top - l * 0.45, z0, x + w, top + 0.02, z1], ice2, PLAIN); p.bm([x + w * 0.15, top - l * 0.8, z0 + 0.01, x + w * 0.85, top - l * 0.4, z1 - 0.01], mix(ice2, ice3, 0.6), PLAIN); p.bm([x + w * 0.3, top - l, z0 + 0.02, x + w * 0.7, top - l * 0.75, z1 - 0.02], ice3, PLAIN); };
    m("fanTorso", PLAIN, 0.03, &|p| {
        p.rbox_m([-0.24, -0.3, -0.17, 0.24, 0.62, 0.17], ice, PLAIN); p.bm([-0.25, -0.3, -0.18, 0.25, 0.0, 0.18], mix(ice, ice2, 0.5), PLAIN); // le drap, plus bleu vers le bas
        p.bm([-0.3, 0.46, -0.15, 0.3, 0.62, 0.15], ice, PLAIN); // la carrure, d'où tombent les pans
        facets(p, &[(-0.2, 0.44, 0.08, 0.08), (0.08, 0.3, 0.1, 0.07), (-0.1, 0.14, 0.07, 0.09), (0.12, 0.02, 0.08, 0.08), (-0.22, -0.08, 0.09, 0.07), (0.0, -0.2, 0.07, 0.07), (0.14, 0.5, 0.07, 0.06), (-0.05, 0.34, 0.06, 0.06)], 0.17);
        // l'ourlet : des franges inégales, devant, derrière et sur les flancs
        for (x, w, l) in [(-0.25, 0.07, 0.3), (-0.18, 0.08, 0.62), (-0.1, 0.07, 0.4), (-0.03, 0.08, 0.78), (0.05, 0.07, 0.5), (0.12, 0.07, 0.68), (0.19, 0.06, 0.34)] { fringe(p, x, w, -0.3, l, 0.09, 0.18); fringe(p, x + 0.02, w, -0.3, l * 0.8, -0.18, -0.09); }
        for (z, l) in [(-0.08, 0.46), (0.0, 0.6), (0.06, 0.36)] { for x in [-0.25, 0.19] { fringe(p, x, 0.06, -0.3, l, z, z + 0.07); } }
        for (x, y, z, r) in [(-0.16, -1.0, 0.12, 0.032), (0.1, -1.1, -0.1, 0.028), (0.0, -1.22, 0.1, 0.024), (-0.07, -1.32, -0.06, 0.018), (0.17, -0.94, 0.06, 0.022), (-0.2, -1.16, -0.02, 0.016), (0.06, -1.4, 0.02, 0.014)] { p.bm([x - r, y - r, z - r, x + r, y + r, z + r], cyan, PLAIN); }
    });
    m("fanHead", PLAIN, 0.03, &|p| {
        p.rbox_m([-0.25, -0.02, -0.2, 0.25, 0.42, 0.2], ice, PLAIN); p.bm([-0.2, 0.4, -0.16, 0.2, 0.44, 0.16], [0.9, 0.97, 1.0], PLAIN);
        facets(p, &[(-0.2, 0.3, 0.07, 0.07), (0.13, 0.32, 0.07, 0.06), (-0.04, 0.33, 0.07, 0.05), (0.16, 0.06, 0.06, 0.07), (-0.22, 0.04, 0.07, 0.06)], 0.2);
        for sx in [-1.0f64, 1.0] { p.bm([sx * 0.105 - 0.055, 0.17, 0.195, sx * 0.105 + 0.055, 0.28, 0.215], cyan, PLAIN); p.bm([sx * 0.105 - 0.02, 0.21, 0.21, sx * 0.105 + 0.02, 0.245, 0.219], [0.75, 1.0, 1.0], PLAIN); p.bm([sx * 0.105 - 0.07, 0.155, 0.19, sx * 0.105 + 0.07, 0.295, 0.2], mix(ice, cyan, 0.45), PLAIN); } // deux yeux carrés et leur halo
        p.bm([-0.07, 0.01, 0.195, 0.07, 0.13, 0.213], dark, PLAIN); // la bouche
    });
    // Un pan du drap : une manche large et plate qui tombe de l'épaule, frangée au bout.
    m("fanArm", PLAIN, 0.03, &|p| {
        p.bm([-0.11, -0.34, -0.11, 0.11, 0.05, 0.11], ice, PLAIN); p.bm([-0.115, -0.34, -0.115, 0.115, -0.16, 0.115], mix(ice, ice2, 0.55), PLAIN);
        for (x, w, l) in [(-0.11, 0.07, 0.2), (-0.04, 0.07, 0.36), (0.03, 0.08, 0.26)] { for (z0, z1) in [(0.03, 0.11), (-0.11, -0.03)] { fringe(p, x, w, -0.34, l, z0, z1); } }
        p.bm([-0.02, -0.82, -0.02, 0.02, -0.78, 0.02], cyan, PLAIN);
    });
    m("fanLeg", PLAIN, 0.05, &|p| { p.bx([-0.01, -0.02, -0.01, 0.01, 0.0, 0.01], ice3); }); // (pas de jambes)

    // ---- La Reine des glaces : une figure élancée, aux proportions de statue — longue taille, épaules étroites, long cou, petite
    // tête ovale — qui glisse sans un pas. Ici, pas de tranches en escalier : la robe, la cape, les voiles, la chevelure et les
    // aiguilles sont faits de lames et de fuseaux orientés, qui suivent leur pente. Une robe à vingt-six plis qui s'évase jusqu'au
    // sol, du bleu profond de l'ourlet au blanc de givre du corsage, ourlée de stalactites ; une cape de nuit ; une collerette
    // d'aiguilles fines en éventail ; une tiare d'aiguilles ; de longues mèches d'argent ; des manches d'ange, voiles de givre ; un
    // sceptre fin couronné d'un astre de cristaux ; des éclats qui tournoient autour d'elle.
    let (gl1, gl2, gl3, frost, deep, night, glow, pale) = ([0.62, 0.8, 1.0], [0.36, 0.58, 0.95], [0.86, 0.95, 1.0], [0.97, 0.99, 1.0], [0.1, 0.2, 0.55], [0.07, 0.09, 0.26], [0.55, 1.0, 1.0], [0.88, 0.94, 1.0]);
    type V = [f64; 3];
    fn sub(a: V, b: V) -> V { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] } fn sc(a: V, k: f64) -> V { [a[0] * k, a[1] * k, a[2] * k] }
    fn cr(a: V, b: V) -> V { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] } fn nz(a: V) -> V { let l = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt().max(1e-9); sc(a, 1.0 / l) }
    fn lp(a: V, b: V, t: f64) -> V { [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t] }
    /// Un fuseau orienté de `a` à `b`, de section carrée, du rayon `ra` au rayon `rb`, en `n` tronçons.
    fn rod(p: &mut Pen, a: V, b: V, ra: f64, rb: f64, ca: C, cb: C, n: usize) {
        let d = sub(b, a); let up = if d[1].abs() > 0.9 * (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] }; let u = nz(cr(d, up)); let v = nz(cr(d, u));
        for k in 0..n { let (t0, t1) = (k as f64 / n as f64, (k + 1) as f64 / n as f64); let tm = (t0 + t1) * 0.5; let r = ra + (rb - ra) * tm;
            let (u2, v2) = (sc([u[0] + v[0], u[1] + v[1], u[2] + v[2]], std::f64::consts::FRAC_1_SQRT_2), sc([v[0] - u[0], v[1] - u[1], v[2] - u[2]], std::f64::consts::FRAC_1_SQRT_2)); let c = mix(ca, cb, tm);
            crate::mesher::obox(p.q, lp(a, b, tm), [sc(d, (t1 - t0) * 0.55), sc(u, r), sc(v, r)], c, -1.0, 255.0, 0.0);
            if r > 0.012 { crate::mesher::obox(p.q, lp(a, b, tm), [sc(d, (t1 - t0) * 0.55), sc(u2, r * 0.97), sc(v2, r * 0.97)], c, -1.0, 255.0, 0.0); } } // deux boîtes croisées : une section octogonale
    }
    /// Une coque à facettes, tournée autour d'un axe vertical : des anneaux (hauteur, rayon en x, rayon en z, décalage en z), `n` pans.
    fn shell(p: &mut Pen, c: V, prof: &[(f64, f64, f64, f64)], n: usize, ca: C, cb: C, th: f64) {
        let (y0, y1) = (prof[0].0, prof[prof.len() - 1].0); let ch = 2.0 * (std::f64::consts::PI / n as f64).sin() * 1.1;
        for w in prof.windows(2) { let ((ya, xa, za, oa), (yb, xb, zb, ob)) = (w[0], w[1]); for k in 0..n { let a = (k as f64 + 0.5) * std::f64::consts::TAU / n as f64; let (sn, cs) = (a.sin(), a.cos());
            let (pa, pb) = ([c[0] + sn * xa, c[1] + ya, c[2] + cs * za + oa], [c[0] + sn * xb, c[1] + yb, c[2] + cs * zb + ob]); let t = ((ya + yb) * 0.5 - y0) / (y1 - y0).abs().max(1e-6);
            if (pa[0] - pb[0]).abs() + (pa[1] - pb[1]).abs() + (pa[2] - pb[2]).abs() < 1e-6 { continue; }
            slab(p, pa, pb, ch * xa.max(za) + 0.004, ch * xb.max(zb) + 0.004, th, [sn, 0.0, cs], mix(ca, cb, t), mix(ca, cb, t), 1); } }
    }
    /// Un ovoïde à facettes (une coque d'anneaux tirés d'une ellipse).
    fn egg(p: &mut Pen, c: V, r: V, ca: C, cb: C, n: usize) {
        let rings: Vec<(f64, f64, f64, f64)> = (0..=6).map(|i| { let t = -std::f64::consts::FRAC_PI_2 + std::f64::consts::PI * i as f64 / 6.0; (r[1] * t.sin(), (r[0] * t.cos()).max(0.004), (r[2] * t.cos()).max(0.004), 0.0) }).collect();
        shell(p, c, &rings, n, ca, cb, (r[0].min(r[2]) * 0.35).clamp(0.01, 0.05));
    }
    /// Une lame d'étoffe ou de glace de `a` à `b`, de la largeur `wa` à `wb`, d'épaisseur `th`, tournée vers `out`, en `n` tronçons.
    fn slab(p: &mut Pen, a: V, b: V, wa: f64, wb: f64, th: f64, out: V, ca: C, cb: C, n: usize) {
        let d = sub(b, a); let w = nz(cr(d, out)); let nn = nz(cr(w, d));
        for k in 0..n { let (t0, t1) = (k as f64 / n as f64, (k + 1) as f64 / n as f64); let tm = (t0 + t1) * 0.5;
            crate::mesher::obox(p.q, lp(a, b, tm), [sc(d, (t1 - t0) * 0.56), sc(w, (wa + (wb - wa) * tm) * 0.5), sc(nn, th * 0.5)], mix(ca, cb, tm), -1.0, 255.0, 0.0); }
    }
    let spike = |p: &mut Pen, a: V, b: V, r: f64| rod(p, a, b, r, r * 0.12, gl1, frost, 4);
    m("rnTorso", Mat::Cloth, 0.014, &|p| {
        // la robe : un fond sombre, puis vingt-six plis en lames, de la taille jusqu'au sol, qui s'ouvrent en corolle
        p.column_m(&[(-0.88, 0.44, 0.4), (-0.5, 0.3, 0.27), (0.0, 0.1, 0.08)], 0.0, night, deep, 1.0, Mat::Cloth);
        for k in 0..26 { let a = k as f64 * std::f64::consts::TAU / 26.0; let (c, sn) = (a.cos(), a.sin()); let r = if k % 2 == 0 { 0.53 } else { 0.49 };
            slab(p, [c * 0.1, 0.02, sn * 0.085], [c * r, -0.92, sn * (r - 0.04)], 0.035, 0.15, 0.018, [c, 0.3, sn], gl2, if k % 2 == 0 { deep } else { mix(deep, night, 0.4) }, 7); }
        // l'ourlet : des stalactites fines tout autour
        for k in 0..26 { let a = k as f64 * std::f64::consts::TAU / 26.0 + 0.12; let (c, sn) = (a.cos(), a.sin()); let l = [0.09, 0.16, 0.06, 0.13][k % 4];
            rod(p, [c * 0.52, -0.9, sn * 0.48], [c * 0.55, -0.9 - l, sn * 0.51], 0.014, 0.002, gl1, frost, 3); }
        // le corsage : la taille fine, la poitrine, les épaules étroites ; un long cou
        p.column_m(&[(0.0, 0.08, 0.06), (0.3, 0.12, 0.08), (0.54, 0.13, 0.06)], 0.0, gl2, gl1, 1.0, PLAIN);
        for k in 0..16 { let a = k as f64 * std::f64::consts::TAU / 16.0; let (c, sn) = (a.cos(), a.sin()); let (w0, b, s0) = ([c * 0.095, 0.0, sn * 0.075], [c * 0.142, 0.3, sn * 0.1], [c * 0.155, 0.55, sn * 0.072]);
            slab(p, w0, b, 0.04, 0.058, 0.018, [c, 0.0, sn], gl1, mix(gl1, gl3, 0.6), 4); slab(p, b, s0, 0.058, 0.05, 0.018, [c, 0.2, sn], mix(gl1, gl3, 0.6), gl3, 4); }
        rod(p, [-0.17, 0.55, 0.0], [0.17, 0.55, 0.0], 0.045, 0.045, gl3, gl3, 6);
        rod(p, [0.0, 0.55, 0.0], [0.0, 0.71, 0.012], 0.034, 0.03, pale, pale, 3);
        // le décolleté de givre, une ceinture fine, le joyau au cœur
        for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.15, 0.54, 0.05], [sx * 0.015, 0.34, 0.1], 0.01, 0.008, frost, frost, 5); }
        rod(p, [-0.1, 0.03, 0.075], [0.1, 0.03, 0.075], 0.011, 0.011, frost, frost, 4); p.blob_m([0.0, 0.3, 0.1], [0.028, 0.035, 0.015], glow, frost, PLAIN);
        // la cape de nuit : deux grands pans qui tombent des épaules, s'élargissent et traînent au sol
        for sx in [-1.0f64, 1.0] { slab(p, [sx * 0.1, 0.55, -0.08], [sx * 0.34, -0.92, -0.5], 0.12, 0.5, 0.016, [sx * 0.25, 0.15, -1.0], night, deep, 10); }
        slab(p, [0.0, -0.6, -0.36], [0.0, -0.95, -0.95], 0.5, 0.55, 0.016, [0.0, 1.0, -0.2], deep, night, 5); // la traîne
        // la collerette : de fines aiguilles de glace en éventail, deux rangs, derrière la tête
        for k in 0..17 { let a = (k as f64 - 8.0) * 0.19; let l = 0.55 + 0.6 * (1.0 - (k as f64 - 8.0).abs() / 8.0); let (sn, c) = (a.sin(), a.cos());
            spike(p, [sn * 0.06, 0.6, -0.08], [sn * l, 0.6 + c * l, -0.18 - 0.08 * c], 0.02);
            if k % 2 == 1 { let l = l * 0.55; spike(p, [sn * 0.05, 0.6, -0.06], [sn * l * 1.15, 0.6 + c * l, -0.12], 0.015); } }
        // de petites aiguilles aux épaules
        for sx in [-1.0f64, 1.0] { for (dz, h) in [(0.0, 0.16), (-0.04, 0.11), (0.04, 0.09)] { spike(p, [sx * 0.16, 0.56, dz], [sx * 0.24, 0.56 + h, dz * 1.4], 0.016); } }
    });
    m("rnHead", Mat::Human, 0.012, &|p| {
        // une petite tête ovale : le crâne, la mâchoire fine, un nez droit
        p.blob_m([0.0, 0.14, 0.0], [0.07, 0.093, 0.08], pale, mix(pale, gl1, 0.2), Mat::Human); p.blob_m([0.0, 0.078, 0.02], [0.05, 0.05, 0.058], mix(pale, gl1, 0.15), pale, Mat::Human);
        rod(p, [0.0, 0.14, 0.074], [0.0, 0.098, 0.088], 0.007, 0.01, pale, pale, 3);
        // des yeux en amande, qui luisent, sous des sourcils de givre ; des lèvres bleuies
        for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.014, 0.128, 0.076], [sx * 0.048, 0.136, 0.067], 0.006, 0.004, glow, glow, 3); rod(p, [sx * 0.012, 0.148, 0.077], [sx * 0.05, 0.156, 0.066], 0.004, 0.002, frost, frost, 3); }
        rod(p, [-0.017, 0.07, 0.074], [0.017, 0.07, 0.074], 0.005, 0.005, [0.32, 0.46, 0.82], [0.32, 0.46, 0.82], 3);
        // la chevelure d'argent : une calotte lisse, puis de longues mèches qui tombent dans le dos
        p.blob_m([0.0, 0.178, -0.02], [0.078, 0.074, 0.084], frost, gl3, Mat::Hair);
        for k in 0..11 { let x = (k as f64 - 5.0) * 0.015; let l = 0.62 + 0.12 * ((k * 3 % 5) as f64 / 4.0); rod(p, [x * 0.8, 0.2, -0.06], [x * 2.2, 0.2 - l, -0.14 - 0.03 * (k % 2) as f64], 0.014, 0.004, frost, mix(frost, gl1, 0.45), 6); }
        for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.068, 0.18, 0.02], [sx * 0.085, -0.02, -0.01], 0.011, 0.004, frost, gl3, 5); }
        // la tiare : un fin bandeau d'argent, des aiguilles de glace, la plus haute au milieu, un cristal au front
        rod(p, [-0.075, 0.2, 0.045], [0.075, 0.2, 0.045], 0.006, 0.006, [0.85, 0.9, 0.98], [0.85, 0.9, 0.98], 4);
        for (x, z, h) in [(0.0, 0.05, 0.3), (-0.03, 0.045, 0.2), (0.03, 0.045, 0.2), (-0.055, 0.03, 0.14), (0.055, 0.03, 0.14), (-0.07, 0.0, 0.09), (0.07, 0.0, 0.09)] { spike(p, [x, 0.2, z], [x * 1.5, 0.2 + h, z * 0.8], 0.011); }
        p.blob_m([0.0, 0.215, 0.07], [0.012, 0.016, 0.008], glow, frost, PLAIN);
    });
    // Un bras, de l'épaule vers le bas : un bras mince, un poignet fin, une main aux longs doigts ; et la manche d'ange, un voile
    // de givre qui s'ouvre au coude et pend jusqu'aux genoux.
    m("rnArm", Mat::Human, 0.012, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.3, 0.0], 0.032, 0.027, gl1, gl3, 4); rod(p, [0.0, -0.3, 0.0], [0.0, -0.56, 0.02], 0.024, 0.019, pale, pale, 4);
        p.blob_m([0.0, -0.6, 0.025], [0.022, 0.035, 0.016], pale, pale, Mat::Human); for x in [-0.014, -0.004, 0.006, 0.016] { rod(p, [x, -0.62, 0.03], [x * 1.3, -0.7, 0.04], 0.005, 0.003, pale, gl1, 2); }
        rod(p, [0.0, -0.53, 0.0], [0.0, -0.57, 0.0], 0.028, 0.028, frost, frost, 1); // un bracelet de givre
        slab(p, [0.0, -0.22, -0.02], [0.0, -0.98, -0.16], 0.1, 0.24, 0.012, [0.0, 0.0, -1.0], mix(gl3, gl1, 0.3), gl2, 8);
        for k in 0..5 { let x = -0.11 + k as f64 * 0.055; rod(p, [x, -0.96, -0.16], [x, -1.04, -0.17], 0.008, 0.002, gl1, frost, 2); }
    });
    m("rnLeg", PLAIN, 0.05, &|p| { p.bx([-0.01, -0.02, -0.01, 0.01, 0.0, 0.01], deep); }); // (sous la robe)
    // Le sceptre : une hampe fine de glace, cerclée d'argent, couronnée d'un astre de cristaux qui luit en son cœur.
    m("rnStaff", PLAIN, 0.016, &|p| {
        rod(p, [0.0, -0.95, 0.0], [0.0, 0.9, 0.0], 0.013, 0.016, gl2, gl1, 6); for y in [-0.5, 0.2, 0.86] { rod(p, [0.0, y, 0.0], [0.0, y + 0.03, 0.0], 0.022, 0.022, [0.85, 0.88, 0.95], [0.85, 0.88, 0.95], 1); }
        rod(p, [0.0, -0.95, 0.0], [0.0, -1.08, 0.0], 0.014, 0.002, gl1, frost, 2);
        for sx in [-1.0f64, 1.0] { rod(p, [0.0, 0.86, 0.0], [sx * 0.1, 1.0, 0.0], 0.008, 0.008, [0.85, 0.88, 0.95], frost, 3); rod(p, [sx * 0.1, 1.0, 0.0], [sx * 0.05, 1.16, 0.0], 0.008, 0.005, frost, frost, 3); }
        p.blob_m([0.0, 1.05, 0.0], [0.045, 0.06, 0.045], glow, frost, PLAIN);
        for (x, y, z) in [(0.0, 1.0, 0.0), (0.7, 0.4, 0.0), (-0.7, 0.4, 0.0), (0.0, 0.4, 0.7), (0.0, 0.4, -0.7), (0.5, -0.3, 0.5), (-0.5, -0.3, -0.5), (0.5, -0.3, -0.5), (-0.5, -0.3, 0.5)] { spike(p, [0.0, 1.05, 0.0], [x * 0.2, 1.05 + y * 0.22, z * 0.2], 0.02); }
    });
    // Un éclat de glace qui tournoie autour d'elle : un fuseau effilé aux deux bouts.
    m("rnShard", PLAIN, 0.012, &|p| { rod(p, [0.0, -0.14, 0.0], [0.0, 0.0, 0.0], 0.002, 0.032, frost, gl1, 3); rod(p, [0.0, 0.0, 0.0], [0.0, 0.2, 0.0], 0.032, 0.002, gl1, frost, 3); });

    // ---- Les boss, sculptés comme la Reine : des volumes galbés, des lames et des fuseaux orientés, plus de briques.
    // Chacun : un torse (avec le bassin, et ce qui pend des épaules), une tête, un bras (le même des deux côtés), une jambe (de
    // la hanche au sol), une arme tenue dans la main droite, le long de +y.
    fn arc(p: &mut Pen, c: V, r: f64, a0: f64, a1: f64, y: f64, rr: f64, ca: C, cb: C, n: usize, zs: f64) { // un arc de cercle horizontal fait de fuseaux
        for k in 0..n { let (t0, t1) = (a0 + (a1 - a0) * k as f64 / n as f64, a0 + (a1 - a0) * (k + 1) as f64 / n as f64);
            rod(p, [c[0] + t0.sin() * r, c[1] + y, c[2] + t0.cos() * r * zs], [c[0] + t1.sin() * r, c[1] + y, c[2] + t1.cos() * r * zs], rr, rr, mix(ca, cb, k as f64 / n as f64), mix(ca, cb, (k + 1) as f64 / n as f64), 1); }
    }

    // ---- Le Colosse englouti : une statue de géant, rongée par la mer, qui s'est éveillée. Des épaules de rocher, une petite tête
    // enfoncée entre elles, une barbe de pierre sculptée ; des fissures où luit une lumière turquoise ; des coraux, des bernacles,
    // des algues qui pendent ; il brandit un fût de colonne brisé.
    let (st, st2, st3, moss, alga, cglow, coral, barn) = ([0.55, 0.57, 0.52], [0.36, 0.4, 0.38], [0.66, 0.68, 0.62], [0.26, 0.42, 0.22], [0.16, 0.42, 0.3], [0.3, 1.0, 0.85], [0.95, 0.45, 0.4], [0.86, 0.83, 0.72]);
    let crack = |p: &mut Pen, pts: &[V]| { for w in pts.windows(2) { rod(p, w[0], w[1], 0.012, 0.012, cglow, cglow, 2); } };
    let coral_at = |p: &mut Pen, c: V, k: f64| { rod(p, c, [c[0], c[1] + 0.14 * k, c[2]], 0.02 * k, 0.012 * k, coral, mix(coral, [1.0, 0.75, 0.7], 0.5), 2);
        for (dx, dz) in [(0.07, 0.02), (-0.06, 0.03), (0.02, -0.06)] { rod(p, [c[0], c[1] + 0.07 * k, c[2]], [c[0] + dx * k, c[1] + 0.15 * k, c[2] + dz * k], 0.012 * k, 0.007 * k, coral, [1.0, 0.75, 0.7], 2); } };
    let barnacles = |p: &mut Pen, c: V, r: f64, n: usize| { for i in 0..n { let a = i as f64 * 2.4; let q = [c[0] + a.cos() * r * (0.4 + 0.15 * (i % 3) as f64), c[1] + (i as f64 * 0.37).sin() * r * 0.4, c[2] + a.sin() * r * 0.5]; p.blob_m(q, [0.025, 0.02, 0.025], barn, mix(barn, st2, 0.4), PLAIN); } };
    m("colTorso", Mat::Barnacle, 0.024, &|p| {
        egg(p, [0.0, 0.04, 0.0], [0.27, 0.15, 0.19], st2, st, 12); shell(p, [0.0, 0.0, 0.0], &[(0.0, 0.2, 0.15, 0.0), (0.15, 0.23, 0.17, 0.0), (0.3, 0.27, 0.19, 0.0)], 12, st2, st, 0.05);
        egg(p, [0.0, 0.47, 0.01], [0.42, 0.27, 0.26], st, st3, 14);
        for sx in [-1.0f64, 1.0] { slab(p, [sx * 0.15, 0.58, 0.2], [sx * 0.17, 0.36, 0.25], 0.26, 0.24, 0.05, [sx * 0.2, 0.0, 1.0], st3, st, 3); // les pectoraux sculptés
            egg(p, [sx * 0.44, 0.6, 0.0], [0.19, 0.16, 0.18], st, st2, 10); } // les épaules de rocher
        for y in [0.08, 0.17, 0.26] { rod(p, [-0.13, y, 0.2], [0.13, y, 0.2], 0.035, 0.035, st3, st, 4); } // les abdominaux taillés
        crack(p, &[[0.05, 0.66, 0.25], [0.09, 0.55, 0.27], [0.04, 0.44, 0.27], [0.11, 0.33, 0.24], [0.07, 0.2, 0.2]]); crack(p, &[[-0.25, 0.6, 0.18], [-0.2, 0.5, 0.23], [-0.27, 0.42, 0.2]]); crack(p, &[[0.5, 0.72, 0.08], [0.42, 0.66, 0.15], [0.47, 0.58, 0.16]]);
        coral_at(p, [-0.48, 0.74, 0.02], 1.2); coral_at(p, [-0.36, 0.72, -0.08], 0.8); barnacles(p, [0.44, 0.66, 0.0], 0.2, 9); barnacles(p, [0.0, 0.5, -0.24], 0.3, 10);
        // les algues : sur les épaules et le dos, en longues lanières qui pendent ; un pagne de pierre moussue
        for k in 0..9 { let x = -0.4 + k as f64 * 0.1; let l = 0.55 + 0.35 * ((k * 5 % 7) as f64 / 6.0); slab(p, [x, 0.7, -0.18], [x * 1.1, 0.7 - l, -0.3], 0.07, 0.04, 0.012, [0.0, 0.0, -1.0], moss, alga, 5); }
        for sx in [-1.0f64, 1.0] { for k in 0..3 { let x = sx * (0.32 + k as f64 * 0.06); slab(p, [x, 0.72, 0.1], [x * 1.05, 0.32 - k as f64 * 0.06, 0.18], 0.05, 0.03, 0.01, [sx, 0.0, 0.4], moss, alga, 4); } }
        for (x, z, nx, nz) in [(0.0, 0.18, 0.0, 1.0), (0.0, -0.17, 0.0, -1.0), (-0.2, 0.0, -1.0, 0.0), (0.2, 0.0, 1.0, 0.0)] { slab(p, [x, 0.0, z], [x * 1.1, -0.36, z * 1.15], 0.24, 0.22, 0.04, [nx, 0.0, nz], st, moss, 3); }
    });
    m("colHead", Mat::Barnacle, 0.018, &|p| {
        egg(p, [0.0, 0.12, 0.03], [0.13, 0.14, 0.13], st3, st, 10); rod(p, [-0.12, 0.17, 0.13], [0.12, 0.17, 0.13], 0.03, 0.03, st, st, 4); // l'arcade lourde
        for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.03, 0.14, 0.15], [sx * 0.08, 0.145, 0.14], 0.012, 0.01, cglow, cglow, 2); } rod(p, [0.0, 0.15, 0.15], [0.0, 0.09, 0.17], 0.02, 0.03, st, st3, 2);
        // une barbe de pierre en boucles sculptées
        for k in 0..7 { let x = (k as f64 - 3.0) * 0.035; rod(p, [x, 0.06, 0.14], [x * 1.1, -0.16 + 0.03 * (x.abs() * 20.0).sin(), 0.16], 0.026, 0.016, st, st2, 4); }
        crack(p, &[[0.08, 0.26, 0.1], [0.04, 0.2, 0.15], [0.07, 0.12, 0.16]]); coral_at(p, [-0.08, 0.24, -0.02], 0.7); slab(p, [0.0, 0.26, -0.05], [0.0, -0.05, -0.15], 0.2, 0.14, 0.012, [0.0, 0.2, -1.0], moss, alga, 4);
    });
    m("colArm", Mat::Barnacle, 0.022, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.32, 0.0], 0.11, 0.09, st, st2, 4); egg(p, [0.0, -0.33, 0.0], [0.1, 0.09, 0.1], st2, st, 8);
        rod(p, [0.0, -0.34, 0.0], [0.0, -0.58, 0.03], 0.1, 0.12, st, st3, 4); rod(p, [0.0, -0.45, 0.0], [0.0, -0.49, 0.0], 0.13, 0.13, st2, st2, 1);
        egg(p, [0.0, -0.66, 0.04], [0.13, 0.12, 0.13], st3, st, 10); crack(p, &[[0.08, -0.05, 0.06], [0.1, -0.18, 0.04], [0.07, -0.3, 0.07]]); barnacles(p, [0.0, -0.5, 0.0], 0.12, 5);
        slab(p, [0.06, -0.02, -0.06], [0.08, -0.5, -0.1], 0.06, 0.04, 0.01, [1.0, 0.0, -0.5], moss, alga, 3);
    });
    m("colLeg", Mat::Barnacle, 0.024, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.44, 0.02], 0.14, 0.12, st, st2, 4); egg(p, [0.0, -0.46, 0.04], [0.12, 0.1, 0.12], st2, st, 8); rod(p, [0.0, -0.47, 0.03], [0.0, -0.84, 0.0], 0.12, 0.13, st, st2, 4);
        crate::mesher::obox(p.q, [0.0, -0.88, 0.08], [[0.15, 0.0, 0.0], [0.0, 0.05, 0.0], [0.0, 0.0, 0.2]], st2, -1.0, 255.0, 0.0); barnacles(p, [0.0, -0.7, 0.05], 0.12, 4);
        slab(p, [-0.08, -0.05, 0.1], [-0.1, -0.5, 0.13], 0.05, 0.03, 0.01, [0.0, 0.0, 1.0], moss, alga, 3);
    });
    m("colClub", Mat::Barnacle, 0.022, &|p| { // un fût de colonne brisé, cannelé, son chapiteau encore là
        rod(p, [0.0, -0.25, 0.0], [0.0, 0.1, 0.0], 0.06, 0.07, st2, st, 2); rod(p, [0.0, 0.1, 0.0], [0.0, 1.0, 0.0], 0.13, 0.12, st, st3, 6);
        for k in 0..8 { let a = k as f64 * std::f64::consts::TAU / 8.0; rod(p, [a.cos() * 0.13, 0.14, a.sin() * 0.13], [a.cos() * 0.12, 0.96, a.sin() * 0.12], 0.012, 0.012, st2, st2, 4); }
        crate::mesher::obox(p.q, [0.0, 1.05, 0.0], [[0.2, 0.0, 0.0], [0.0, 0.06, 0.0], [0.0, 0.0, 0.2]], st3, -1.0, 255.0, 0.0); crate::mesher::obox(p.q, [0.0, 1.13, 0.0], [[0.24, 0.0, 0.0], [0.0, 0.03, 0.0], [0.0, 0.0, 0.24]], st, -1.0, 255.0, 0.0);
        for (x, z, h) in [(0.12, 0.1, 0.07), (-0.1, 0.05, 0.05), (0.02, -0.13, 0.09)] { crate::mesher::obox(p.q, [x, 1.17 + h * 0.5, z], [[0.06, 0.0, 0.0], [0.0, h * 0.5, 0.0], [0.0, 0.0, 0.06]], st2, -1.0, 255.0, 0.0); }
        barnacles(p, [0.0, 0.6, 0.0], 0.14, 6); coral_at(p, [0.1, 0.45, 0.05], 0.6); slab(p, [0.1, 0.9, 0.0], [0.14, 0.4, 0.04], 0.05, 0.03, 0.01, [1.0, 0.0, 0.0], moss, alga, 3);
    });

    // ---- Le Roi noyé : un roi mort sous les eaux. Un corps décharné, les côtes saillantes, la peau verdâtre ; un grand manteau
    // royal en lambeaux, un col de varech, une chaîne d'or rongée ; un visage creusé aux yeux verts qui luisent, une longue barbe
    // d'algues, une couronne rouillée sertie de perles ; un grand trident de corail.
    let (sk, sk2, robe, robe2, purp, rgold, pearl, eye, kelp) = ([0.45, 0.6, 0.54], [0.26, 0.38, 0.35], [0.1, 0.3, 0.32], [0.04, 0.13, 0.16], [0.33, 0.18, 0.38], [0.74, 0.58, 0.26], [0.95, 0.94, 0.88], [0.5, 1.0, 0.6], [0.24, 0.4, 0.16]);
    m("rnyTorso", Mat::Flesh, 0.018, &|p| {
        p.column_m(&[(0.0, 0.12, 0.09), (0.2, 0.13, 0.09), (0.42, 0.18, 0.11), (0.56, 0.2, 0.09)], 0.0, sk2, sk, 0.9, Mat::Flesh);
        for k in 0..5 { let y = 0.24 + k as f64 * 0.06; for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.02, y + 0.01, 0.115], [sx * 0.15, y - 0.03, 0.07], 0.012, 0.01, sk, sk2, 3); } } // les côtes
        // le manteau royal : deux pans du haut des épaules jusqu'au sol, déchirés en bas ; le dos ; un col de varech
        for k in 0..9 { let a = -2.0 + k as f64 * 0.5; let (sn, c) = (a.sin(), a.cos()); let l = 1.38 - 0.18 * ((k * 3 % 4) as f64 / 3.0);
            slab(p, [sn * 0.2, 0.56, c * -0.1 - 0.02], [sn * 0.42, 0.56 - l, -c * 0.35 - 0.06], 0.13, 0.26, 0.016, [sn, 0.1, -c], purp, robe2, 9); }
        for sx in [-1.0f64, 1.0] { slab(p, [sx * 0.17, 0.56, 0.06], [sx * 0.3, -0.84, 0.18], 0.1, 0.16, 0.014, [sx * 0.6, 0.0, 1.0], robe, robe2, 9); }
        slab(p, [0.0, 0.02, 0.1], [0.0, -0.82, 0.2], 0.2, 0.3, 0.014, [0.0, 0.0, 1.0], robe, mix(robe2, purp, 0.3), 8); // le devant de la robe
        for k in 0..16 { let a = -2.3 + k as f64 * 0.3; rod(p, [a.sin() * 0.19, 0.57, a.cos() * 0.11], [a.sin() * 0.27, 0.46 - 0.04 * (k % 3) as f64, a.cos() * 0.16], 0.03, 0.012, kelp, mix(kelp, robe2, 0.4), 3); }
        // la chaîne d'or et son médaillon, une ceinture à boucle
        arc(p, [0.0, 0.0, 0.0], 0.15, -1.3, 1.3, 0.44, 0.012, rgold, rgold, 8, 0.85); p.blob_m([0.0, 0.3, 0.13], [0.04, 0.05, 0.015], rgold, [0.5, 0.36, 0.14], Mat::Metal); p.blob_m([0.0, 0.3, 0.145], [0.015, 0.015, 0.008], pearl, pearl, PLAIN);
        rod(p, [-0.13, 0.03, 0.1], [0.13, 0.03, 0.1], 0.02, 0.02, [0.3, 0.2, 0.12], [0.3, 0.2, 0.12], 4); for x in [-0.2, 0.15, 0.25] { p.blob_m([x, 0.5, -0.12], [0.03, 0.025, 0.03], barn, sk2, PLAIN); }
    });
    m("rnyHead", Mat::Flesh, 0.012, &|p| {
        egg(p, [0.0, 0.15, 0.0], [0.08, 0.11, 0.09], sk2, sk, 10); egg(p, [0.0, 0.065, 0.03], [0.055, 0.05, 0.06], sk2, sk, 8);
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.032, 0.15, 0.07], [0.022, 0.016, 0.012], sk2, [0.08, 0.12, 0.1], PLAIN); rod(p, [sx * 0.022, 0.152, 0.083], [sx * 0.042, 0.152, 0.08], 0.006, 0.006, eye, eye, 1); } // orbites creuses, yeux verts
        rod(p, [-0.02, 0.055, 0.085], [0.02, 0.055, 0.085], 0.008, 0.008, [0.05, 0.08, 0.07], [0.05, 0.08, 0.07], 2); // la bouche béante
        for k in 0..9 { let x = (k as f64 - 4.0) * 0.016; let l = 0.38 + 0.14 * ((k * 2 % 5) as f64 / 4.0); rod(p, [x, 0.06, 0.07], [x * 1.6, 0.06 - l, 0.1 + 0.02 * (k % 2) as f64], 0.012, 0.004, kelp, mix(kelp, sk2, 0.5), 5); } // la barbe d'algues
        for k in 0..10 { let x = (k as f64 - 4.5) * 0.02; rod(p, [x, 0.22, -0.05], [x * 2.2, -0.25, -0.14], 0.012, 0.004, [0.2, 0.28, 0.22], kelp, 5); } // les cheveux ruisselants
        // la couronne rouillée, penchée, sertie de perles
        for k in 0..12 { let a = k as f64 * std::f64::consts::TAU / 12.0; let (c, sn) = (a.cos(), a.sin()); let tip = if k % 2 == 0 { 0.13 } else { 0.07 };
            rod(p, [sn * 0.085, 0.235 + c * 0.012, c * 0.09], [sn * 0.095, 0.235 + tip + c * 0.012, c * 0.1], 0.016, 0.005, rgold, [0.5, 0.36, 0.14], 3); if k % 2 == 0 { p.blob_m([sn * 0.097, 0.235 + tip + c * 0.012, c * 0.102], [0.012, 0.012, 0.012], pearl, pearl, PLAIN); } }
        arc(p, [0.0, 0.0, 0.0], 0.09, 0.0, std::f64::consts::TAU, 0.235, 0.014, rgold, rgold, 12, 1.0);
    });
    m("rnyArm", Mat::Flesh, 0.012, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.3, 0.0], 0.034, 0.026, sk, sk2, 4); p.blob_m([0.0, -0.31, 0.0], [0.03, 0.03, 0.03], sk2, sk, Mat::Flesh); rod(p, [0.0, -0.31, 0.0], [0.0, -0.57, 0.02], 0.026, 0.02, sk, sk2, 4);
        p.blob_m([0.0, -0.6, 0.02], [0.03, 0.035, 0.022], sk, sk2, Mat::Flesh); for x in [-0.018, -0.006, 0.006, 0.018] { rod(p, [x, -0.62, 0.025], [x * 1.4, -0.72, 0.045], 0.006, 0.004, sk, sk2, 3); }
        rod(p, [0.006, -0.645, 0.03], [0.006, -0.655, 0.03], 0.009, 0.009, rgold, rgold, 1); slab(p, [0.0, -0.04, -0.03], [0.0, -0.5, -0.12], 0.12, 0.17, 0.012, [0.0, 0.0, -1.0], robe, robe2, 6); // une manche en lambeaux
    });
    m("rnyLeg", Mat::Flesh, 0.014, &|p| { rod(p, [0.0, 0.0, 0.0], [0.0, -0.44, 0.01], 0.05, 0.035, sk, sk2, 4); p.blob_m([0.0, -0.45, 0.02], [0.035, 0.035, 0.035], sk2, sk, Mat::Flesh); rod(p, [0.0, -0.46, 0.02], [0.0, -0.84, 0.0], 0.034, 0.028, sk, sk2, 4);
        crate::mesher::obox(p.q, [0.0, -0.865, 0.05], [[0.04, 0.0, 0.0], [0.0, 0.02, 0.0], [0.0, 0.0, 0.1]], sk2, -1.0, 255.0, 0.0); });
    m("rnyTrident", Mat::Metal, 0.016, &|p| {
        rod(p, [0.0, -1.0, 0.0], [0.0, 1.0, 0.0], 0.018, 0.02, [0.5, 0.38, 0.18], rgold, 8); for y in [-0.4, 0.3, 0.95] { rod(p, [0.0, y, 0.0], [0.0, y + 0.05, 0.0], 0.03, 0.03, rgold, rgold, 1); }
        rod(p, [-0.16, 1.03, 0.0], [0.16, 1.03, 0.0], 0.022, 0.022, rgold, rgold, 4);
        for x in [-0.15, 0.0, 0.15] { let h = if x == 0.0 { 0.42 } else { 0.32 }; rod(p, [x, 1.03, 0.0], [x * 1.05, 1.03 + h, 0.0], 0.018, 0.004, rgold, [0.9, 0.8, 0.5], 4); rod(p, [x, 1.03 + h * 0.75, 0.0], [x + if x == 0.0 { 0.04 } else { -x.signum() * 0.04 }, 1.03 + h * 0.62, 0.0], 0.01, 0.003, rgold, rgold, 2); }
        coral_at(p, [0.03, 0.55, 0.02], 0.55); p.blob_m([0.0, 1.03, 0.025], [0.025, 0.025, 0.02], pearl, pearl, PLAIN); for y in [-0.1, 0.1] { p.blob_m([0.02, y, 0.01], [0.022, 0.018, 0.022], barn, sk2, PLAIN); }
    });

    // ---- Le Gardien du Soleil : un gardien de basalte poli, vêtu d'or et de lapis, à tête de faucon. Le némès rayé lui tombe sur
    // les épaules, un grand collier d'or et de lapis couvre sa poitrine ; un pagne plissé, un tablier d'or ; derrière lui, un grand
    // disque solaire hérissé de rayons, et deux ailes repliées ; il porte une lance solaire.
    let (onyx, onyx2, gold, gold2, lapis, linen, sglow) = ([0.13, 0.11, 0.12], [0.24, 0.2, 0.22], [0.96, 0.76, 0.26], [0.7, 0.5, 0.15], [0.12, 0.26, 0.68], [0.93, 0.89, 0.76], [1.0, 0.95, 0.6]);
    m("gsoTorso", Mat::Skin, 0.016, &|p| {
        p.column_m(&[(0.0, 0.12, 0.09), (0.22, 0.14, 0.1), (0.42, 0.19, 0.12), (0.56, 0.21, 0.1)], 0.0, onyx, onyx2, 1.0, Mat::Skin);
        // le grand collier : des rangs d'or et de lapis en demi-cercle sur la poitrine et le dos
        for (k, r) in [0.12, 0.155, 0.19, 0.225].iter().enumerate() { let c = if k % 2 == 0 { gold } else { lapis }; arc(p, [0.0, 0.0, 0.0], *r, -1.6, 1.6, 0.56 - k as f64 * 0.025, 0.018, c, c, 10, 0.8); arc(p, [0.0, 0.0, 0.0], *r, 1.6, 4.68, 0.56 - k as f64 * 0.02, 0.016, c, c, 8, 0.75); }
        for k in 0..9 { let a = -1.3 + k as f64 * 0.325; p.blob_m([a.sin() * 0.25, 0.46, a.cos() * 0.2 + 0.01], [0.015, 0.02, 0.01], if k % 2 == 0 { gold } else { [0.85, 0.2, 0.15] }, gold2, PLAIN); }
        // la ceinture, le pagne plissé, le tablier d'or rayé de lapis
        rod(p, [-0.13, 0.02, 0.0], [0.13, 0.02, 0.0], 0.1, 0.1, gold, gold2, 4);
        for k in 0..18 { let a = k as f64 * std::f64::consts::TAU / 18.0; let (c, sn) = (a.cos(), a.sin()); slab(p, [sn * 0.12, -0.02, c * 0.1], [sn * 0.2, -0.4, c * 0.17], 0.05, 0.08, 0.012, [sn, 0.0, c], linen, mix(linen, gold2, 0.2), 4); }
        slab(p, [0.0, -0.02, 0.12], [0.0, -0.46, 0.2], 0.12, 0.16, 0.016, [0.0, 0.0, 1.0], gold, gold2, 4); for y in [-0.12, -0.24, -0.36] { slab(p, [0.0, y, 0.165 + y * -0.12], [0.0, y - 0.03, 0.17 + y * -0.12], 0.14, 0.15, 0.02, [0.0, 0.0, 1.0], lapis, lapis, 1); }
        // le disque solaire et ses rayons, derrière les épaules
        let sc_ = [0.0, 0.95, -0.24]; for k in 0..28 { let a = k as f64 * std::f64::consts::TAU / 28.0; let (c, sn) = (a.cos(), a.sin()); let (r0, r1) = (0.3, if k % 2 == 0 { 0.62 } else { 0.48 });
            rod(p, [sc_[0] + sn * r0, sc_[1] + c * r0, sc_[2]], [sc_[0] + sn * r1, sc_[1] + c * r1, sc_[2] - 0.02], 0.022, 0.005, gold, sglow, 3); }
        for (r, c) in [(0.29, gold), (0.24, gold2), (0.2, gold)] { for k in 0..16 { let (a0, a1) = (k as f64 * std::f64::consts::TAU / 16.0, (k + 1) as f64 * std::f64::consts::TAU / 16.0); rod(p, [sc_[0] + a0.sin() * r, sc_[1] + a0.cos() * r, sc_[2]], [sc_[0] + a1.sin() * r, sc_[1] + a1.cos() * r, sc_[2]], 0.025, 0.025, c, c, 1); } }
        p.blob_m(sc_, [0.18, 0.18, 0.03], sglow, gold, PLAIN);
        // les ailes repliées dans le dos : des rangs de plumes d'or et de lapis
        for sx in [-1.0f64, 1.0] { for k in 0..7 { let y = 0.52 - k as f64 * 0.1; let c = if k % 2 == 0 { gold } else { lapis }; slab(p, [sx * 0.08, y + 0.06, -0.12], [sx * (0.3 - k as f64 * 0.015), y - 0.35, -0.2], 0.1, 0.06, 0.012, [sx * 0.2, 0.0, -1.0], c, mix(c, onyx, 0.2), 4); } }
    });
    m("gsoHead", Mat::Skin, 0.012, &|p| {
        egg(p, [0.0, 0.14, 0.01], [0.075, 0.09, 0.085], onyx, onyx2, 10);
        rod(p, [0.0, 0.14, 0.07], [0.0, 0.11, 0.17], 0.03, 0.012, gold, gold2, 4); rod(p, [0.0, 0.11, 0.17], [0.0, 0.07, 0.18], 0.012, 0.004, gold2, gold2, 2); // le bec de faucon
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.045, 0.155, 0.06], [0.014, 0.014, 0.01], sglow, sglow, PLAIN); rod(p, [sx * 0.05, 0.14, 0.065], [sx * 0.07, 0.09, 0.05], 0.007, 0.005, lapis, lapis, 2); } // l'œil d'Horus
        // le némès : une coiffe rayée d'or et de lapis qui tombe en deux pans sur la poitrine
        egg(p, [0.0, 0.18, -0.01], [0.09, 0.08, 0.095], gold2, gold, 10);
        for sx in [-1.0f64, 1.0] { for k in 0..5 { let c = if k % 2 == 0 { gold } else { lapis }; let y = 0.16 - k as f64 * 0.05; slab(p, [sx * 0.085, y + 0.025, 0.0], [sx * 0.11, y - 0.025, 0.02], 0.12, 0.13, 0.015, [sx, 0.0, 0.0], c, c, 1); } slab(p, [sx * 0.11, -0.08, 0.02], [sx * 0.12, -0.25, 0.07], 0.07, 0.07, 0.014, [0.0, 0.0, 1.0], gold, lapis, 3); }
        p.blob_m([0.0, 0.29, 0.0], [0.05, 0.05, 0.02], [0.9, 0.25, 0.15], [0.7, 0.15, 0.1], PLAIN); rod(p, [0.0, 0.26, 0.06], [0.0, 0.32, 0.07], 0.012, 0.008, gold, gold, 2); // un petit disque rouge, l'uræus
    });
    m("gsoArm", Mat::Skin, 0.012, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.3, 0.0], 0.045, 0.035, onyx2, onyx, 4); rod(p, [0.0, -0.31, 0.0], [0.0, -0.56, 0.02], 0.035, 0.03, onyx2, onyx, 4); p.blob_m([0.0, -0.6, 0.02], [0.034, 0.04, 0.028], onyx, onyx2, Mat::Skin);
        for (y, h) in [(-0.06, 0.04), (-0.4, 0.12)] { rod(p, [0.0, y, 0.0], [0.0, y - h, 0.0], 0.05, 0.045, gold, gold2, 2); } rod(p, [0.0, -0.46, 0.0], [0.0, -0.47, 0.0], 0.05, 0.05, lapis, lapis, 1);
    });
    m("gsoLeg", Mat::Skin, 0.014, &|p| { rod(p, [0.0, 0.0, 0.0], [0.0, -0.44, 0.02], 0.06, 0.045, onyx2, onyx, 4); rod(p, [0.0, -0.45, 0.02], [0.0, -0.84, 0.0], 0.045, 0.038, onyx2, onyx, 4);
        rod(p, [0.0, -0.5, 0.0], [0.0, -0.78, 0.0], 0.05, 0.045, gold, gold2, 3); crate::mesher::obox(p.q, [0.0, -0.865, 0.05], [[0.05, 0.0, 0.0], [0.0, 0.02, 0.0], [0.0, 0.0, 0.11]], gold2, -1.0, 255.0, 0.0); });
    m("gsoSpear", Mat::Metal, 0.014, &|p| {
        rod(p, [0.0, -1.0, 0.0], [0.0, 1.0, 0.0], 0.016, 0.016, gold2, gold, 8); for k in 0..6 { let y = -0.8 + k as f64 * 0.3; rod(p, [0.0, y, 0.0], [0.0, y + 0.06, 0.0], 0.022, 0.022, lapis, lapis, 1); }
        for k in 0..12 { let (a0, a1) = (k as f64 * std::f64::consts::TAU / 12.0, (k + 1) as f64 * std::f64::consts::TAU / 12.0); rod(p, [a0.sin() * 0.09, 1.02 + a0.cos() * 0.09, 0.0], [a1.sin() * 0.09, 1.02 + a1.cos() * 0.09, 0.0], 0.012, 0.012, gold, gold, 1); }
        slab(p, [0.0, 1.1, 0.0], [0.0, 1.45, 0.0], 0.1, 0.01, 0.012, [0.0, 0.0, 1.0], sglow, gold, 6); p.blob_m([0.0, 1.02, 0.0], [0.03, 0.03, 0.015], [0.9, 0.25, 0.15], gold2, PLAIN);
    });

    // ---- Le Premier Roi : un roi squelette. Une vraie charpente d'os — colonne, côtes en arceaux, sternum, bassin, os longs —
    // sous un grand manteau pourpre doublé d'hermine, une chaîne d'or sur les côtes ; un crâne aux orbites où brûle une braise,
    // une couronne d'or fleuronnée ; une longue épée à deux mains.
    let (bone, bone2, crim, crim2, erm, egold, ember) = ([0.88, 0.84, 0.72], [0.6, 0.56, 0.46], [0.48, 0.07, 0.1], [0.22, 0.03, 0.06], [0.95, 0.94, 0.9], [0.92, 0.73, 0.22], [1.0, 0.55, 0.2]);
    m("prrTorso", Mat::Bone, 0.012, &|p| {
        for k in 0..12 { let y = -0.02 + k as f64 * 0.05; p.blob_m([0.0, y, -0.06], [0.03, 0.02, 0.03], bone, bone2, Mat::Bone); } // la colonne
        for k in 0..6 { let y = 0.26 + k as f64 * 0.05; let w = 0.12 + 0.04 * (1.0 - ((k as f64 - 3.5) / 3.5).powi(2)); for sx in [-1.0f64, 1.0] { // les côtes en arceaux
            rod(p, [sx * 0.02, y, -0.07], [sx * w, y - 0.02, -0.03], 0.011, 0.011, bone, bone2, 2); rod(p, [sx * w, y - 0.02, -0.03], [sx * (w - 0.02), y - 0.05, 0.07], 0.011, 0.011, bone, bone2, 2); rod(p, [sx * (w - 0.02), y - 0.05, 0.07], [sx * 0.015, y - 0.03, 0.1], 0.01, 0.01, bone, bone2, 2); } }
        rod(p, [0.0, 0.5, 0.1], [0.0, 0.26, 0.1], 0.016, 0.014, bone, bone2, 3); for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.02, 0.54, 0.08], [sx * 0.2, 0.56, 0.0], 0.014, 0.012, bone, bone2, 3); slab(p, [sx * 0.08, 0.5, -0.1], [sx * 0.12, 0.34, -0.1], 0.1, 0.06, 0.012, [0.0, 0.0, -1.0], bone, bone2, 2); }
        egg(p, [0.0, 0.0, 0.0], [0.13, 0.06, 0.07], bone2, bone, 10); for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.1, 0.03, -0.01], [0.05, 0.06, 0.04], bone, bone2, Mat::Bone); }
        // le manteau pourpre, en lambeaux, qui traîne derrière ; le col d'hermine ; une ceinture, un pan de tabard
        for k in 0..9 { let a = -2.1 + k as f64 * 0.525; let (sn, c) = (a.sin(), a.cos()); let l = 1.36 - 0.16 * ((k * 5 % 4) as f64 / 3.0);
            slab(p, [sn * 0.2, 0.57, c * -0.1 - 0.04], [sn * 0.45, 0.57 - l, -c * 0.38 - 0.1], 0.13, 0.28, 0.016, [sn, 0.1, -c], crim, crim2, 9); }
        for k in 0..18 { let a = -2.4 + k as f64 * 0.28; let q = [a.sin() * 0.21, 0.58, a.cos() * 0.12 - 0.02]; p.blob_m(q, [0.045, 0.035, 0.04], erm, erm, Mat::Fur); if k % 3 == 1 { p.blob_m([q[0], q[1] + 0.01, q[2] + 0.035 * a.cos().signum()], [0.01, 0.014, 0.008], [0.08, 0.07, 0.07], [0.08, 0.07, 0.07], PLAIN); } }
        rod(p, [-0.13, -0.02, 0.05], [0.13, -0.02, 0.05], 0.02, 0.02, [0.25, 0.16, 0.08], [0.25, 0.16, 0.08], 4); slab(p, [0.0, -0.03, 0.09], [0.0, -0.55, 0.12], 0.14, 0.12, 0.012, [0.0, 0.0, 1.0], crim, crim2, 6);
        arc(p, [0.0, 0.0, 0.0], 0.15, -1.2, 1.2, 0.46, 0.01, egold, egold, 8, 0.8); p.blob_m([0.0, 0.33, 0.12], [0.035, 0.045, 0.012], egold, [0.6, 0.45, 0.1], Mat::Metal); p.blob_m([0.0, 0.335, 0.132], [0.014, 0.016, 0.006], [0.75, 0.1, 0.15], [0.75, 0.1, 0.15], PLAIN);
    });
    m("prrHead", Mat::Bone, 0.012, &|p| {
        egg(p, [0.0, 0.16, -0.01], [0.085, 0.09, 0.1], bone2, bone, 10); egg(p, [0.0, 0.09, 0.035], [0.065, 0.045, 0.06], bone2, bone, 8);
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.035, 0.145, 0.075], [0.025, 0.022, 0.016], [0.05, 0.04, 0.04], [0.05, 0.04, 0.04], PLAIN); p.blob_m([sx * 0.035, 0.145, 0.085], [0.008, 0.008, 0.006], ember, ember, PLAIN); }
        crate::mesher::obox(p.q, [0.0, 0.105, 0.095], [[0.012, 0.0, 0.0], [0.0, 0.018, 0.0], [0.0, 0.0, 0.006]], [0.06, 0.05, 0.04], -1.0, 255.0, 0.0);
        rod(p, [-0.05, 0.035, 0.05], [0.05, 0.035, 0.05], 0.022, 0.022, bone, bone2, 4); for k in 0..8 { let x = -0.035 + k as f64 * 0.01; rod(p, [x, 0.06, 0.088], [x, 0.045, 0.088], 0.004, 0.004, bone, bone, 1); }
        // la couronne d'or : un bandeau, huit fleurons, des gemmes
        arc(p, [0.0, 0.0, -0.01], 0.092, 0.0, std::f64::consts::TAU, 0.22, 0.016, egold, egold, 14, 1.05);
        for k in 0..8 { let a = k as f64 * std::f64::consts::TAU / 8.0; let (sn, c) = (a.sin(), a.cos()); let q = [sn * 0.094, 0.235, c * 0.098 - 0.01];
            rod(p, q, [q[0], q[1] + 0.07, q[2]], 0.012, 0.004, egold, egold, 2); for sd in [-1.0, 1.0] { rod(p, [q[0], q[1] + 0.03, q[2]], [q[0] + c * sd * 0.025, q[1] + 0.055, q[2] - sn * sd * 0.025], 0.008, 0.003, egold, egold, 2); }
            if k % 2 == 0 { p.blob_m([sn * 0.1, 0.22, c * 0.104 - 0.01], [0.012, 0.012, 0.008], [0.75, 0.1, 0.15], [0.5, 0.05, 0.1], PLAIN); } }
    });
    m("prrArm", Mat::Bone, 0.012, &|p| {
        p.blob_m([0.0, 0.0, 0.0], [0.04, 0.035, 0.04], bone, bone2, Mat::Bone); rod(p, [0.0, -0.02, 0.0], [0.0, -0.3, 0.0], 0.018, 0.016, bone, bone2, 4); p.blob_m([0.0, -0.31, 0.0], [0.025, 0.022, 0.025], bone, bone2, Mat::Bone);
        for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.012, -0.32, 0.0], [sx * 0.01, -0.56, 0.02], 0.011, 0.01, bone, bone2, 4); }
        p.blob_m([0.0, -0.585, 0.02], [0.026, 0.02, 0.016], bone, bone2, Mat::Bone); for x in [-0.018, -0.006, 0.006, 0.018] { rod(p, [x, -0.6, 0.025], [x * 1.3, -0.69, 0.04], 0.005, 0.004, bone, bone2, 3); }
        slab(p, [0.0, -0.02, -0.03], [0.0, -0.36, -0.08], 0.11, 0.13, 0.012, [0.0, 0.0, -1.0], crim, crim2, 4); rod(p, [0.006, -0.635, 0.03], [0.006, -0.645, 0.03], 0.008, 0.008, egold, egold, 1);
    });
    m("prrLeg", Mat::Bone, 0.012, &|p| { rod(p, [0.0, 0.0, 0.0], [0.0, -0.42, 0.02], 0.022, 0.018, bone, bone2, 4); p.blob_m([0.0, -0.43, 0.025], [0.03, 0.03, 0.03], bone, bone2, Mat::Bone);
        for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.012, -0.45, 0.02], [sx * 0.01, -0.83, 0.0], 0.013, 0.011, bone, bone2, 4); }
        crate::mesher::obox(p.q, [0.0, -0.86, 0.05], [[0.04, 0.0, 0.0], [0.0, 0.018, 0.0], [0.0, 0.0, 0.1]], bone2, -1.0, 255.0, 0.0); for x in [-0.025, 0.0, 0.025] { rod(p, [x, -0.865, 0.14], [x, -0.865, 0.17], 0.007, 0.006, bone, bone, 1); } });
    m("prrSword", Mat::Metal, 0.014, &|p| {
        let steel = [0.72, 0.74, 0.78]; rod(p, [0.0, -0.3, 0.0], [0.0, 0.06, 0.0], 0.02, 0.02, [0.25, 0.14, 0.08], [0.25, 0.14, 0.08], 3); p.blob_m([0.0, -0.32, 0.0], [0.03, 0.03, 0.03], egold, egold, Mat::Metal);
        rod(p, [-0.18, 0.08, 0.0], [0.18, 0.08, 0.0], 0.022, 0.022, egold, egold, 4); for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.18, 0.08, 0.0], [sx * 0.22, 0.13, 0.0], 0.016, 0.006, egold, egold, 2); }
        slab(p, [0.0, 0.1, 0.0], [0.0, 1.05, 0.0], 0.085, 0.06, 0.016, [0.0, 0.0, 1.0], steel, mix(steel, [1.0, 1.0, 1.0], 0.3), 8); rod(p, [0.0, 1.05, 0.0], [0.0, 1.16, 0.0], 0.025, 0.003, steel, steel, 2);
        rod(p, [0.0, 0.12, 0.0], [0.0, 0.8, 0.0], 0.009, 0.009, mix(steel, [0.3, 0.3, 0.35], 0.5), steel, 6); p.blob_m([0.0, 0.08, 0.02], [0.016, 0.016, 0.01], [0.75, 0.1, 0.15], [0.5, 0.05, 0.1], PLAIN);
    });

    // ---- Le Maître de forge : un forgeron colossal. Un poitrail en tonneau, des bras énormes nus et brûlés, un tablier de cuir
    // roussi clouté, des lanières, une ceinture d'outils ; un crâne chauve, des lunettes de forge qui rougeoient sur le front, deux
    // longues tresses de barbe cerclées de fer ; un marteau de guerre dont la tête luit comme un fer sorti du feu.
    let (hide, hide2, leath, leath2, iron, soot, embr, beard) = ([0.74, 0.47, 0.34], [0.52, 0.31, 0.22], [0.38, 0.23, 0.12], [0.22, 0.13, 0.07], [0.36, 0.36, 0.4], [0.14, 0.12, 0.11], [1.0, 0.5, 0.12], [0.55, 0.2, 0.08]);
    m("mfoTorso", Mat::Flesh, 0.02, &|p| {
        egg(p, [0.0, 0.42, 0.02], [0.36, 0.3, 0.27], hide2, hide, 14); egg(p, [0.0, 0.12, 0.04], [0.28, 0.2, 0.24], hide2, hide, 12); egg(p, [0.0, 0.0, 0.0], [0.24, 0.12, 0.18], leath2, leath, 12);
        for sx in [-1.0f64, 1.0] { egg(p, [sx * 0.36, 0.6, 0.0], [0.14, 0.12, 0.14], hide, hide2, 10); } egg(p, [-0.38, 0.65, 0.0], [0.16, 0.08, 0.16], soot, iron, 10); for k in 0..3 { rod(p, [-0.5, 0.6 - k as f64 * 0.04, -0.1], [-0.5, 0.6 - k as f64 * 0.04, 0.1], 0.012, 0.012, soot, soot, 2); }
        // le tablier de cuir roussi, clouté, des brûlures ; ses lanières sur les épaules
        slab(p, [0.0, 0.5, 0.26], [0.0, -0.62, 0.28], 0.4, 0.46, 0.03, [0.0, 0.0, 1.0], leath, leath2, 9); for (x, y) in [(-0.17, 0.42), (0.17, 0.42), (-0.19, -0.1), (0.19, -0.1), (-0.2, -0.5), (0.2, -0.5)] { p.blob_m([x, y, 0.3], [0.014, 0.014, 0.008], iron, iron, Mat::Metal); }
        for (x, y, r) in [(0.08, 0.1, 0.05), (-0.1, -0.3, 0.035), (0.12, -0.44, 0.03)] { p.blob_m([x, y, 0.295], [r, r * 0.8, 0.006], soot, embr, PLAIN); } slab(p, [-0.1, 0.0, 0.3], [-0.1, -0.16, 0.31], 0.14, 0.14, 0.012, [0.0, 0.0, 1.0], leath2, leath2, 2);
        for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.18, 0.5, 0.27], [sx * 0.22, 0.7, 0.0], 0.03, 0.03, leath2, leath2, 3); rod(p, [sx * 0.22, 0.7, 0.0], [sx * 0.1, 0.4, -0.27], 0.03, 0.03, leath2, leath2, 3); }
        // la ceinture d'outils : une boucle de fer, des tenailles, un petit marteau, une chaîne
        rod(p, [-0.26, -0.02, 0.05], [0.26, -0.02, 0.05], 0.045, 0.045, leath2, leath2, 6); p.blob_m([0.0, -0.02, 0.25], [0.05, 0.04, 0.02], iron, soot, Mat::Metal);
        rod(p, [0.26, -0.04, 0.05], [0.28, -0.4, 0.1], 0.012, 0.01, iron, soot, 3); rod(p, [0.28, -0.04, 0.09], [0.31, -0.38, 0.14], 0.012, 0.01, iron, soot, 3); rod(p, [-0.27, -0.05, 0.04], [-0.29, -0.32, 0.05], 0.014, 0.014, leath, leath2, 3); crate::mesher::obox(p.q, [-0.29, -0.34, 0.05], [[0.05, 0.0, 0.0], [0.0, 0.03, 0.0], [0.0, 0.0, 0.03]], iron, -1.0, 255.0, 0.0);
        for k in 0..6 { let a = -1.0 + k as f64 * 0.4; p.blob_m([a.sin() * 0.27, -0.12 - 0.04 * (1.0 - a.cos()), a.cos() * 0.2 - 0.02], [0.018, 0.012, 0.018], iron, soot, Mat::Metal); }
    });
    m("mfoHead", Mat::Flesh, 0.014, &|p| {
        egg(p, [0.0, 0.14, 0.02], [0.11, 0.12, 0.11], hide2, hide, 10); rod(p, [-0.09, 0.17, 0.11], [0.09, 0.17, 0.11], 0.024, 0.024, hide2, hide2, 4); rod(p, [0.0, 0.16, 0.12], [0.0, 0.1, 0.145], 0.02, 0.026, hide, hide2, 2);
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.04, 0.145, 0.105], [0.016, 0.01, 0.008], [0.1, 0.08, 0.07], [0.1, 0.08, 0.07], PLAIN); p.blob_m([sx * 0.11, 0.14, 0.0], [0.02, 0.035, 0.025], hide2, hide, Mat::Flesh); }
        // les lunettes de forge relevées sur le front, qui rougeoient
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.045, 0.235, 0.09], [0.035, 0.03, 0.02], iron, soot, Mat::Metal); p.blob_m([sx * 0.045, 0.235, 0.105], [0.025, 0.02, 0.008], embr, [1.0, 0.85, 0.4], PLAIN); } arc(p, [0.0, 0.0, 0.01], 0.115, 1.1, 5.2, 0.23, 0.012, leath2, leath2, 10, 1.0);
        // la moustache, deux longues tresses de barbe cerclées de fer
        for sx in [-1.0f64, 1.0] { rod(p, [0.0, 0.085, 0.13], [sx * 0.08, 0.06, 0.11], 0.02, 0.01, beard, beard, 3); rod(p, [sx * 0.035, 0.07, 0.11], [sx * 0.05, -0.35, 0.15], 0.032, 0.014, beard, mix(beard, soot, 0.3), 6); for y in [-0.08, -0.22] { rod(p, [sx * 0.042, y, 0.13], [sx * 0.043, y - 0.03, 0.135], 0.03, 0.03, iron, iron, 1); } }
        egg(p, [0.0, 0.06, 0.08], [0.09, 0.06, 0.06], mix(beard, soot, 0.3), beard, 8);
    });
    m("mfoArm", Mat::Flesh, 0.016, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.3, 0.0], 0.1, 0.08, hide, hide2, 4); egg(p, [0.0, -0.31, 0.0], [0.08, 0.07, 0.08], hide2, hide, 8); rod(p, [0.0, -0.32, 0.0], [0.0, -0.56, 0.03], 0.085, 0.07, hide, hide2, 4);
        rod(p, [0.0, -0.44, 0.0], [0.0, -0.56, 0.03], 0.085, 0.075, iron, soot, 3); for y in [-0.46, -0.53] { rod(p, [0.0, y, 0.0], [0.0, y - 0.01, 0.0], 0.09, 0.09, soot, soot, 1); }
        egg(p, [0.0, -0.63, 0.04], [0.085, 0.08, 0.085], leath2, leath, 8); for (x, y, r) in [(0.04, -0.15, 0.025), (-0.05, -0.38, 0.02)] { p.blob_m([x, y, 0.085], [r, r * 1.4, 0.008], [0.86, 0.5, 0.42], hide2, PLAIN); }
    });
    m("mfoLeg", Mat::Leather, 0.018, &|p| { rod(p, [0.0, 0.0, 0.0], [0.0, -0.42, 0.02], 0.12, 0.1, soot, [0.24, 0.2, 0.18], 4); rod(p, [0.0, -0.43, 0.02], [0.0, -0.64, 0.0], 0.095, 0.085, soot, [0.24, 0.2, 0.18], 3);
        rod(p, [0.0, -0.6, 0.0], [0.0, -0.78, 0.01], 0.1, 0.1, leath, leath2, 3); crate::mesher::obox(p.q, [0.0, -0.8, 0.06], [[0.1, 0.0, 0.0], [0.0, 0.03, 0.0], [0.0, 0.0, 0.17]], leath2, -1.0, 255.0, 0.0); crate::mesher::obox(p.q, [0.0, -0.79, 0.19], [[0.09, 0.0, 0.0], [0.0, 0.035, 0.0], [0.0, 0.0, 0.05]], iron, -1.0, 255.0, 0.0); });
    m("mfoHammer", Mat::Metal, 0.016, &|p| {
        rod(p, [0.0, -0.35, 0.0], [0.0, 0.95, 0.0], 0.03, 0.035, leath, [0.3, 0.2, 0.12], 6); for y in [-0.3, -0.2, -0.1] { rod(p, [0.0, y, 0.0], [0.0, y + 0.04, 0.0], 0.04, 0.04, leath2, leath2, 1); } rod(p, [0.0, 0.7, 0.0], [0.0, 0.95, 0.0], 0.045, 0.045, iron, soot, 2);
        crate::mesher::obox(p.q, [0.0, 1.02, 0.0], [[0.12, 0.0, 0.0], [0.0, 0.13, 0.0], [0.0, 0.0, 0.24]], iron, -1.0, 255.0, 0.0);
        for sz in [-1.0f64, 1.0] { crate::mesher::obox(p.q, [0.0, 1.02, sz * 0.245], [[0.08, 0.0, 0.0], [0.0, 0.09, 0.0], [0.0, 0.0, 0.01]], embr, -1.0, 255.0, 0.0); crate::mesher::obox(p.q, [0.0, 1.02, sz * 0.25], [[0.04, 0.0, 0.0], [0.0, 0.045, 0.0], [0.0, 0.0, 0.01]], [1.0, 0.85, 0.4], -1.0, 255.0, 0.0); }
        for (y, z) in [(0.97, -0.24), (1.08, -0.24), (1.02, 0.24)] { rod(p, [-0.121, y, z * 0.5], [-0.121, y + 0.05, z * 0.5 + 0.08], 0.008, 0.008, embr, [1.0, 0.85, 0.4], 2); rod(p, [0.121, y, z * 0.5], [0.121, y + 0.05, z * 0.5 + 0.08], 0.008, 0.008, embr, [1.0, 0.85, 0.4], 2); }
        for sx in [-1.0f64, 1.0] { rod(p, [sx * 0.12, 0.9, 0.0], [sx * 0.12, 1.14, 0.0], 0.02, 0.02, soot, soot, 2); }
    });

    // ---- L'Automate de l'horloger : un chevalier mécanique de laiton. Un poitrail en tonneau riveté, percé d'un hublot de verre
    // où tourne un rouage ; des épaulières, une taille d'abeille ; un heaume en dôme, une visière en T qui luit d'ambre ; des
    // membres en tubes articulés sur des rotules ; dans le dos, la grande clé qui le remonte (et tourne) ; pour arme, l'aiguille
    // des heures de la tour, forgée en lance, lestée d'un contrepoids.
    let (br, br2, brd, cop, verd, stl, amb, glass) = ([0.88, 0.68, 0.32], [0.62, 0.45, 0.2], [0.36, 0.25, 0.12], [0.74, 0.42, 0.25], [0.32, 0.6, 0.5], [0.6, 0.62, 0.66], [1.0, 0.72, 0.25], [0.66, 0.84, 0.9]);
    let rivets = |p: &mut Pen, c: V, rx: f64, rz: f64, y: f64, n: usize| { for k in 0..n { let a = k as f64 * std::f64::consts::TAU / n as f64; p.blob_m([c[0] + a.sin() * rx, c[1] + y, c[2] + a.cos() * rz], [0.014, 0.014, 0.014], brd, br2, Mat::Metal); } };
    m("autTorso", Mat::Metal, 0.016, &|p| {
        // le tonneau du poitrail, ses cerclages et ses rivets ; la taille d'abeille ; le bassin
        egg(p, [0.0, 0.38, 0.0], [0.27, 0.26, 0.21], br2, br, 14); for y in [0.22, 0.52] { arc(p, [0.0, 0.0, 0.0], 0.255, 0.0, std::f64::consts::TAU, y, 0.018, brd, br2, 16, 0.8); } rivets(p, [0.0, 0.0, 0.0], 0.262, 0.212, 0.37, 14);
        rod(p, [0.0, 0.14, 0.0], [0.0, -0.02, 0.0], 0.1, 0.075, brd, br2, 3); egg(p, [0.0, -0.06, 0.0], [0.17, 0.1, 0.13], br2, br, 12); rod(p, [-0.17, -0.06, 0.02], [0.17, -0.06, 0.02], 0.03, 0.03, cop, cop, 5);
        // le hublot du poitrail : un anneau de cuivre, une vitre, le rouage qui tourne derrière
        for k in 0..14 { let a = k as f64 * std::f64::consts::TAU / 14.0; let (x, y) = (a.sin() * 0.11, a.cos() * 0.11 + 0.4); rod(p, [x, y, 0.2], [x, y, 0.245], 0.022, 0.022, cop, cop, 1); }
        p.blob_m([0.0, 0.4, 0.228], [0.095, 0.095, 0.008], mix(glass, amb, 0.35), glass, PLAIN); p.blob_m([0.0, 0.4, 0.232], [0.06, 0.06, 0.008], amb, [1.0, 0.9, 0.5], PLAIN);
        for k in 0..6 { let a = k as f64 * std::f64::consts::PI / 6.0; rod(p, [a.sin() * -0.075, 0.4 + a.cos() * -0.075, 0.24], [a.sin() * 0.075, 0.4 + a.cos() * 0.075, 0.24], 0.009, 0.009, brd, brd, 2); } p.blob_m([0.0, 0.4, 0.245], [0.022, 0.022, 0.01], brd, brd, Mat::Metal);
        // les épaulières de laiton à trois lames, un col de cuivre vert-de-gris
        for sx in [-1.0f64, 1.0] { egg(p, [sx * 0.27, 0.6, 0.0], [0.12, 0.1, 0.13], br2, br, 10); for k in 0..3 { let y = 0.62 - k as f64 * 0.06; slab(p, [sx * 0.2, y, 0.0], [sx * (0.38 + k as f64 * 0.02), y - 0.08, 0.0], 0.26, 0.24, 0.02, [sx, 0.6, 0.0], br, brd, 2); } }
        rod(p, [0.0, 0.62, 0.0], [0.0, 0.7, 0.0], 0.09, 0.07, verd, cop, 3);
        // la jupe de lames articulées sur les hanches
        for k in 0..8 { let a = (k as f64 - 3.5) * 0.36; slab(p, [a.sin() * 0.15, -0.08, a.cos() * 0.12], [a.sin() * 0.21, -0.32, a.cos() * 0.17], 0.09, 0.1, 0.016, [a.sin(), -0.2, a.cos()], br, br2, 3); }
        for k in 0..3 { let a = std::f64::consts::PI + (k as f64 - 1.0) * 0.5; slab(p, [a.sin() * 0.15, -0.08, a.cos() * 0.12], [a.sin() * 0.21, -0.3, a.cos() * 0.17], 0.1, 0.1, 0.016, [a.sin(), -0.2, a.cos()], br2, brd, 3); }
        // le moyeu de la clé, dans le dos
        rod(p, [0.0, 0.4, -0.18], [0.0, 0.4, -0.25], 0.04, 0.04, brd, br2, 2);
    });
    m("autKey", Mat::Metal, 0.016, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, 0.0, -0.18], 0.022, 0.022, br2, br, 3);
        for sx in [-1.0f64, 1.0] { arc(p, [sx * 0.1, 0.0, -0.2], 0.075, 0.0, std::f64::consts::TAU, 0.0, 0.018, br, br2, 10, 0.4); p.blob_m([sx * 0.1, 0.0, -0.2], [0.03, 0.03, 0.03], brd, brd, Mat::Metal); }
    });
    m("autHead", Mat::Metal, 0.012, &|p| {
        // un heaume en dôme, une visière en T qui luit, une crête de lames, des rouages aux tempes
        egg(p, [0.0, 0.14, 0.0], [0.12, 0.14, 0.13], br2, br, 12); rod(p, [0.0, 0.0, 0.0], [0.0, 0.05, 0.0], 0.11, 0.12, brd, br2, 2);
        crate::mesher::obox(p.q, [0.0, 0.15, 0.13], [[0.085, 0.0, 0.0], [0.0, 0.02, 0.0], [0.0, 0.0, 0.016]], amb, -1.0, 255.0, 0.0); crate::mesher::obox(p.q, [0.0, 0.1, 0.135], [[0.02, 0.0, 0.0], [0.0, 0.055, 0.0], [0.0, 0.0, 0.014]], amb, -1.0, 255.0, 0.0);
        for k in 0..5 { let z = 0.1 - k as f64 * 0.05; slab(p, [0.0, 0.26, z], [0.0, 0.34 - (k as f64 - 2.0).abs() * 0.02, z - 0.03], 0.05, 0.03, 0.014, [1.0, 0.0, 0.0], cop, verd, 2); }
        for sx in [-1.0f64, 1.0] { arc(p, [sx * 0.125, 0.14, 0.0], 0.045, 0.0, std::f64::consts::TAU, 0.0, 0.012, brd, br2, 8, 1.0); p.blob_m([sx * 0.13, 0.14, 0.0], [0.016, 0.03, 0.03], cop, cop, Mat::Metal); }
        rivets(p, [0.0, 0.0, 0.0], 0.12, 0.13, 0.05, 10);
    });
    m("autArm", Mat::Metal, 0.014, &|p| {
        p.blob_m([0.0, 0.0, 0.0], [0.07, 0.07, 0.07], brd, br2, Mat::Metal); rod(p, [0.0, -0.03, 0.0], [0.0, -0.3, 0.0], 0.055, 0.05, br, br2, 4); for y in [-0.1, -0.22] { rod(p, [0.0, y, 0.0], [0.0, y - 0.015, 0.0], 0.062, 0.062, cop, cop, 1); }
        p.blob_m([0.0, -0.32, 0.0], [0.055, 0.055, 0.055], brd, stl, Mat::Metal); rod(p, [0.0, -0.34, 0.0], [0.0, -0.55, 0.02], 0.05, 0.06, br2, br, 4); slab(p, [0.0, -0.36, 0.06], [0.0, -0.54, 0.07], 0.08, 0.1, 0.014, [0.0, 0.0, 1.0], br, br2, 2);
        // la main : une paume, quatre doigts en tubes, un pouce
        egg(p, [0.0, -0.61, 0.03], [0.05, 0.045, 0.05], br2, brd, 8); for k in 0..4 { let x = (k as f64 - 1.5) * 0.025; rod(p, [x, -0.64, 0.05], [x, -0.7, 0.07], 0.012, 0.01, stl, br2, 2); } rod(p, [0.04, -0.6, 0.05], [0.06, -0.64, 0.08], 0.012, 0.01, stl, br2, 2);
    });
    m("autLeg", Mat::Metal, 0.016, &|p| {
        p.blob_m([0.0, 0.0, 0.0], [0.08, 0.07, 0.08], brd, br2, Mat::Metal); rod(p, [0.0, -0.03, 0.0], [0.0, -0.38, 0.02], 0.075, 0.065, br, br2, 4); rod(p, [0.0, -0.18, 0.0], [0.0, -0.2, 0.0], 0.08, 0.08, cop, cop, 1);
        p.blob_m([0.0, -0.4, 0.02], [0.065, 0.065, 0.065], brd, stl, Mat::Metal); slab(p, [0.0, -0.36, 0.08], [0.0, -0.46, 0.09], 0.1, 0.08, 0.02, [0.0, 0.0, 1.0], br, br2, 2);
        rod(p, [0.0, -0.42, 0.02], [0.0, -0.74, 0.0], 0.065, 0.055, br2, br, 4); slab(p, [0.0, -0.5, 0.07], [0.0, -0.72, 0.06], 0.09, 0.075, 0.016, [0.0, 0.0, 1.0], br, br2, 3);
        crate::mesher::obox(p.q, [0.0, -0.8, 0.05], [[0.075, 0.0, 0.0], [0.0, 0.035, 0.0], [0.0, 0.0, 0.14]], br2, -1.0, 255.0, 0.0); crate::mesher::obox(p.q, [0.0, -0.8, 0.19], [[0.065, 0.0, 0.0], [0.0, 0.03, 0.0], [0.0, 0.0, 0.035]], brd, -1.0, 255.0, 0.0);
    });
    m("autLance", Mat::Metal, 0.014, &|p| {
        // la hampe d'acier, le contrepoids en bas, puis l'aiguille des heures : une lame en fer de lance ajourée, du cuivre au laiton
        rod(p, [0.0, -0.45, 0.0], [0.0, 0.85, 0.0], 0.022, 0.022, stl, mix(stl, brd, 0.4), 8); egg(p, [0.0, -0.5, 0.0], [0.06, 0.07, 0.06], br2, brd, 8); arc(p, [0.0, 0.0, 0.0], 0.06, 0.0, std::f64::consts::TAU, -0.36, 0.012, br, br, 8, 1.0);
        for y in [-0.2, 0.0, 0.3] { rod(p, [0.0, y, 0.0], [0.0, y + 0.03, 0.0], 0.03, 0.03, br2, br2, 1); }
        slab(p, [0.0, 0.85, 0.0], [0.0, 1.12, 0.0], 0.03, 0.22, 0.02, [0.0, 0.0, 1.0], br2, br, 4); slab(p, [0.0, 1.12, 0.0], [0.0, 1.5, 0.0], 0.22, 0.01, 0.02, [0.0, 0.0, 1.0], br, mix(br, [1.0, 0.95, 0.7], 0.4), 6);
        p.blob_m([0.0, 1.12, 0.0], [0.045, 0.045, 0.02], brd, brd, Mat::Metal); for sx in [-1.0f64, 1.0] { arc(p, [sx * 0.06, 1.1, 0.0], 0.03, 0.0, std::f64::consts::TAU, 0.0, 0.006, brd, brd, 6, 1.0); }
    });

    // ---- L'Astronome : un vieillard long et maigre, dans une robe bleu de nuit semée d'étoiles d'or, une cape étoilée, un capuchon
    // bordé d'or, une longue barbe blanche, une lunette d'oculaire vissée à l'œil, des yeux qui luisent ; pour bâton, une sphère
    // armillaire (trois anneaux croisés autour d'une étoile).
    let (nv1, nv2, nv3, gld, gld2, skn, bea, eye) = ([0.1, 0.12, 0.3], [0.17, 0.2, 0.45], [0.06, 0.07, 0.18], [0.95, 0.78, 0.35], [0.7, 0.52, 0.2], [0.85, 0.76, 0.68], [0.92, 0.92, 0.9], [0.75, 0.9, 1.0]);
    // un anneau vertical (dans le plan qui contient `n0` et l'axe vertical) fait de petits tubes
    let ring = |p: &mut Pen, c: V, r: f64, n0: V, th: f64, col: C, k: usize| { for i in 0..k { let (a0, a1) = (i as f64 / k as f64 * std::f64::consts::TAU, (i + 1) as f64 / k as f64 * std::f64::consts::TAU);
        let pt = |a: f64| [c[0] + n0[0] * a.cos() * r, c[1] + a.sin() * r, c[2] + n0[2] * a.cos() * r]; rod(p, pt(a0), pt(a1), th, th, col, col, 1); } };
    let stars = |p: &mut Pen, pts: &[V]| { for q in pts { p.blob_m(*q, [0.014, 0.014, 0.01], gld, [1.0, 0.95, 0.7], PLAIN); } };
    m("astTorso", Mat::Cloth, 0.014, &|p| {
        // la robe : un buste étroit, une jupe longue qui s'évase jusqu'aux chevilles ; galons d'or, ceinture, l'astrolabe à la ceinture
        shell(p, [0.0, 0.0, 0.0], &[(-0.62, 0.27, 0.24, 0.0), (-0.3, 0.22, 0.19, 0.0), (0.0, 0.16, 0.13, 0.0), (0.3, 0.19, 0.15, 0.0), (0.56, 0.21, 0.15, 0.0), (0.66, 0.08, 0.08, 0.0)], 12, nv3, nv1, 0.03);
        for y in [-0.6, -0.56] { arc(p, [0.0, 0.0, 0.0], 0.272, 0.0, std::f64::consts::TAU, y, 0.012, gld, gld2, 16, 0.89); } arc(p, [0.0, 0.0, 0.0], 0.165, 0.0, std::f64::consts::TAU, 0.02, 0.022, gld2, gld, 12, 0.8);
        slab(p, [0.0, 0.6, 0.14], [0.0, -0.6, 0.235], 0.05, 0.08, 0.012, [0.0, 0.0, 1.0], gld, gld2, 6);
        ring(p, [0.1, -0.04, 0.15], 0.055, [0.8, 0.0, 0.6], 0.01, gld, 10); p.blob_m([0.1, -0.04, 0.15], [0.03, 0.03, 0.01], gld2, gld, Mat::Metal);
        stars(p, &[[0.12, -0.3, 0.18], [-0.15, -0.45, 0.21], [0.2, 0.3, 0.12], [-0.17, 0.2, 0.12], [-0.08, -0.2, 0.17], [0.18, -0.52, 0.22], [-0.2, -0.12, 0.15]]);
        // la cape étoilée, du col aux talons, et son col haut doré
        for k in 0..5 { let x = (k as f64 - 2.0) * 0.1; slab(p, [x * 0.8, 0.6, -0.15], [x * 1.3, -0.64, -0.3 - (k as f64 - 2.0).abs() * 0.02], 0.12, 0.17, 0.02, [x, 0.0, -1.0], nv2, nv3, 6); }
        stars(p, &[[0.1, 0.3, -0.21], [-0.12, -0.1, -0.27], [0.18, -0.4, -0.31], [-0.05, 0.45, -0.18], [0.0, -0.3, -0.29], [-0.2, 0.15, -0.23], [0.22, 0.05, -0.25]]);
        for k in 0..7 { let a = (k as f64 - 3.0) * 0.42 + std::f64::consts::PI; slab(p, [a.sin() * 0.12, 0.6, a.cos() * 0.1], [a.sin() * 0.16, 0.76, a.cos() * 0.14], 0.07, 0.08, 0.014, [a.sin(), 0.0, a.cos()], gld2, gld, 2); }
    });
    m("astHead", Mat::Cloth, 0.012, &|p| {
        // un visage long et pâle, une barbe blanche en pointe, le capuchon bleu de nuit bordé d'or ; la lunette d'oculaire à l'œil droit
        egg(p, [0.0, 0.12, 0.01], [0.08, 0.11, 0.09], skn, mix(skn, [0.6, 0.5, 0.45], 0.4), 10); p.blob_m([0.0, 0.12, 0.095], [0.016, 0.03, 0.02], skn, skn, PLAIN);
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.032, 0.15, 0.085], [0.016, 0.01, 0.008], eye, [1.0, 1.0, 1.0], PLAIN); slab(p, [sx * 0.05, 0.18, 0.08], [sx * 0.015, 0.175, 0.09], 0.012, 0.01, 0.01, [0.0, 0.0, 1.0], bea, bea, 1); }
        for k in 0..5 { let x = (k as f64 - 2.0) * 0.022; slab(p, [x * 1.6, 0.08, 0.075], [x * 0.5, -0.12 - (2.0 - (k as f64 - 2.0).abs()) * 0.025, 0.06], 0.04, 0.012, 0.016, [0.0, 0.0, 1.0], bea, mix(bea, [0.7, 0.72, 0.75], 0.5), 4); }
        rod(p, [0.032, 0.15, 0.09], [0.032, 0.15, 0.15], 0.022, 0.018, gld2, gld, 3); p.blob_m([0.032, 0.15, 0.152], [0.016, 0.016, 0.006], eye, [1.0, 1.0, 1.0], PLAIN);
        shell(p, [0.0, 0.0, -0.01], &[(0.0, 0.11, 0.12, -0.01), (0.14, 0.12, 0.13, -0.01), (0.24, 0.09, 0.1, -0.03), (0.3, 0.05, 0.06, -0.06), (0.33, 0.01, 0.01, -0.1)], 12, nv2, nv1, 0.02);
        for k in 0..9 { let a = (k as f64 - 4.0) * 0.3; let (x, z) = (a.sin() * 0.12, a.cos() * 0.12 - 0.01); rod(p, [x, 0.0, z], [x * 0.95, 0.2, z * 0.9], 0.012, 0.01, gld, gld2, 2); }
    });
    m("astArm", Mat::Cloth, 0.014, &|p| {
        // une manche large qui s'évase jusqu'au poignet, un revers d'or, une main maigre
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.32, 0.0], 0.06, 0.07, nv1, nv2, 4); rod(p, [0.0, -0.3, 0.0], [0.0, -0.52, 0.03], 0.07, 0.11, nv2, nv1, 4); rod(p, [0.0, -0.5, 0.03], [0.0, -0.54, 0.035], 0.112, 0.112, gld, gld2, 1);
        stars(p, &[[0.06, -0.2, 0.03], [-0.05, -0.42, 0.06]]); egg(p, [0.0, -0.6, 0.04], [0.035, 0.05, 0.035], skn, mix(skn, [0.6, 0.5, 0.45], 0.4), 8);
        for k in 0..4 { let x = (k as f64 - 1.5) * 0.017; rod(p, [x, -0.63, 0.05], [x, -0.7, 0.07], 0.008, 0.007, skn, skn, 2); }
    });
    m("astLeg", Mat::Cloth, 0.014, &|p| {
        // sous la robe, des chausses sombres et des souliers pointus à boucle d'or
        rod(p, [0.0, -0.4, 0.0], [0.0, -0.72, 0.0], 0.045, 0.04, nv3, nv1, 3); crate::mesher::obox(p.q, [0.0, -0.77, 0.04], [[0.045, 0.0, 0.0], [0.0, 0.03, 0.0], [0.0, 0.0, 0.11]], nv3, -1.0, 255.0, 0.0);
        rod(p, [0.0, -0.77, 0.13], [0.0, -0.74, 0.2], 0.03, 0.006, nv3, nv1, 2); p.blob_m([0.0, -0.74, 0.08], [0.02, 0.015, 0.01], gld, gld, Mat::Metal);
    });
    m("astStaff", Mat::Metal, 0.012, &|p| {
        // un bâton d'ébène cerclé d'or, la sphère armillaire au sommet : trois anneaux croisés, l'écliptique penchée, une étoile au cœur
        rod(p, [0.0, -0.75, 0.0], [0.0, 0.85, 0.0], 0.02, 0.016, [0.12, 0.08, 0.06], [0.2, 0.14, 0.1], 8); for y in [-0.4, 0.0, 0.4, 0.82] { rod(p, [0.0, y, 0.0], [0.0, y + 0.03, 0.0], 0.026, 0.026, gld, gld2, 1); }
        let c = [0.0, 1.05, 0.0]; ring(p, c, 0.17, [1.0, 0.0, 0.0], 0.01, gld, 18); ring(p, c, 0.17, [0.0, 0.0, 1.0], 0.01, gld, 18); arc(p, c, 0.17, 0.0, std::f64::consts::TAU, 0.0, 0.01, gld2, gld, 18, 1.0);
        ring(p, c, 0.13, [0.7, 0.0, 0.7], 0.008, gld2, 14); egg(p, c, [0.05, 0.05, 0.05], [1.0, 0.95, 0.6], [1.0, 1.0, 0.85], 8); rod(p, [0.0, 0.85, 0.0], [0.0, 1.25, 0.0], 0.006, 0.006, gld2, gld2, 1);
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.12, 1.12, 0.06], [0.02, 0.02, 0.02], [0.6, 0.7, 0.95], [0.9, 0.95, 1.0], PLAIN); }
    });

    // ---- La Mère-Racine : un grand corps d'écorce tordue, des racines pendues en jupe jusqu'au sol, des chapeaux de champignons en
    // épaulières, de la mousse, des pustules de spores qui luisent ; au creux du poitrail, une graine qui bat. Pour tête, une
    // énorme fleur carnivore : une gueule à dents d'épine sous une corolle pourpre, des étamines lumineuses. Des bras-racines à
    // trois griffes, une massue de ronces.
    let (bk1, bk2, bk3, mos, pet1, pet2, glow, thorn) = ([0.32, 0.22, 0.13], [0.45, 0.32, 0.18], [0.2, 0.13, 0.08], [0.3, 0.48, 0.16], [0.62, 0.1, 0.32], [0.88, 0.3, 0.5], [0.75, 0.95, 0.4], [0.85, 0.8, 0.6]);
    let thorns = |p: &mut Pen, a: V, b: V, n: usize, r: f64| { for k in 0..n { let t = (k as f64 + 0.5) / n as f64; let c = lp(a, b, t); let ang = k as f64 * 2.4; let d = [ang.sin(), 0.3, ang.cos()]; rod(p, c, [c[0] + d[0] * r, c[1] + d[1] * r, c[2] + d[2] * r], 0.012, 0.002, bk3, thorn, 2); } };
    m("mraTorso", Mat::Cloth, 0.016, &|p| {
        // le tronc : cinq brins d'écorce tordus l'un sur l'autre, du bassin aux épaules
        for k in 0..5 { let a = k as f64 * std::f64::consts::TAU / 5.0; let mut prev = [a.sin() * 0.14, -0.1, a.cos() * 0.11];
            for i in 1..=6 { let t = i as f64 / 6.0; let b = a + t * 1.3; let r = 0.14 + 0.08 * (t * std::f64::consts::PI).sin(); let q = [b.sin() * r, -0.1 + t * 0.72, b.cos() * r * 0.8]; rod(p, prev, q, 0.08, 0.075, if k % 2 == 0 { bk1 } else { bk2 }, bk2, 1); prev = q; } }
        // le creux du poitrail et sa graine qui luit ; la mousse ; les pustules de spores
        p.blob_m([0.0, 0.36, 0.13], [0.07, 0.08, 0.04], glow, [1.0, 1.0, 0.7], PLAIN); for k in 0..6 { let a = k as f64 * 1.05; rod(p, [0.0, 0.36, 0.13], [a.sin() * 0.1, 0.36 + a.cos() * 0.11, 0.16], 0.014, 0.008, bk3, bk2, 1); }
        for (x, y, z) in [(-0.15, 0.5, 0.12), (0.18, 0.15, 0.13), (-0.1, 0.05, -0.16), (0.12, 0.55, -0.13)] { egg(p, [x, y, z], [0.07, 0.04, 0.05], mos, mix(mos, glow, 0.3), 6); }
        for (x, y, z) in [(0.16, 0.42, 0.1), (-0.18, 0.25, 0.08), (0.05, 0.6, -0.15), (-0.12, 0.38, -0.17), (0.19, 0.0, -0.08)] { p.blob_m([x, y, z], [0.022, 0.022, 0.022], [0.7, 0.5, 1.0], [0.9, 0.85, 1.0], PLAIN); }
        // les épaulières : deux chapeaux de champignon en console
        for sx in [-1.0f64, 1.0] { egg(p, [sx * 0.27, 0.66, -0.02], [0.17, 0.05, 0.14], [0.55, 0.15, 0.12], [0.75, 0.3, 0.2], 10); p.blob_m([sx * 0.27, 0.63, -0.02], [0.13, 0.015, 0.11], [0.85, 0.75, 0.6], [0.85, 0.75, 0.6], PLAIN);
            for k in 0..4 { let a = k as f64 * 1.57 + 0.4; p.blob_m([sx * 0.27 + a.sin() * 0.09, 0.69, -0.02 + a.cos() * 0.08], [0.018, 0.012, 0.018], [0.95, 0.9, 0.8], [0.95, 0.9, 0.8], PLAIN); } }
        // la jupe de racines pendues jusqu'au sol, qui s'écartent en touchant terre
        for k in 0..11 { let a = k as f64 * std::f64::consts::TAU / 11.0 + 0.2; let (sx, sz) = (a.sin(), a.cos()); let l = 0.6 + 0.15 * ((k * 7 % 5) as f64 / 4.0);
            let (p0, p1, p2) = ([sx * 0.17, -0.08, sz * 0.14], [sx * 0.24, -0.08 - l * 0.6, sz * 0.2], [sx * 0.34, -0.08 - l, sz * 0.3]); rod(p, p0, p1, 0.04, 0.03, bk2, bk1, 2); rod(p, p1, p2, 0.03, 0.012, bk1, bk3, 2); }
        thorns(p, [0.0, 0.0, -0.17], [0.0, 0.6, -0.15], 6, 0.07);
    });
    m("mraHead", Mat::Cloth, 0.014, &|p| {
        // la gueule : un bulbe sombre fendu, des dents d'épine en couronne ; la corolle de huit grands pétales ; les étamines
        egg(p, [0.0, 0.12, 0.04], [0.15, 0.13, 0.15], [0.28, 0.35, 0.12], mix(pet1, [0.2, 0.05, 0.1], 0.5), 10); crate::mesher::obox(p.q, [0.0, 0.11, 0.17], [[0.11, 0.0, 0.0], [0.0, 0.025, 0.0], [0.0, 0.0, 0.03]], [0.12, 0.02, 0.05], -1.0, 255.0, 0.0);
        for k in 0..9 { let x = (k as f64 - 4.0) * 0.025; for sy in [-1.0f64, 1.0] { rod(p, [x, 0.11 + sy * 0.03, 0.17], [x, 0.11 + sy * 0.005, 0.19], 0.009, 0.002, thorn, [1.0, 1.0, 0.95], 1); } }
        for k in 0..8 { let a = k as f64 * std::f64::consts::TAU / 8.0; let (dx, dy) = (a.sin(), a.cos()); slab(p, [dx * 0.12, 0.12 + dy * 0.11, -0.02], [dx * 0.36, 0.12 + dy * 0.33, -0.12], 0.13, 0.05, 0.016, [0.0, 0.0, 1.0], pet1, pet2, 4);
            slab(p, [dx * 0.2, 0.12 + dy * 0.19, -0.04], [dx * 0.3, 0.12 + dy * 0.28, -0.09], 0.02, 0.01, 0.02, [0.0, 0.0, 1.0], [0.98, 0.85, 0.5], [0.98, 0.85, 0.5], 1); }
        for k in 0..6 { let a = k as f64 * 1.05 + 0.3; let tip = [a.sin() * 0.1, 0.12 + a.cos() * 0.1 + 0.18, 0.06]; rod(p, [a.sin() * 0.04, 0.24, 0.02], tip, 0.008, 0.006, [0.5, 0.6, 0.2], [0.6, 0.7, 0.3], 2); p.blob_m(tip, [0.02, 0.02, 0.02], glow, [1.0, 1.0, 0.8], PLAIN); }
        for k in 0..10 { let a = k as f64 * std::f64::consts::TAU / 10.0; slab(p, [a.sin() * 0.12, -0.02, a.cos() * 0.12], [a.sin() * 0.2, -0.08, a.cos() * 0.2], 0.08, 0.02, 0.012, [a.sin(), 0.5, a.cos()], mos, [0.2, 0.35, 0.1], 2); }
    });
    m("mraArm", Mat::Cloth, 0.014, &|p| {
        // un bras-racine noueux, gainé de mousse au coude, des épines ; trois griffes de racine
        rod(p, [0.0, 0.0, 0.0], [0.02, -0.34, 0.02], 0.07, 0.055, bk2, bk1, 4); p.blob_m([0.02, -0.35, 0.02], [0.07, 0.06, 0.07], bk1, mos, PLAIN); rod(p, [0.02, -0.36, 0.02], [0.0, -0.6, 0.06], 0.055, 0.045, bk1, bk2, 4);
        egg(p, [0.0, -0.36, -0.02], [0.08, 0.05, 0.07], mos, mix(mos, glow, 0.2), 6); thorns(p, [0.0, -0.05, 0.0], [0.0, -0.55, 0.05], 5, 0.06);
        for k in 0..3 { let x = (k as f64 - 1.0) * 0.04; rod(p, [x * 0.5, -0.6, 0.06], [x, -0.7, 0.1], 0.025, 0.018, bk2, bk1, 2); rod(p, [x, -0.7, 0.1], [x * 1.3, -0.8, 0.16], 0.018, 0.003, bk1, thorn, 2); }
    });
    m("mraLeg", Mat::Cloth, 0.016, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.4, 0.03], 0.085, 0.07, bk2, bk1, 4); p.blob_m([0.0, -0.4, 0.03], [0.07, 0.06, 0.07], bk1, bk2, PLAIN); rod(p, [0.0, -0.4, 0.03], [0.0, -0.74, 0.0], 0.07, 0.06, bk1, bk2, 4);
        for k in 0..5 { let a = (k as f64 - 2.0) * 0.7; rod(p, [0.0, -0.74, 0.0], [a.sin() * 0.17, -0.8, a.cos() * 0.15 + 0.03], 0.035, 0.008, bk2, bk3, 2); }
    });
    m("mraClub", Mat::Cloth, 0.014, &|p| {
        // une massue de ronces : une branche tordue qui s'épaissit, hérissée d'épines, une gousse de spores au bout
        let mut prev = [0.0, -0.5, 0.0]; for i in 1..=8 { let t = i as f64 / 8.0; let q = [(t * 4.0).sin() * 0.04, -0.5 + t * 1.25, (t * 3.0).cos() * 0.03 - 0.03]; rod(p, prev, q, 0.025 + 0.03 * t, 0.025 + 0.035 * t, bk1, bk2, 1); thorns(p, prev, q, 2, 0.06 + 0.04 * t); prev = q; }
        egg(p, [prev[0], prev[1] + 0.08, prev[2]], [0.08, 0.11, 0.08], [0.45, 0.3, 0.5], [0.7, 0.5, 1.0], 8); for k in 0..5 { let a = k as f64 * 1.26; p.blob_m([prev[0] + a.sin() * 0.075, prev[1] + 0.08 + a.cos() * 0.05, prev[2] + 0.05], [0.02, 0.02, 0.02], [0.75, 0.6, 1.0], [1.0, 0.95, 1.0], PLAIN); }
    });

    // ---- Le Capitaine : un spectre de capitaine. Une longue redingote bleu nuit à basques, des revers et des boutons d'or, une
    // ceinture et son baudrier ; un tricorne à plume ; un visage pâle et verdâtre, des orbites qui luisent, une barbe où pend du
    // varech ; des bottes à revers. Au poing, un sabre d'abordage courbe, sa garde en coquille.
    let (cot1, cot2, gol, gol2, ghost, ghost2, glw, hat, kelp, lea) = ([0.1, 0.13, 0.25], [0.16, 0.2, 0.36], [0.92, 0.75, 0.32], [0.65, 0.5, 0.2], [0.62, 0.8, 0.76], [0.42, 0.58, 0.56], [0.55, 1.0, 0.9], [0.08, 0.08, 0.1], [0.25, 0.45, 0.2], [0.24, 0.16, 0.1]);
    m("cptTorso", Mat::Cloth, 0.014, &|p| {
        // le buste de la redingote, ses revers d'or, la double rangée de boutons ; la ceinture, le baudrier ; les basques jusqu'aux genoux
        shell(p, [0.0, 0.0, 0.0], &[(-0.05, 0.17, 0.13, 0.0), (0.2, 0.19, 0.14, 0.0), (0.45, 0.21, 0.14, 0.0), (0.6, 0.18, 0.12, 0.0), (0.68, 0.07, 0.07, 0.0)], 12, cot1, cot2, 0.025);
        for sx in [-1.0f64, 1.0] { slab(p, [sx * 0.04, 0.62, 0.11], [sx * 0.11, 0.2, 0.15], 0.06, 0.03, 0.012, [sx * 0.3, 0.0, 1.0], gol, gol2, 4); for k in 0..5 { p.blob_m([sx * 0.07, 0.52 - k as f64 * 0.075, 0.155], [0.016, 0.016, 0.01], gol, [1.0, 0.95, 0.7], Mat::Metal); } }
        arc(p, [0.0, 0.0, 0.0], 0.175, 0.0, std::f64::consts::TAU, 0.06, 0.03, lea, lea, 12, 0.82); crate::mesher::obox(p.q, [0.0, 0.06, 0.15], [[0.04, 0.0, 0.0], [0.0, 0.035, 0.0], [0.0, 0.0, 0.01]], gol, -1.0, 255.0, 0.0);
        slab(p, [0.17, 0.6, 0.06], [-0.15, 0.04, 0.12], 0.05, 0.05, 0.012, [0.0, 0.0, 1.0], lea, lea, 6);
        for k in 0..8 { let a = (k as f64 - 3.5) * 0.4 + std::f64::consts::PI; slab(p, [a.sin() * 0.17, 0.02, a.cos() * 0.13], [a.sin() * 0.26, -0.5, a.cos() * 0.2], 0.08, 0.11, 0.016, [a.sin(), -0.15, a.cos()], cot2, cot1, 5); }
        for sx in [-1.0f64, 1.0] { slab(p, [sx * 0.15, 0.02, 0.06], [sx * 0.22, -0.45, 0.1], 0.07, 0.1, 0.016, [sx, 0.0, 0.5], cot2, cot1, 5); }
        // les épaulettes d'or à franges ; la lueur spectrale qui suinte du col
        for sx in [-1.0f64, 1.0] { egg(p, [sx * 0.2, 0.6, 0.0], [0.08, 0.03, 0.08], gol, gol2, 8); for k in 0..6 { let z = (k as f64 - 2.5) * 0.025; rod(p, [sx * 0.27, 0.59, z], [sx * 0.29, 0.53, z], 0.006, 0.004, gol, gol2, 1); } }
        p.blob_m([0.0, 0.66, 0.0], [0.07, 0.03, 0.06], glw, [0.9, 1.0, 1.0], PLAIN);
    });
    m("cptHead", Mat::Cloth, 0.012, &|p| {
        // un visage maigre, pâle et verdâtre, des orbites qui luisent ; une barbe noire et ses tresses de varech ; le tricorne à plume
        egg(p, [0.0, 0.12, 0.01], [0.08, 0.1, 0.09], ghost, ghost2, 10); p.blob_m([0.0, 0.11, 0.09], [0.015, 0.025, 0.02], ghost, ghost2, PLAIN);
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.032, 0.14, 0.078], [0.02, 0.014, 0.01], [0.05, 0.08, 0.08], [0.05, 0.08, 0.08], PLAIN); p.blob_m([sx * 0.032, 0.14, 0.085], [0.009, 0.009, 0.006], glw, [1.0, 1.0, 1.0], PLAIN); }
        for k in 0..5 { let x = (k as f64 - 2.0) * 0.024; slab(p, [x * 1.4, 0.07, 0.07], [x, -0.08 - (2.0 - (k as f64 - 2.0).abs()) * 0.02, 0.06], 0.035, 0.012, 0.014, [0.0, 0.0, 1.0], hat, [0.15, 0.15, 0.17], 3); }
        for x in [-0.04, 0.03] { rod(p, [x, 0.0, 0.08], [x + 0.01, -0.16, 0.06], 0.01, 0.006, kelp, mix(kelp, glw, 0.3), 3); }
        // le tricorne : un fond bombé, trois bords relevés, un galon d'or, la plume
        egg(p, [0.0, 0.22, 0.0], [0.1, 0.06, 0.1], hat, [0.14, 0.14, 0.16], 10);
        for k in 0..3 { let a = k as f64 * std::f64::consts::TAU / 3.0; let (dx, dz) = (a.sin(), a.cos()); slab(p, [dx * 0.06 - dz * 0.12, 0.2, dz * 0.06 + dx * 0.12], [dx * 0.06 + dz * 0.12, 0.2, dz * 0.06 - dx * 0.12], 0.07, 0.07, 0.012, [dx, 0.6, dz], hat, [0.14, 0.14, 0.16], 3);
            rod(p, [dx * 0.12 - dz * 0.12, 0.25, dz * 0.12 + dx * 0.12], [dx * 0.12 + dz * 0.12, 0.25, dz * 0.12 - dx * 0.12], 0.008, 0.008, gol, gol, 3); }
        let mut q = [0.08, 0.26, -0.04]; for i in 0..5 { let n = [q[0] + 0.03, q[1] + 0.045 - i as f64 * 0.006, q[2] - 0.035]; rod(p, q, n, 0.022 - i as f64 * 0.003, 0.02 - i as f64 * 0.003, [0.85, 0.85, 0.82], [0.95, 0.95, 0.92], 1); q = n; }
    });
    m("cptArm", Mat::Cloth, 0.014, &|p| {
        // la manche de redingote, son grand parement à boutons d'or, la manchette de dentelle, une main de spectre
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.32, 0.0], 0.058, 0.05, cot1, cot2, 4); rod(p, [0.0, -0.32, 0.0], [0.0, -0.5, 0.03], 0.05, 0.05, cot2, cot1, 3); rod(p, [0.0, -0.42, 0.03], [0.0, -0.52, 0.035], 0.064, 0.066, gol2, gol, 2);
        for y in [-0.45, -0.49] { p.blob_m([0.0, y, 0.095], [0.012, 0.012, 0.008], gol, gol, Mat::Metal); } for k in 0..6 { let a = k as f64 * 1.05; rod(p, [a.sin() * 0.05, -0.52, 0.03 + a.cos() * 0.05], [a.sin() * 0.06, -0.57, 0.035 + a.cos() * 0.06], 0.01, 0.004, [0.92, 0.92, 0.88], [1.0, 1.0, 0.95], 1); }
        egg(p, [0.0, -0.6, 0.04], [0.035, 0.045, 0.035], ghost, ghost2, 8); for k in 0..4 { let x = (k as f64 - 1.5) * 0.016; rod(p, [x, -0.63, 0.05], [x, -0.69, 0.065], 0.008, 0.006, ghost, ghost2, 2); }
    });
    m("cptLeg", Mat::Cloth, 0.014, &|p| {
        // une culotte claire, de hautes bottes à revers
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.36, 0.02], 0.065, 0.055, [0.75, 0.72, 0.62], [0.62, 0.6, 0.52], 3); rod(p, [0.0, -0.34, 0.02], [0.0, -0.74, 0.0], 0.06, 0.055, lea, mix(lea, hat, 0.4), 4);
        rod(p, [0.0, -0.32, 0.02], [0.0, -0.4, 0.02], 0.075, 0.07, mix(lea, [0.5, 0.35, 0.2], 0.5), lea, 2); crate::mesher::obox(p.q, [0.0, -0.78, 0.04], [[0.055, 0.0, 0.0], [0.0, 0.035, 0.0], [0.0, 0.0, 0.11]], lea, -1.0, 255.0, 0.0);
    });
    m("cptSabre", Mat::Metal, 0.012, &|p| {
        // le sabre d'abordage : une lame large et courbe, sa garde en coquille dorée, la poignée de cuir, le pommeau
        rod(p, [0.0, -0.12, 0.0], [0.0, 0.02, 0.0], 0.02, 0.018, lea, lea, 3); p.blob_m([0.0, -0.13, 0.0], [0.025, 0.025, 0.025], gol, gol2, Mat::Metal);
        egg(p, [0.0, 0.04, 0.03], [0.06, 0.025, 0.06], gol, gol2, 8); arc(p, [0.0, 0.0, 0.0], 0.05, 0.0, std::f64::consts::PI, -0.05, 0.008, gol, gol, 6, 1.0);
        let mut q = [0.0, 0.05, 0.0]; for i in 0..8 { let t = (i + 1) as f64 / 8.0; let n = [0.0, 0.05 + t * 0.7, -0.15 * t * t]; slab(p, q, n, 0.045 - t * 0.012, 0.045 - (t + 0.125) * 0.012, 0.006, [1.0, 0.0, 0.0], [0.75, 0.82, 0.85], [0.92, 0.96, 1.0], 1); q = n; }
    });

    // ---- L'Archimage : un vieux mage immense, drapé d'une robe indigo semée d'étoiles d'argent qui tombe jusqu'aux pieds ; une ceinture
    // d'or à boucle de cristal ; un grand col relevé ; une longue barbe blanche qui descend sur la poitrine ; un chapeau pointu à
    // larges bords, cerclé d'une bande d'étoiles ; des manches évasées. Au poing, un bâton noueux couronné d'un cristal qui luit.
    let (rb, rb2, sil, gdm, skm, brd, hat, crys) = ([0.2, 0.17, 0.46], [0.12, 0.1, 0.3], [0.85, 0.88, 0.95], [0.92, 0.75, 0.3], [0.82, 0.68, 0.58], [0.93, 0.93, 0.9], [0.16, 0.13, 0.38], [0.65, 0.5, 1.0]);
    m("magTorso", Mat::Cloth, 0.016, &|p| {
        // la robe : un cône qui s'évase jusqu'aux chevilles (elle cache les jambes), le col relevé derrière la tête
        shell(p, [0.0, 0.0, 0.0], &[(-0.82, 0.3, 0.26, 0.0), (-0.5, 0.25, 0.21, 0.0), (-0.1, 0.18, 0.14, 0.0), (0.2, 0.19, 0.13, 0.0), (0.45, 0.22, 0.14, 0.0), (0.62, 0.21, 0.13, 0.0), (0.7, 0.1, 0.08, 0.0)], 14, rb2, rb, 0.03);
        for k in 0..26 { let a = k as f64 * 2.39996; let y = -0.78 + (k as f64 / 26.0) * 1.35; let r = 0.3 - (y + 0.82) * 0.1; let rr = r.max(0.15) + 0.005; p.blob_m([a.sin() * rr, y, a.cos() * rr * 0.85], [0.016, 0.016, 0.008], sil, [1.0, 1.0, 1.0], PLAIN); }
        arc(p, [0.0, 0.0, 0.0], 0.19, 0.0, std::f64::consts::TAU, 0.04, 0.035, gdm, [0.7, 0.55, 0.2], 14, 0.65); p.blob_m([0.0, 0.0, 0.145], [0.03, 0.035, 0.02], crys, [0.9, 0.85, 1.0], PLAIN);
        arc(p, [0.0, 0.6, -0.02], 0.2, 1.6, 4.7, 0.0, 0.18, rb, rb2, 12, 0.6);
        // la barbe, qui descend sur la poitrine
        for k in 0..5 { let x = (k as f64 - 2.0) * 0.035; rod(p, [x, 0.66, 0.12], [x * 0.6, 0.3 - (2.0 - (k as f64 - 2.0).abs()) * 0.05, 0.16], 0.03, 0.008, brd, [0.8, 0.8, 0.78], 4); }
    });
    m("magHead", Mat::Human, 0.012, &|p| {
        // le visage ridé, le nez long, des sourcils broussailleux, la barbe ; le chapeau pointu à larges bords, sa bande d'étoiles
        egg(p, [0.0, 0.1, 0.0], [0.08, 0.1, 0.085], skm, [0.7, 0.56, 0.47], 10); rod(p, [0.0, 0.1, 0.07], [0.0, 0.06, 0.11], 0.018, 0.012, skm, skm, 3);
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.032, 0.125, 0.075], [0.012, 0.008, 0.006], [0.15, 0.2, 0.45], [0.3, 0.4, 0.8], PLAIN); slab(p, [sx * 0.015, 0.145, 0.08], [sx * 0.06, 0.15, 0.07], 0.012, 0.01, 0.012, [0.0, 0.0, 1.0], brd, brd, 1); }
        egg(p, [0.0, 0.0, 0.06], [0.08, 0.08, 0.05], brd, [0.82, 0.82, 0.78], 8); egg(p, [0.0, 0.05, 0.09], [0.06, 0.02, 0.03], brd, brd, 6);
        arc(p, [0.0, 0.17, 0.0], 0.0, 0.0, std::f64::consts::TAU, 0.2, 0.012, hat, rb, 18, 1.0);
        for i in 0..7 { let t0 = i as f64 / 7.0; let t1 = (i + 1) as f64 / 7.0; let c = |t: f64| [0.03 * t * t, 0.18 + t * 0.38, -0.06 * t * t]; let r = |t: f64| 0.1 * (1.0 - t) + 0.006; rod(p, c(t0), c(t1), r(t0), r(t1), hat, rb, 1); }
        arc(p, [0.0, 0.2, 0.0], 0.1, 0.0, std::f64::consts::TAU, 0.0, 0.03, gdm, gdm, 14, 1.0); for k in 0..6 { let a = k as f64 * 1.047; p.blob_m([a.sin() * 0.103, 0.215, a.cos() * 0.103], [0.012, 0.012, 0.008], sil, [1.0, 1.0, 1.0], PLAIN); }
    });
    m("magArm", Mat::Cloth, 0.012, &|p| {
        // la manche évasée, la main osseuse qui en sort
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.3, 0.02], 0.06, 0.07, rb, rb2, 5); shell(p, [0.0, -0.3, 0.02], &[(0.0, 0.07, 0.07, 0.0), (-0.2, 0.11, 0.1, 0.0), (-0.26, 0.12, 0.11, 0.0)], 10, rb, rb2, 0.01);
        arc(p, [0.0, -0.55, 0.02], 0.115, 0.0, std::f64::consts::TAU, 0.0, 0.02, sil, sil, 12, 1.0); egg(p, [0.0, -0.6, 0.04], [0.035, 0.045, 0.035], skm, [0.7, 0.56, 0.47], 8);
    });
    m("magLeg", Mat::Cloth, 0.012, &|p| { rod(p, [0.0, -0.4, 0.0], [0.0, -0.72, 0.0], 0.06, 0.06, rb2, rb2, 3); crate::mesher::obox(p.q, [0.0, -0.77, 0.05], [[0.05, 0.0, 0.0], [0.0, 0.025, 0.0], [0.0, 0.0, 0.1]], [0.25, 0.18, 0.12], -1.0, 255.0, 0.0); });
    m("magStaff", Mat::Bone, 0.012, &|p| {
        // le bâton noueux, sa tête en griffe qui serre un cristal violet
        for i in 0..6 { let y0 = -0.4 + i as f64 * 0.22; rod(p, [0.012 * ((i % 2) as f64 - 0.5), y0, 0.0], [0.012 * (((i + 1) % 2) as f64 - 0.5), y0 + 0.22, 0.0], 0.025, 0.022, [0.32, 0.22, 0.14], [0.45, 0.32, 0.2], 3); }
        for k in 0..3 { let a = k as f64 * 2.094; rod(p, [a.sin() * 0.02, 0.9, a.cos() * 0.02], [a.sin() * 0.06, 1.06, a.cos() * 0.06], 0.014, 0.008, [0.32, 0.22, 0.14], [0.45, 0.32, 0.2], 2); }
        p.blob_m([0.0, 1.03, 0.0], [0.05, 0.08, 0.05], crys, [0.95, 0.9, 1.0], PLAIN);
    });

    // ---- Le Gardien chacal : le gardien de la tombe du Pharaon, à tête de chacal noir. De hautes oreilles pointues, un long museau,
    // des yeux d'or ; le némès rayé bleu et or qui retombe sur les épaules ; un large collier ousekh de perles ; un torse sombre et
    // sec ; un pagne blanc plissé, une ceinture d'or ; des bracelets d'or. Au poing, un khépesh : l'épée-faucille de bronze.
    let (jk, jk2, gd, gd2, lap, lin, eye) = ([0.16, 0.15, 0.17], [0.1, 0.09, 0.11], [0.95, 0.76, 0.3], [0.7, 0.5, 0.15], [0.18, 0.32, 0.72], [0.93, 0.9, 0.82], [1.0, 0.82, 0.25]);
    m("jacTorso", Mat::Human, 0.016, &|p| {
        shell(p, [0.0, 0.0, 0.0], &[(-0.04, 0.15, 0.11, 0.0), (0.15, 0.16, 0.11, 0.0), (0.38, 0.2, 0.12, 0.0), (0.56, 0.22, 0.12, 0.0), (0.66, 0.1, 0.08, 0.0)], 12, jk2, jk, 0.03);
        // le collier ousekh : des rangs de perles bleues et d'or en demi-cercle sur la poitrine
        for (k, col) in [(0, gd), (1, lap), (2, gd), (3, lap)].into_iter() { let r = 0.12 + k as f64 * 0.035; for i in 0..11 { let a = -1.25 + i as f64 * 0.25; p.blob_m([a.sin() * r, 0.62 - a.cos() * r * 0.75, 0.12 + 0.02 * (1.0 - a.abs())], [0.02, 0.016, 0.012], col, mix(col, [1.0, 1.0, 1.0], 0.3), if col == gd { Mat::Metal } else { PLAIN }); } }
        arc(p, [0.0, 0.0, 0.0], 0.17, 0.0, std::f64::consts::TAU, 0.05, 0.04, gd2, gd, 14, 0.75);
        slab(p, [0.0, 0.0, 0.12], [0.0, -0.34, 0.15], 0.16, 0.2, 0.015, [0.0, 0.0, 1.0], lin, mix(lin, [0.6, 0.58, 0.5], 0.5), 5); slab(p, [0.0, 0.0, -0.12], [0.0, -0.3, -0.13], 0.18, 0.2, 0.015, [0.0, 0.0, -1.0], lin, mix(lin, [0.6, 0.58, 0.5], 0.5), 5);
        slab(p, [0.0, 0.03, 0.14], [0.0, -0.3, 0.16], 0.04, 0.05, 0.01, [0.0, 0.0, 1.0], gd, lap, 3);
    });
    m("jacHead", Mat::Human, 0.012, &|p| {
        // la tête de chacal : un crâne fin, le long museau, la truffe, les hautes oreilles ; le némès rayé derrière
        egg(p, [0.0, 0.1, -0.01], [0.085, 0.09, 0.1], jk2, jk, 10); rod(p, [0.0, 0.08, 0.06], [0.0, 0.05, 0.22], 0.05, 0.025, jk, jk2, 6); egg(p, [0.0, 0.055, 0.225], [0.018, 0.015, 0.015], [0.03, 0.03, 0.03], [0.05, 0.05, 0.05], 6);
        for sx in [-1.0f64, 1.0] { slab(p, [sx * 0.05, 0.15, -0.02], [sx * 0.07, 0.34, -0.04], 0.045, 0.006, 0.015, [0.0, 0.0, 1.0], jk, [0.5, 0.25, 0.2], 3); p.blob_m([sx * 0.045, 0.115, 0.075], [0.014, 0.008, 0.008], eye, [1.0, 1.0, 0.7], PLAIN); }
        for k in 0..7 { let c = if k % 2 == 0 { gd } else { lap }; let y = 0.16 - k as f64 * 0.045; slab(p, [-0.11, y, -0.06], [0.11, y, -0.06], 0.02, 0.02, 0.05, [0.0, 0.0, -1.0], c, mix(c, [0.0, 0.0, 0.0], 0.2), 4); for sx in [-1.0f64, 1.0] { slab(p, [sx * 0.1, y, 0.0], [sx * 0.1, y, -0.1], 0.02, 0.02, 0.015, [sx, 0.0, 0.0], c, c, 2); } }
    });
    m("jacArm", Mat::Human, 0.012, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.3, 0.02], 0.06, 0.05, jk, jk2, 4); for y in [-0.08, -0.4, -0.5] { arc(p, [0.0, y, 0.02], 0.058, 0.0, std::f64::consts::TAU, 0.0, 0.02, gd, gd2, 10, 1.0); }
        rod(p, [0.0, -0.3, 0.02], [0.0, -0.55, 0.04], 0.05, 0.045, jk2, jk, 4); egg(p, [0.0, -0.6, 0.05], [0.04, 0.045, 0.04], jk, jk2, 8);
    });
    m("jacLeg", Mat::Human, 0.012, &|p| {
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.4, 0.02], 0.07, 0.055, jk, jk2, 4); rod(p, [0.0, -0.4, 0.02], [0.0, -0.74, 0.0], 0.05, 0.04, jk2, jk, 3); arc(p, [0.0, -0.7, 0.0], 0.045, 0.0, std::f64::consts::TAU, 0.0, 0.02, gd, gd2, 10, 1.0);
        crate::mesher::obox(p.q, [0.0, -0.78, 0.04], [[0.05, 0.0, 0.0], [0.0, 0.02, 0.0], [0.0, 0.0, 0.1]], [0.6, 0.45, 0.25], -1.0, 255.0, 0.0);
    });
    m("jacKhopesh", Mat::Metal, 0.012, &|p| {
        // le khépesh : une poignée noire, une lame droite puis recourbée en faucille, de bronze doré
        rod(p, [0.0, -0.14, 0.0], [0.0, 0.02, 0.0], 0.02, 0.02, [0.12, 0.1, 0.1], [0.2, 0.18, 0.16], 3); egg(p, [0.0, -0.15, 0.0], [0.025, 0.02, 0.025], gd, gd2, 6);
        slab(p, [0.0, 0.02, 0.0], [0.0, 0.32, 0.0], 0.03, 0.03, 0.01, [0.0, 0.0, 1.0], [0.78, 0.6, 0.3], [0.9, 0.72, 0.4], 2);
        for i in 0..8 { let (a0, a1) = (i as f64 / 8.0 * 2.4, (i + 1) as f64 / 8.0 * 2.4); let c = |a: f64| [0.0, 0.32 + a.sin() * 0.16, 0.16 - a.cos() * 0.16]; slab(p, c(a0), c(a1), 0.035 - i as f64 * 0.003, 0.032 - i as f64 * 0.003, 0.01, [1.0, 0.0, 0.0], [0.82, 0.64, 0.3], [0.95, 0.78, 0.45], 1); }
    });

    // ---- Le Champion de l'arène : un gladiateur colossal. Un casque de bronze à grande crête de crin rouge et à visière grillagée ;
    // le torse nu, hâlé, couturé de cicatrices, barré d'un baudrier de cuir ; un pagne et une large ceinture de bronze ; aux bras, des
    // manches de cuir lacées, une épaulière de bronze ; des jambières de bronze, des sandales lacées. Au poing, un glaive lourd.
    let (skn, skn2, brz, brz2, lth, lth2, crest, scar) = ([0.72, 0.5, 0.36], [0.6, 0.4, 0.28], [0.8, 0.58, 0.28], [0.55, 0.38, 0.17], [0.42, 0.27, 0.15], [0.3, 0.19, 0.1], [0.75, 0.12, 0.1], [0.82, 0.6, 0.5]);
    m("chpTorso", Mat::Human, 0.016, &|p| {
        // le torse nu, les pectoraux, les abdominaux, quelques cicatrices ; le baudrier ; la ceinture de bronze ; le pagne
        shell(p, [0.0, 0.0, 0.0], &[(-0.04, 0.17, 0.13, 0.0), (0.15, 0.18, 0.13, 0.0), (0.38, 0.23, 0.15, 0.01), (0.56, 0.25, 0.14, 0.0), (0.66, 0.12, 0.09, 0.0)], 12, skn2, skn, 0.03);
        for sx in [-1.0f64, 1.0] { egg(p, [sx * 0.09, 0.47, 0.11], [0.09, 0.06, 0.04], skn, skn2, 8); for k in 0..2 { egg(p, [sx * 0.045, 0.3 - k as f64 * 0.08, 0.13], [0.04, 0.035, 0.02], skn, skn2, 6); } }
        for (a, b) in [([-0.12, 0.5, 0.15], [0.02, 0.4, 0.16]), ([0.06, 0.28, 0.14], [0.14, 0.22, 0.13])] { rod(p, a, b, 0.006, 0.006, scar, scar, 2); }
        for z in [0.16, -0.15] { slab(p, [0.17, 0.6, z * 0.85], [-0.17, 0.08, z], 0.07, 0.07, 0.02, [0.0, 0.0, z.signum()], lth, lth2, 6); }
        arc(p, [0.0, 0.0, 0.0], 0.185, 0.0, std::f64::consts::TAU, 0.06, 0.05, brz2, brz, 14, 0.75); for k in 0..5 { p.blob_m([(k as f64 - 2.0) * 0.05, 0.06, 0.14], [0.016, 0.03, 0.01], brz, [1.0, 0.85, 0.5], Mat::Metal); }
        slab(p, [0.0, 0.02, 0.14], [0.0, -0.3, 0.16], 0.2, 0.17, 0.015, [0.0, 0.0, 1.0], [0.85, 0.82, 0.72], [0.7, 0.66, 0.55], 4);
        slab(p, [0.0, 0.02, -0.14], [0.0, -0.28, -0.15], 0.24, 0.2, 0.015, [0.0, 0.0, -1.0], [0.85, 0.82, 0.72], [0.7, 0.66, 0.55], 4);
    });
    m("chpHead", Mat::Metal, 0.012, &|p| {
        // le casque de bronze qui couvre toute la tête : un timbre rond, de larges bords, la visière en grille, la grande crête rouge
        egg(p, [0.0, 0.12, 0.0], [0.11, 0.12, 0.11], brz2, brz, 12); arc(p, [0.0, 0.0, 0.0], 0.15, 0.0, std::f64::consts::TAU, 0.06, 0.02, brz, brz2, 16, 1.0);
        crate::mesher::obox(p.q, [0.0, 0.11, 0.105], [[0.07, 0.0, 0.0], [0.0, 0.06, 0.0], [0.0, 0.0, 0.01]], [0.08, 0.06, 0.05], -1.0, 255.0, 0.0);
        for k in 0..4 { let x = (k as f64 - 1.5) * 0.035; rod(p, [x, 0.05, 0.118], [x, 0.17, 0.118], 0.007, 0.007, brz, brz, 1); } for y in [0.08, 0.14] { rod(p, [-0.07, y, 0.118], [0.07, y, 0.118], 0.006, 0.006, brz, brz, 2); }
        for k in 0..9 { let t = k as f64 / 8.0; let a = -1.2 + t * 2.4; let (z, y) = (a.sin() * 0.13, 0.22 + a.cos() * 0.08); slab(p, [0.0, y - 0.02, z], [0.0, y + 0.07 + 0.03 * (1.0 - (t - 0.5).abs() * 2.0), z - 0.01], 0.04, 0.02, 0.05, [1.0, 0.0, 0.0], crest, [0.95, 0.3, 0.2], 1); }
    });
    m("chpArm", Mat::Human, 0.014, &|p| {
        // l'épaulière de bronze, le bras nu et musclé, la manche de cuir lacée de l'avant-bras, un poing bandé
        egg(p, [0.0, 0.02, 0.0], [0.1, 0.06, 0.1], brz, brz2, 10); for k in 0..3 { arc(p, [0.0, -0.04 - k as f64 * 0.035, 0.0], 0.1 - k as f64 * 0.005, 0.0, std::f64::consts::TAU, 0.0, 0.012, brz2, brz, 10, 1.0); }
        rod(p, [0.0, -0.02, 0.0], [0.0, -0.3, 0.02], 0.07, 0.06, skn, skn2, 4); egg(p, [0.0, -0.15, 0.04], [0.06, 0.08, 0.05], skn, skn2, 6);
        rod(p, [0.0, -0.3, 0.02], [0.0, -0.55, 0.05], 0.062, 0.055, lth2, lth, 4); for k in 0..4 { let y = -0.33 - k as f64 * 0.06; arc(p, [0.0, y, 0.035], 0.064, 0.0, std::f64::consts::TAU, 0.0, 0.006, lth, lth, 8, 1.0); }
        egg(p, [0.0, -0.6, 0.06], [0.045, 0.05, 0.045], [0.85, 0.8, 0.7], [0.7, 0.65, 0.55], 8);
    });
    m("chpLeg", Mat::Human, 0.014, &|p| {
        // la cuisse nue, la jambière de bronze sur le tibia, la sandale lacée
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.38, 0.02], 0.085, 0.065, skn, skn2, 4); egg(p, [0.0, -0.4, 0.04], [0.06, 0.05, 0.06], brz, brz2, 8);
        shell(p, [0.0, -0.4, 0.02], &[(0.0, 0.06, 0.07, 0.0), (-0.18, 0.065, 0.075, 0.0), (-0.34, 0.05, 0.06, 0.0)], 10, brz, brz2, 0.015); rod(p, [0.0, -0.4, -0.02], [0.0, -0.74, -0.01], 0.05, 0.045, skn2, skn, 3);
        crate::mesher::obox(p.q, [0.0, -0.78, 0.04], [[0.055, 0.0, 0.0], [0.0, 0.02, 0.0], [0.0, 0.0, 0.11]], lth2, -1.0, 255.0, 0.0); for k in 0..3 { arc(p, [0.0, -0.68 - k as f64 * 0.03, -0.01], 0.05, 0.0, std::f64::consts::TAU, 0.0, 0.006, lth, lth, 8, 1.0); }
    });
    m("chpSword", Mat::Metal, 0.012, &|p| {
        // le glaive lourd : une lame large et droite à pointe en feuille, la garde de bronze, la poignée de bois, le pommeau rond
        rod(p, [0.0, -0.13, 0.0], [0.0, 0.0, 0.0], 0.022, 0.02, [0.45, 0.3, 0.18], [0.35, 0.22, 0.12], 3); egg(p, [0.0, -0.15, 0.0], [0.035, 0.03, 0.035], brz, brz2, 8);
        crate::mesher::obox(p.q, [0.0, 0.015, 0.0], [[0.07, 0.0, 0.0], [0.0, 0.02, 0.0], [0.0, 0.0, 0.03]], brz, -1.0, 255.0, 0.0);
        for i in 0..8 { let t0 = i as f64 / 8.0; let t1 = (i + 1) as f64 / 8.0; let wd = |t: f64| if t < 0.8 { 0.065 + 0.012 * (t * 3.0).sin() } else { 0.065 * (1.0 - t) / 0.2 + 0.004 }; slab(p, [0.0, 0.03 + t0 * 0.75, 0.0], [0.0, 0.03 + t1 * 0.75, 0.0], wd(t0), wd(t1), 0.012, [0.0, 0.0, 1.0], [0.78, 0.8, 0.82], [0.92, 0.94, 0.96], 1); }
    });

    // ---- Le Golem de minerai : le gardien des rois nains, taillé dans le cœur de la montagne. Un torse énorme de roche brute, fait de
    // blocs empilés ; des veines de fer rouille et de cuivre vert-de-gris ; dans la poitrine, une fente où brûle un cœur de cristal ;
    // une tête basse et carrée enfoncée entre les épaules, deux yeux de braise ; des runes naines gravées, qui luisent ; des
    // poings comme des rochers, et au poing droit une masse : un manche de fer cerclé, une tête de minerai.
    let (rk1, rk2, rk3, fe, cu, gem, gem2, ember, rune) = ([0.66, 0.63, 0.58], [0.5, 0.48, 0.45], [0.78, 0.75, 0.69], [0.66, 0.42, 0.3], [0.32, 0.62, 0.52], [0.35, 0.75, 1.0], [0.8, 0.95, 1.0], [1.0, 0.62, 0.2], [0.45, 0.85, 1.0]);
    let boulder = |p: &mut Pen, c: V, r: V, k: usize| { let h = |i: usize| ((k * 7 + i * 13) % 11) as f64 / 11.0; let col = mix(rk1, if h(0) < 0.5 { rk2 } else { rk3 }, 0.5 * h(1)); egg(p, c, r, col, mix(col, rk2, 0.4), 6); };
    m("gmiTorso", Mat::Bone, 0.02, &|p| {
        // le tronc : une coque trapue, plus large aux épaules ; des blocs de roche en saillie partout
        shell(p, [0.0, 0.0, 0.0], &[(-0.05, 0.2, 0.16, 0.0), (0.15, 0.25, 0.19, 0.0), (0.4, 0.33, 0.23, 0.0), (0.6, 0.35, 0.22, -0.02), (0.72, 0.18, 0.13, 0.0)], 10, rk2, rk1, 0.05);
        for (k, (c, r)) in [([0.0, 0.62, -0.08], [0.24, 0.1, 0.16]), ([0.22, 0.62, -0.02], [0.13, 0.1, 0.14]), ([-0.22, 0.62, -0.02], [0.13, 0.1, 0.14]), ([0.14, 0.38, 0.14], [0.13, 0.12, 0.08]), ([-0.15, 0.36, 0.14], [0.12, 0.13, 0.08]),
            ([0.0, 0.12, 0.13], [0.16, 0.09, 0.07]), ([0.0, 0.42, -0.2], [0.22, 0.2, 0.08]), ([0.2, 0.2, -0.12], [0.1, 0.12, 0.08]), ([-0.2, 0.22, -0.12], [0.1, 0.12, 0.08])].into_iter().enumerate() { boulder(p, c, r, k); }
        // des mousses de lichen, les veines de fer et de cuivre
        for (c, col) in [([0.25, 0.47, 0.13], fe), ([-0.07, 0.2, 0.18], fe), ([0.28, 0.3, -0.05], cu), ([-0.28, 0.5, 0.05], cu), ([0.08, 0.66, 0.1], fe), ([-0.18, 0.1, 0.12], cu)] { p.blob_m(c, [0.045, 0.035, 0.03], col, mix(col, [1.0, 1.0, 1.0], 0.25), Mat::Metal); p.blob_m([c[0] * 1.05, c[1] - 0.05, c[2]], [0.025, 0.02, 0.02], col, col, Mat::Metal); }
        // la fente de la poitrine et son cœur de cristal qui brûle
        slab(p, [0.0, 0.58, 0.2], [0.0, 0.28, 0.21], 0.13, 0.08, 0.02, [0.0, 0.0, 1.0], [0.08, 0.07, 0.08], [0.08, 0.07, 0.08], 3);
        for (c, r) in [([0.0, 0.45, 0.215], [0.06, 0.1, 0.04]), ([0.03, 0.34, 0.21], [0.035, 0.05, 0.03]), ([-0.03, 0.53, 0.21], [0.03, 0.045, 0.03])] { p.blob_m(c, r, gem, gem2, PLAIN); }
        // une ceinture de fer à rivets, les runes du dos
        arc(p, [0.0, 0.0, 0.0], 0.215, 0.0, std::f64::consts::TAU, 0.06, 0.035, [0.25, 0.24, 0.26], [0.36, 0.34, 0.36], 14, 0.85);
        for k in 0..6 { let a = k as f64 * 1.047; p.blob_m([a.sin() * 0.22, 0.06, a.cos() * 0.19], [0.016, 0.016, 0.016], [0.55, 0.55, 0.58], [0.7, 0.7, 0.72], Mat::Metal); }
        for (x, y) in [(-0.08, 0.5), (0.0, 0.5), (0.08, 0.5), (0.0, 0.42), (0.0, 0.34), (-0.08, 0.26), (0.08, 0.26)] { p.blob_m([x, y, -0.29], [0.022, 0.022, 0.012], rune, [1.0, 1.0, 1.0], PLAIN); }
    });
    m("gmiHead", Mat::Bone, 0.018, &|p| {
        // une tête basse et carrée, un front proéminent ; deux yeux de braise sous l'arcade ; une barbe de stalactites
        boulder(p, [0.0, 0.08, 0.02], [0.13, 0.11, 0.12], 3); boulder(p, [0.0, 0.16, 0.06], [0.14, 0.05, 0.08], 5);
        for sx in [-1.0f64, 1.0] { p.blob_m([sx * 0.05, 0.1, 0.13], [0.03, 0.018, 0.012], [0.1, 0.05, 0.02], [0.1, 0.05, 0.02], PLAIN); p.blob_m([sx * 0.05, 0.1, 0.138], [0.02, 0.012, 0.008], ember, [1.0, 0.95, 0.7], PLAIN); }
        for k in 0..5 { let x = (k as f64 - 2.0) * 0.045; let l = 0.08 + (2.0 - (k as f64 - 2.0).abs()) * 0.04; rod(p, [x, 0.0, 0.1], [x * 0.8, -l, 0.12], 0.03, 0.006, rk1, rk2, 3); }
        // une couronne de cristaux bleus, comme poussés dans la pierre
        for (k, (x, z, h)) in [(-0.08, 0.0, 0.1), (0.0, -0.02, 0.15), (0.08, 0.0, 0.11), (-0.04, -0.07, 0.08), (0.05, -0.07, 0.09)].into_iter().enumerate() { let tip = [x * 1.4, 0.18 + h, z - 0.02]; rod(p, [x, 0.17, z], tip, 0.03 - k as f64 * 0.002, 0.006, gem, gem2, 3); }
    });
    m("gmiArm", Mat::Bone, 0.02, &|p| {
        // l'épaule : un gros bloc ; le bras, fait de pierres empilées ; un poing comme un rocher, ses phalanges de cuivre
        boulder(p, [0.0, 0.02, 0.0], [0.14, 0.12, 0.13], 1); rod(p, [0.0, -0.05, 0.0], [0.0, -0.32, 0.02], 0.1, 0.085, rk2, rk1, 3); boulder(p, [0.0, -0.33, 0.02], [0.1, 0.07, 0.1], 7);
        rod(p, [0.0, -0.34, 0.02], [0.0, -0.52, 0.05], 0.09, 0.1, rk1, rk2, 3); boulder(p, [0.0, -0.6, 0.06], [0.12, 0.1, 0.12], 9);
        for k in 0..4 { let x = (k as f64 - 1.5) * 0.05; p.blob_m([x, -0.62, 0.17], [0.03, 0.03, 0.025], cu, mix(cu, fe, 0.4), Mat::Metal); }
        p.blob_m([0.09, -0.2, 0.05], [0.03, 0.04, 0.03], fe, fe, Mat::Metal); for y in [-0.12, -0.24] { p.blob_m([0.0, y, -0.1], [0.018, 0.018, 0.01], rune, [1.0, 1.0, 1.0], PLAIN); }
    });
    m("gmiLeg", Mat::Bone, 0.02, &|p| {
        // des jambes courtes comme des piliers, de larges pieds plats
        rod(p, [0.0, 0.0, 0.0], [0.0, -0.42, 0.02], 0.12, 0.11, rk2, rk1, 3); boulder(p, [0.0, -0.42, 0.04], [0.11, 0.08, 0.11], 4); rod(p, [0.0, -0.44, 0.03], [0.0, -0.72, 0.0], 0.11, 0.12, rk1, rk2, 3);
        crate::mesher::obox(p.q, [0.0, -0.78, 0.05], [[0.13, 0.0, 0.0], [0.0, 0.06, 0.0], [0.0, 0.0, 0.17]], rk2, -1.0, 255.0, 0.0); p.blob_m([0.06, -0.6, 0.1], [0.03, 0.03, 0.02], cu, cu, Mat::Metal);
    });
    m("gmiMaul", Mat::Metal, 0.014, &|p| {
        // la masse des rois nains : un manche de fer cerclé de cuivre, une tête de minerai brut serrée de deux bandes de fer
        rod(p, [0.0, -0.15, 0.0], [0.0, 0.85, 0.0], 0.03, 0.03, [0.3, 0.29, 0.3], [0.4, 0.39, 0.4], 4); for y in [0.0, 0.3, 0.6] { rod(p, [0.0, y, 0.0], [0.0, y + 0.04, 0.0], 0.04, 0.04, cu, cu, 1); }
        boulder(p, [0.0, 0.95, 0.0], [0.2, 0.15, 0.15], 2); for (c, col) in [([0.12, 1.0, 0.1], fe), ([-0.1, 0.9, 0.11], cu), ([0.0, 1.06, -0.1], gem)] { p.blob_m(c, [0.04, 0.035, 0.03], col, mix(col, [1.0, 1.0, 1.0], 0.3), if col == gem { PLAIN } else { Mat::Metal }); }
        for x in [-0.08, 0.08] { crate::mesher::obox(p.q, [x, 0.95, 0.0], [[0.02, 0.0, 0.0], [0.0, 0.16, 0.0], [0.0, 0.0, 0.16]], [0.3, 0.29, 0.3], -1.0, 255.0, 0.0); }
    });

    // Noyé : la peau turquoise des fonds, des yeux qui luisent, la tête et les épaules coiffées d'algues qui dégoulinent,
    // une tunique brune trouée et effilochée, des bernacles ; il tient un trident.
    let (teal, teal2, teal3) = ([0.3, 0.68, 0.68], [0.16, 0.44, 0.48], [0.42, 0.82, 0.8]); let (alg, alg2, rag, rag2) = ([0.36, 0.5, 0.14], [0.2, 0.34, 0.12], [0.44, 0.25, 0.14], [0.28, 0.16, 0.1]);
    let weed = |p: &mut Pen, c: [f64; 3], r: [f64; 3], drips: &[(f64, f64, f64)]| { p.blob_m(c, r, alg, alg2, Mat::Wool); for &(dx, dz, l) in drips { p.bm([c[0] + dx - 0.02, c[1] - l, c[2] + dz - 0.015, c[0] + dx + 0.02, c[1], c[2] + dz + 0.015], if l > 0.15 { alg2 } else { alg }, Mat::Wool); } };
    m("noyTorso", Mat::Flesh, 0.026, &|p| {
        p.column_m(&[(0.0, 0.18, 0.12), (0.25, 0.2, 0.13), (0.5, 0.25, 0.14), (0.6, 0.18, 0.11)], 0.0, teal2, teal, 0.7, Mat::Flesh);
        // la tunique : devant et dos, trouée (la peau paraît au travers), l'ourlet en lambeaux
        for z in [0.115, -0.135] { p.bm([-0.2, 0.04, z, 0.2, 0.52, z + 0.02], rag, Mat::Cloth); }
        for sx in [-1.0f64, 1.0] { p.bm([sx * 0.2 - 0.015, 0.06, -0.12, sx * 0.2 + 0.015, 0.4, 0.12], rag2, Mat::Cloth); p.bm([sx * 0.15 - 0.045, 0.5, -0.12, sx * 0.15 + 0.045, 0.6, 0.13], rag, Mat::Cloth); }
        for (x, y, w, h) in [(-0.14, 0.34, 0.09, 0.08), (0.04, 0.2, 0.1, 0.07), (-0.05, 0.42, 0.07, 0.05), (0.09, 0.4, 0.06, 0.07), (-0.16, 0.12, 0.07, 0.06)] { p.bm([x, y, 0.13, x + w, y + h, 0.14], teal, Mat::Flesh); }
        for (x, l) in [(-0.2, 0.14), (-0.13, 0.3), (-0.06, 0.18), (0.01, 0.34), (0.08, 0.2), (0.14, 0.28)] { for z in [0.115, -0.135] { p.bm([x, 0.04 - l * if z > 0.0 { 1.0 } else { 0.8 }, z, x + 0.06, 0.06, z + 0.018], if (x * 40.0) as i32 % 2 == 0 { rag } else { rag2 }, Mat::Cloth); } }
        p.tube_m([-0.2, 0.54, 0.12], [0.16, 0.14, 0.13], [0.03, 0.012], [0.03, 0.012], rag2, rag2, Mat::Leather); // une sangle
        weed(p, [-0.2, 0.6, 0.0], [0.12, 0.05, 0.14], &[(-0.04, 0.12, 0.22), (0.05, 0.13, 0.12), (-0.06, -0.12, 0.26)]); weed(p, [0.21, 0.6, 0.0], [0.11, 0.05, 0.14], &[(0.03, 0.12, 0.16), (-0.03, -0.12, 0.2)]);
        for (x, y) in [(0.12, 0.3), (-0.1, 0.24), (0.15, 0.1)] { p.bm([x, y, 0.136, x + 0.03, y + 0.03, 0.15], [0.82, 0.86, 0.8], PLAIN); } // des bernacles
    });
    m("noyHead", Mat::Flesh, 0.024, &|p| {
        p.column_m(&[(0.0, 0.12, 0.12), (0.08, 0.14, 0.14), (0.24, 0.14, 0.14), (0.3, 0.11, 0.11)], 0.0, teal2, teal, 0.74, Mat::Flesh);
        for sx in [-1.0f64, 1.0] { p.bm([sx * 0.06 - 0.036, 0.13, 0.13, sx * 0.06 + 0.036, 0.19, 0.148], dk(teal2, 0.5), PLAIN); p.bm([sx * 0.06 - 0.024, 0.14, 0.142, sx * 0.06 + 0.024, 0.18, 0.154], cyan, PLAIN); } // les yeux
        p.bm([-0.05, 0.04, 0.13, 0.05, 0.075, 0.146], dk(teal2, 0.4), PLAIN); p.bm([-0.012, 0.085, 0.135, 0.012, 0.125, 0.152], teal3, PLAIN); // la bouche, le nez
        for (x, y) in [(-0.12, 0.08), (0.09, 0.2), (0.1, 0.06)] { p.bm([x, y, 0.132, x + 0.03, y + 0.035, 0.146], teal3, Mat::Flesh); } // des plaques plus pâles
        weed(p, [0.0, 0.3, 0.0], [0.16, 0.06, 0.16], &[(-0.13, 0.12, 0.2), (-0.05, 0.14, 0.1), (0.07, 0.14, 0.16), (0.14, 0.1, 0.24), (-0.15, -0.04, 0.26), (0.15, -0.06, 0.18), (0.0, -0.15, 0.3), (-0.1, -0.13, 0.2)]);
    });
    m("noyArm", Mat::Flesh, 0.024, &|p| {
        p.tube([0.0, 0.0, 0.0], [0.0, -0.3, 0.0], [0.06, 0.06], [0.05, 0.05], teal, teal2); p.tube([0.0, -0.3, 0.0], [0.0, -0.58, 0.02], [0.05, 0.05], [0.04, 0.04], teal2, teal); p.blob([0.0, -0.63, 0.03], [0.05, 0.065, 0.055], teal, teal2);
        p.bm([-0.07, -0.16, -0.07, 0.07, 0.03, 0.07], rag, Mat::Cloth); for (x, l) in [(-0.07, 0.1), (-0.02, 0.18), (0.03, 0.08)] { p.bm([x, -0.16 - l, 0.05, x + 0.04, -0.14, 0.072], rag2, Mat::Cloth); } // la manche déchirée
        p.bm([-0.056, -0.46, -0.056, 0.056, -0.42, 0.056], alg, Mat::Wool); p.bm([0.02, -0.56, 0.03, 0.05, -0.44, 0.058], alg2, Mat::Wool);
    });
    m("noyLeg", Mat::Flesh, 0.026, &|p| {
        p.tube([0.0, 0.0, 0.0], [0.0, -0.42, 0.0], [0.08, 0.08], [0.065, 0.065], teal, teal2); p.tube([0.0, -0.42, 0.0], [0.0, -0.8, 0.0], [0.065, 0.065], [0.05, 0.05], teal2, teal); p.rbox([-0.065, -0.86, -0.07, 0.065, -0.78, 0.14], teal2);
        p.bm([-0.09, -0.3, -0.09, 0.09, 0.02, 0.09], rag2, Mat::Cloth); for (x, l) in [(-0.09, 0.14), (-0.03, 0.26), (0.03, 0.1)] { p.bm([x, -0.3 - l, 0.07, x + 0.055, -0.28, 0.092], rag, Mat::Cloth); } // les braies en loques
        p.bm([-0.07, -0.74, -0.07, 0.07, -0.69, 0.075], alg, Mat::Wool); p.bm([-0.02, -0.82, 0.07, 0.03, -0.7, 0.1], alg2, Mat::Wool);
    });
    // Le trident : une hampe de bois tenue à mi-hauteur, un fer bleu-vert à trois dents barbelées.
    m("noyTrident", Mat::Metal, 0.022, &|p| { let (iron, wood) = ([0.34, 0.74, 0.74], [0.45, 0.3, 0.17]);
        p.tube_m([0.0, -0.75, 0.0], [0.0, 0.62, 0.0], [0.02, 0.02], [0.02, 0.02], dk(wood, 0.8), wood, Mat::Grain); p.bm([-0.03, 0.56, -0.03, 0.03, 0.66, 0.03], iron, Mat::Metal);
        p.bm([-0.15, 0.64, -0.022, 0.15, 0.69, 0.022], iron, Mat::Metal);
        for (x, h) in [(-0.14, 0.3), (0.0, 0.38), (0.14, 0.3)] { p.tube_m([x, 0.66, 0.0], [x, 0.66 + h, 0.0], [0.02, 0.02], [0.006, 0.006], iron, [0.6, 0.95, 0.92], Mat::Metal); p.bm([x - 0.035, 0.66 + h - 0.1, -0.01, x + 0.035, 0.66 + h - 0.07, 0.01], iron, Mat::Metal); }
        p.bm([-0.026, -0.2, -0.026, 0.026, -0.05, 0.026], [0.3, 0.2, 0.12], Mat::Leather); });

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
