use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};

pub fn rotate_image(img: DynamicImage, angle: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let width = (img.width() as f32 * angle.cos() + img.height() as f32 * angle.sin()) as u32;
    let height = (img.width() as f32 * angle.sin() + img.height() as f32 * angle.cos()) as u32;
    let mut rotated_img = ImageBuffer::new(width, height);

    // 変換後の画像の任意の座標(u,v)に対してアフィン逆変換を行ない、元の画像から色を取得する
    for v in 0..rotated_img.height() - 1 {
        for u in 0..rotated_img.width() - 1 {
            // 逆アフィン変換
            let x = u as f32 * angle.cos() - v as f32 * angle.sin();
            let y = u as f32 * angle.sin() + v as f32 * angle.cos();

            // 計算された座標が元の画像の領域内にあるか
            if 0.0 < x && x < img.width() as f32 && 0.0 < y && y < img.height() as f32 {
                let nw = img.get_pixel(x.floor() as u32, y.floor() as u32);
                let ne = img.get_pixel(x.ceil() as u32, y.floor() as u32);
                let sw = img.get_pixel(x.ceil() as u32, y.ceil() as u32);
                let se = img.get_pixel(x.ceil() as u32, y.ceil() as u32);

                rotated_img.put_pixel(u, v, get_average_color(nw, ne, sw, se));
            } else {
                rotated_img.put_pixel(u, v, Rgba::from_channels(0, 0, 0, 0));
            }
        }
    }
    rotated_img
}

fn get_average_color(nw: Rgba<u8>, ne: Rgba<u8>, sw: Rgba<u8>, se: Rgba<u8>) -> Rgba<u8> {
    Rgba::from_channels(
        (nw[0] + ne[0] + sw[0] + se[0]) / 4,
        (nw[1] + ne[1] + sw[1] + se[1]) / 4,
        (nw[2] + ne[2] + sw[2] + se[2]) / 4,
        (nw[3] + ne[3] + sw[3] + se[3]) / 4,
    )
}
