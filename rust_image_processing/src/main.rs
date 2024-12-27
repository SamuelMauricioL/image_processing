mod filters;

use filters::{ColorInversionFilter, GrayscaleFilter, ImageFilter};
use image::{DynamicImage, ImageReader};

fn main() {
    let temp_path: &str = "./images/temp";
    let output_path: &str = "./images/filters_applied";
    let image_test_path: String = format!("{}/image_test.png", temp_path);
    let image_grayscale_path: String = format!("{}/gray_scale_image.jpg", output_path);
    let image_inverted_path: String = format!("{}/inverted_image.jpg", output_path);

    let grayscale_filter = GrayscaleFilter;
    let color_inversion_filter = ColorInversionFilter;

    let imagen: DynamicImage = ImageReader::open(image_test_path)
        .unwrap()
        .decode()
        .unwrap();

    let imagen_gray_scale = grayscale_filter.apply(imagen.clone());
    imagen_gray_scale.save(image_grayscale_path).unwrap();

    let inverted_image = color_inversion_filter.apply(imagen);
    inverted_image.save(image_inverted_path).unwrap();
}

#[no_mangle]
pub fn main_rust() {
    main();
}
