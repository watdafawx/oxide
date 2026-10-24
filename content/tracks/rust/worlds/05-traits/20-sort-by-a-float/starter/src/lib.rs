#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub name: String,
    pub opacity: f64,
}

/// The layer with the highest opacity, or None when there are no layers.
pub fn most_opaque(layers: &[Layer]) -> Option<&Layer> {
    // TODO: max_by with a closure that compares the two opacities with total_cmp
    layers.first()
}

/// Sort in place, lowest opacity first.
pub fn sort_by_opacity(layers: &mut [Layer]) {
    // TODO: sort_by with a closure that compares the two opacities with total_cmp
    let _ = layers;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer(name: &str, opacity: f64) -> Layer {
        Layer { name: name.to_string(), opacity }
    }

    #[test]
    fn most_opaque_finds_the_top_layer() {
        let layers = [layer("sky", 0.4), layer("ink", 1.0), layer("glow", 0.75)];
        assert_eq!(most_opaque(&layers).map(|l| l.name.as_str()), Some("ink"));
        assert_eq!(most_opaque(&[]), None);
    }

    #[test]
    fn sort_by_opacity_orders_layers() {
        let mut layers = vec![layer("ink", 1.0), layer("sky", 0.4), layer("glow", 0.75)];
        sort_by_opacity(&mut layers);
        let names: Vec<&str> = layers.iter().map(|l| l.name.as_str()).collect();
        assert_eq!(names, vec!["sky", "glow", "ink"]);
    }
}
