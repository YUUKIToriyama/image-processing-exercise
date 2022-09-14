use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let scale: u32 = 2;
    let img = image::open("./sample.png").unwrap();
    let (width, height) = img.dimensions();

    let mut larger_img = ImageBuffer::new(width * scale, height * scale);

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let nw_pixel = img.get_pixel(x, y);
            let ne_pixel = img.get_pixel(x + 1, y);
            let sw_pixel = img.get_pixel(x, y + 1);
            let se_pixel = img.get_pixel(x + 1, y + 1);

            larger_img.put_pixel(x * scale, y * scale, nw_pixel);
            larger_img.put_pixel(
                x * scale + 1,
                y * scale,
                calculate_average(nw_pixel, ne_pixel),
            );
            larger_img.put_pixel(
                x * scale,
                y * scale + 1,
                calculate_average(nw_pixel, sw_pixel),
            );
            larger_img.put_pixel(
                x * scale + 1,
                y * scale + 1,
                calculate_average(nw_pixel, se_pixel),
            );
        }
    }

    larger_img.save("./generated.png").unwrap();
}

fn calculate_average(left: Rgba<u8>, right: Rgba<u8>) -> Rgba<u8> {
    let r = left[0] / 2 + right[0] / 2;
    let g = left[1] / 2 + right[1] / 2;
    let b = left[2] / 2 + right[2] / 2;
    let a = left[3] / 2 + right[3] / 2;
    Rgba([r, g, b, a])
}
