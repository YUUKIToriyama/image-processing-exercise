mod affine;
mod scale;

use affine::rotate_image;
use std::f32::consts::PI;

fn main() {
    let img = image::open("./sample.png").unwrap();
    let rotated_img = rotate_image(img, PI / 6.0);
    rotated_img.save("./generated.png").unwrap();
}
