use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

pub fn enlarge_image(img: DynamicImage, scale: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // まずx軸方向に拡大する
    let mut horizontal_enlarged_img = ImageBuffer::new(img.width() * scale, img.height());
    for y in 0..img.height() - 1 {
        for x in 0..img.width() - 1 {
            let west = img.get_pixel(x, y);
            let east = img.get_pixel(x + 1, y);

            for dx in 0..scale {
                let pixel = Rgba([
                    calculate_intermediate_number(west[0], east[0], scale as u8, dx as u8),
                    calculate_intermediate_number(west[1], east[1], scale as u8, dx as u8),
                    calculate_intermediate_number(west[2], east[2], scale as u8, dx as u8),
                    calculate_intermediate_number(west[3], east[3], scale as u8, dx as u8),
                ]);
                horizontal_enlarged_img.put_pixel(x * scale + dx, y, pixel);
            }
        }
    }
    // 次にy軸方向に拡大する
    let mut vertical_enlarged_img = ImageBuffer::new(
        horizontal_enlarged_img.width(),
        horizontal_enlarged_img.height() * scale,
    );
    for y in 0..horizontal_enlarged_img.height() - 1 {
        for x in 0..horizontal_enlarged_img.width() - 1 {
            let north = horizontal_enlarged_img.get_pixel(x, y);
            let south = horizontal_enlarged_img.get_pixel(x, y + 1);

            for dy in 0..scale {
                let pixel = Rgba([
                    calculate_intermediate_number(north[0], south[0], scale as u8, dy as u8),
                    calculate_intermediate_number(north[1], south[1], scale as u8, dy as u8),
                    calculate_intermediate_number(north[2], south[2], scale as u8, dy as u8),
                    calculate_intermediate_number(north[3], south[3], scale as u8, dy as u8),
                ]);
                vertical_enlarged_img.put_pixel(x, y * scale + dy, pixel);
            }
        }
    }
    vertical_enlarged_img
}

fn calculate_intermediate_number(left: u8, right: u8, scale: u8, d: u8) -> u8 {
    if left > right {
        right + ((((left - right) as f32) / (scale as f32)) * (d as f32)) as u8
    } else {
        left + ((((right - left) as f32) / (scale as f32)) * (d as f32)) as u8
    }
}
