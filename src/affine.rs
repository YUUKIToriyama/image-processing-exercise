use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use ndarray::prelude::*;

pub fn rotate_image(img: DynamicImage, angle: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = (img.width() as f32 * angle.cos() + img.height() as f32 * angle.sin()) as u32;
    let height = (img.width() as f32 * angle.sin() + img.height() as f32 * angle.cos()) as u32;
    let mut rotated_img = ImageBuffer::new(width, height);

    // 変換後の画像の任意の座標(u,v)に対してアフィン逆変換を行ない、元の画像から色を取得する
    for v in 0..rotated_img.height() - 1 {
        for u in 0..rotated_img.width() - 1 {
            // 逆アフィン変換
            let rotation = array![
                [angle.cos(), -angle.sin()],
                [angle.sin(), angle.cos()]
            ];
            let shift_to_zero = array![
                [(u as i32 - (width / 2) as i32) as f32],
                [(v as i32 - (height / 2) as i32) as f32],
            ];
            let shift_to_center_of_image = array![
                [(img.width() / 2) as f32],
                [(img.height() / 2) as f32],
            ];
            let reverse_affine_transformed = rotation.dot(&shift_to_zero) + shift_to_center_of_image;
            let x = reverse_affine_transformed[[0,0]];
            let y = reverse_affine_transformed[[1,0]];

            // 計算された座標が元の画像の領域内にあるか
            if 0.0 < x && x < (img.width() - 1) as f32 && 0.0 < y && y < (img.height() - 1) as f32 {
                let nw = img.get_pixel(x.floor() as u32, y.floor() as u32);
                let ne = img.get_pixel(x.ceil() as u32, y.floor() as u32);
                let sw = img.get_pixel(x.ceil() as u32, y.ceil() as u32);
                let se = img.get_pixel(x.ceil() as u32, y.ceil() as u32);

                rotated_img.put_pixel(u, v, get_average_color(nw, ne, sw, se));
            } else {
                rotated_img.put_pixel(u, v, Rgba([0, 0, 0, 0]))
            }
        }
    }
    rotated_img
}

fn get_average_color(nw: Rgba<u8>, ne: Rgba<u8>, sw: Rgba<u8>, se: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        nw[0] / 4 + ne[0] / 4 + sw[0] / 4 + se[0] / 4,
        nw[1] / 4 + ne[1] / 4 + sw[1] / 4 + se[1] / 4,
        nw[2] / 4 + ne[2] / 4 + sw[2] / 4 + se[2] / 4,
        nw[3] / 4 + ne[3] / 4 + sw[3] / 4 + se[3] / 4,
    ])
}
