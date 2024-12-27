use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

pub trait ImageFilter {
    fn apply(&self, image: DynamicImage) -> DynamicImage;
}

pub struct GrayscaleFilter;

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

pub struct ColorInversionFilter;

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
