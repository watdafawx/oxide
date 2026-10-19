use trait_boss_shape_renderer::*;
use std::panic;

fn main() {
    println!("== Hitbox renderer ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let hitboxes: Vec<Box<dyn Hitbox>> = vec![
            Box::new(Aabb { x: 1.0, y: 1.0, w: 6.0, h: 3.0 }),
            Box::new(Cylinder { cx: 14.0, cy: 5.0, r: 4.0 }),
            Box::new(Wedge { x: 2.0, y: 6.0, base: 7.0, height: 5.0 }),
            Box::new(Scaled { inner: Aabb { x: 20.0, y: 1.0, w: 2.0, h: 2.0 }, factor: 2.0 }),
        ];
        let canvas = Canvas { w: 26, h: 12 };
        println!("{}", canvas.render(&hitboxes));
        println!();
        for hitbox in &hitboxes {
            println!("{:<12} area {:>6.2}", hitbox.name(), hitbox.area());
        }
        let boxes = [Aabb { x: 0.0, y: 0.0, w: 2.0, h: 3.0 }, Aabb { x: 0.0, y: 0.0, w: 4.0, h: 4.0 }];
        println!("Two boxes: total area {:.1}", total_area(&boxes));
        if let Some(big) = biggest(&boxes) {
            println!("Biggest box is {}x{}", big.w, big.h);
        }
    });
    if result.is_err() {
        println!("Finish the `Hitbox` impls, `Canvas::render`, `total_area` and `biggest` to see the picture.");
    }
}
