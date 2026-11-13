//! Les shaders ne sont compilés que sous Windows : ce garde-fou attrape ici les erreurs d'écriture les plus bêtes,
//! comme un commentaire « // » qui avale la fin d'une ligne et laisse une accolade ouverte.
#[test]
fn accolades_et_parentheses_equilibrees() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../game/src/shaders");
    for e in std::fs::read_dir(&dir).unwrap() {
        let p = e.unwrap().path(); if p.extension().and_then(|x| x.to_str()) != Some("wgsl") { continue; }
        let src = std::fs::read_to_string(&p).unwrap(); let mut code = String::new(); let mut rest = src.as_str();
        // retire /* … */ puis // … jusqu'à la fin de ligne
        while let Some(i) = rest.find("/*") { code.push_str(&rest[..i]); rest = rest[i..].find("*/").map_or("", |j| &rest[i + j + 2..]); } code.push_str(rest);
        let code: String = code.lines().map(|l| l.split("//").next().unwrap_or("")).collect::<Vec<_>>().join("\n");
        let n = |c: char| code.chars().filter(|x| *x == c).count();
        assert_eq!(n('{'), n('}'), "accolades déséquilibrées dans {}", p.display()); assert_eq!(n('('), n(')'), "parenthèses déséquilibrées dans {}", p.display());
    }
}

/// Un « // » au milieu d'une ligne avale tout ce qui suit : si la fin de la ligne ressemble à du code (« let », « var »,
/// « return », un « ; »), c'est presque sûrement une déclaration avalée par mégarde.
#[test]
fn pas_de_code_avale_par_un_commentaire() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../game/src/shaders");
    for e in std::fs::read_dir(&dir).unwrap() {
        let p = e.unwrap().path(); if p.extension().and_then(|x| x.to_str()) != Some("wgsl") { continue; }
        for (k, l) in std::fs::read_to_string(&p).unwrap().lines().enumerate() {
            let Some(i) = l.find("//") else { continue }; let tail = &l[i + 2..];
            let code = [" let ", " var ", "return ", "textureSample", "= vec"].iter().any(|w| tail.contains(w)) && tail.contains(';');
            assert!(!code, "{} ligne {} : du code derrière un commentaire : {}", p.display(), k + 1, tail.trim());
        }
    }
}

