use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgb};

fn main() {
    let imagen: DynamicImage = ImageReader::open("./images/image_test.png")
        .unwrap()
        .decode()
        .unwrap();

    let imagen_escala_grises: DynamicImage = apply_grayscale(imagen.clone());
    imagen_escala_grises
        .save("imagen_escala_grises.jpg")
        .unwrap();

    let imagen_invertida: DynamicImage = apply_color_inversion(imagen.clone());
    imagen_invertida.save("imagen_invertida.jpg").unwrap();
}

fn apply_grayscale(imagen: DynamicImage) -> DynamicImage {
    let (ancho, alto) = imagen.dimensions();
    let mut nueva_imagen: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(ancho, alto);

    for y in 0..alto {
        for x in 0..ancho {
            let pixel = imagen.get_pixel(x, y);
            let gris =
                (0.3 * pixel[0] as f32 + 0.59 * pixel[1] as f32 + 0.11 * pixel[2] as f32) as u8;
            nueva_imagen.put_pixel(x, y, Rgb([gris, gris, gris]));
        }
    }

    DynamicImage::ImageRgb8(nueva_imagen)
}

fn apply_color_inversion(imagen: DynamicImage) -> DynamicImage {
    let mut nueva_imagen: ImageBuffer<Rgb<u8>, Vec<u8>> = imagen.to_rgb8();

    for pixel in nueva_imagen.pixels_mut() {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
    }

    DynamicImage::ImageRgb8(nueva_imagen)
}
