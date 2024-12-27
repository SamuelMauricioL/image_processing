use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgb};

fn main() {
    let temp_path: String = String::from("./images/temp");
    let output_path: String = String::from("./images/filters_applied");
    let image_test_path: String = String::from(temp_path + "/image_test.png");
    let image_grayscale_path: String = String::from(output_path.clone() + "/gray_scale_image.jpg");
    let image_inverted_path: String = String::from(output_path.clone() + "/inverted_image.jpg");

    let imagen: DynamicImage = ImageReader::open(image_test_path)
        .unwrap()
        .decode()
        .unwrap();

    let imagen_gray_scale: DynamicImage = apply_grayscale(imagen.clone());
    imagen_gray_scale.save(image_grayscale_path).unwrap();

    let inverted_image: DynamicImage = apply_color_inversion(imagen.clone());
    inverted_image.save(image_inverted_path).unwrap();
}

#[no_mangle]
pub fn main_rust() {
    main();
}

#[no_mangle]
pub fn apply_grayscale(image: DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut new_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let gris =
                (0.3 * pixel[0] as f32 + 0.59 * pixel[1] as f32 + 0.11 * pixel[2] as f32) as u8;
            new_image.put_pixel(x, y, Rgb([gris, gris, gris]));
        }
    }

    DynamicImage::ImageRgb8(new_image)
}

#[no_mangle]
pub fn apply_color_inversion(image: DynamicImage) -> DynamicImage {
    let mut new_image: ImageBuffer<Rgb<u8>, Vec<u8>> = image.to_rgb8();

    for pixel in new_image.pixels_mut() {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
    }

    DynamicImage::ImageRgb8(new_image)
}
