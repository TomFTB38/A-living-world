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


/// Même piège dans le Rust : une instruction collée après un `//` en fin de ligne n'est jamais exécutée.
#[test]
fn pas_d_instruction_rust_avalee_par_un_commentaire() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut bad = Vec::new();
    for sub in ["core/src", "game/src"] {
        for e in std::fs::read_dir(root.join(sub)).unwrap() {
            let p = e.unwrap().path(); if p.extension().and_then(|x| x.to_str()) != Some("rs") { continue; }
            for (k, l) in std::fs::read_to_string(&p).unwrap().lines().enumerate() {
                let Some(i) = l.find("// ") else { continue }; if l[..i].matches('"').count() % 2 == 1 { continue; } // dans une chaîne
                let tail = l[i + 3..].trim_end(); if !tail.ends_with(';') && !tail.ends_with("; }") { continue; }
                // une affectation `a.b = …;`, un `let`, ou un appel de méthode `self.x(…);`
                let w: Vec<&str> = tail.split_whitespace().collect();
                let assign = w.windows(3).any(|t| (t[1] == "=" || t[1] == "+=" || t[1] == "-=") && t[0].contains('.') && t[0].chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '[' || c == ']'));
                let call = tail.contains("self.") && tail.contains("(") && tail.contains(");");
                if assign || tail.contains(" let ") || tail.starts_with("let ") || call { bad.push(format!("{} ligne {} : {}", p.display(), k + 1, tail)); }
            }
        }
    }
    assert!(bad.is_empty(), "du code derrière un commentaire :\n{}", bad.join("\n"));
}
