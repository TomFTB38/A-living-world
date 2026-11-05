use alw_core::blocks::mat_layer;
#[test]
fn couches_du_shader() {
    let s = include_str!("../../game/src/shaders/geom.wgsl");
    for (name, mat) in [("STONE0", "stone"), ("MOSSY0", "mossy_stone"), ("GRAVEL0", "gravel"), ("BARK0", "bark_oak"), ("BEECH0", "bark_beech"), ("LEAF0", "leaf_oak")] {
        assert!(s.contains(&format!("{name}: i32 = {};", mat_layer(mat))), "{name}");
    }
}
