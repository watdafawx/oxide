//! Player cuts: every photo trimmed, centred on a square and resized, the way the HUD wants them.

#![allow(unused_imports)]

use std::path::{Path, PathBuf};

use image::{RgbaImage, imageops};
use rayon::prelude::*;

use crate::*;

/// Centre an image on a transparent square as wide as its longer side.
pub fn square(img: &RgbaImage) -> RgbaImage {
    let side = img.width().max(img.height());
    let mut canvas = RgbaImage::new(side, side);
    imageops::overlay(&mut canvas, img, i64::from((side - img.width()) / 2), i64::from((side - img.height()) / 2));
    canvas
}

/// One player cut: trimmed, squared, `size` pixels wide.
pub fn cut(img: &RgbaImage, size: u32) -> RgbaImage {
    imageops::resize(&square(&trim_transparent(img)), size, size, imageops::FilterType::Triangle)
}

/// Cut every PNG in `input` into `output`, on every core. Gives back how many were written.
pub fn cut_folder(input: &Path, output: &Path, size: u32) -> Result<usize, String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let files: Vec<PathBuf> = std::fs::read_dir(input)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("png")))
        .collect();
    files
        .par_iter()
        .map(|path| {
            let img = image::open(path).map_err(|e| format!("{}: {e}", path.display()))?.to_rgba8();
            let name = path.file_name().ok_or("a file without a name")?;
            cut(&img, size).save(output.join(name)).map_err(|e| e.to_string())
        })
        .collect::<Result<Vec<()>, String>>()
        .map(|done| done.len())
}

/// The box around every pixel that is not fully transparent: (left, top, right, bottom), both edges included.
/// `None` for a fully transparent image.
pub fn visible_box(img: &RgbaImage) -> Option<(u32, u32, u32, u32)> {
    let (mut left, mut top, mut right, mut bottom) = (u32::MAX, u32::MAX, 0, 0);
    for (x, y, pixel) in img.enumerate_pixels() {
        if pixel[3] > 0 {
            left = left.min(x);
            top = top.min(y);
            right = right.max(x);
            bottom = bottom.max(y);
        }
    }
    if left > right {
        return None;
    }
    Some((left, top, right, bottom))
}
