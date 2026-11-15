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
