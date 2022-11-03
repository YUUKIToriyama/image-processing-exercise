mod scale;

use scale::enlarge_image;

fn main() {
    let scale: u32 = 3;
    let img = image::open("./sample.png").unwrap();
    let enlarged_img = enlarge_image(img, scale);
    enlarged_img.save("./generated.png").unwrap();
}
