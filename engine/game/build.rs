//! Sous Windows, l'icône et le nom du jeu sont intégrés au programme. Ailleurs, rien à faire.
fn main() {
    println!("cargo:rerun-if-changed=assets/icone.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut r = winresource::WindowsResource::new();
        r.set_icon("assets/icone.ico"); r.set("ProductName", "A Living World"); r.set("FileDescription", "A Living World");
        if let Err(e) = r.compile() { println!("cargo:warning=icône non intégrée : {e}"); }
    }
}
