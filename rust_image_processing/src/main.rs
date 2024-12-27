use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgb};

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

trait ImageFilter {
    fn apply(&self, image: DynamicImage) -> DynamicImage;
}

struct GrayscaleFilter;

impl ImageFilter for GrayscaleFilter {
    fn apply(&self, image: DynamicImage) -> DynamicImage {
        let (width, height) = image.dimensions();
        let mut new_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let gray =
                    (0.3 * pixel[0] as f32 + 0.59 * pixel[1] as f32 + 0.11 * pixel[2] as f32) as u8;
                new_image.put_pixel(x, y, Rgb([gray, gray, gray]));
            }
        }

        DynamicImage::ImageRgb8(new_image)
    }
}

struct ColorInversionFilter;

impl ImageFilter for ColorInversionFilter {
    fn apply(&self, image: DynamicImage) -> DynamicImage {
        let mut new_image: ImageBuffer<Rgb<u8>, Vec<u8>> = image.to_rgb8();

        for pixel in new_image.pixels_mut() {
            pixel[0] = 255 - pixel[0];
            pixel[1] = 255 - pixel[1];
            pixel[2] = 255 - pixel[2];
        }

        DynamicImage::ImageRgb8(new_image)
    }
}
