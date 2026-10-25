use trait_boss_shape_renderer::*;
use std::panic;

fn main() {
    println!("== Shape renderer ==");
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rect { x: 1.0, y: 1.0, w: 6.0, h: 3.0 }),
            Box::new(Circle { cx: 14.0, cy: 5.0, r: 4.0 }),
            Box::new(Triangle { x: 2.0, y: 6.0, base: 7.0, height: 5.0 }),
            Box::new(Scaled { inner: Rect { x: 20.0, y: 1.0, w: 2.0, h: 2.0 }, factor: 2.0 }),
        ];
        let canvas = Canvas { w: 26, h: 12 };
        println!("{}", canvas.render(&shapes));
        println!();
        for shape in &shapes {
            println!("{:<12} area {:>6.2}", shape.name(), shape.area());
        }
        let rects = [Rect { x: 0.0, y: 0.0, w: 2.0, h: 3.0 }, Rect { x: 0.0, y: 0.0, w: 4.0, h: 4.0 }];
        println!("Two rects: total area {:.1}", total_area(&rects));
        if let Some(big) = largest(&rects) {
            println!("Largest rect is {}x{}", big.w, big.h);
        }
    });
    if result.is_err() {
        println!("Finish the `Shape` impls, `Canvas::render`, `total_area` and `largest` to see the picture.");
    }
}
