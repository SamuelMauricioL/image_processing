mod filters;
mod utils;

use filters::{ColorInversionFilter, GrayscaleFilter, ImageFilter};
use image::{DynamicImage, ImageReader};
use utils::list_files_in_folder;

fn main() {
    let temp_path: &str = "./images/temp";

    if let Ok(files) = list_files_in_folder(temp_path) {
        for file_name in files {
            println!("{}", file_name);
            let image_path: String = format!("{}/{}", temp_path, file_name);
            let image: DynamicImage = ImageReader::open(image_path).unwrap().decode().unwrap();

            apply_filters(image, file_name);
        }
    } else {
        eprintln!("Error al obtener la lista de archivos");
    }
}

fn apply_filters(image: DynamicImage, file_name: String) {
    let output_path: &str = "./images/filters_applied";
    let image_grayscale_path: String = format!("{}/gray_scale_{}", output_path, file_name);
    let image_inverted_path: String = format!("{}/inverted_image_{}", output_path, file_name);

    let grayscale_filter = GrayscaleFilter;
    let color_inversion_filter = ColorInversionFilter;

    let gray_scale_image = grayscale_filter.apply(image.clone());
    gray_scale_image.save(image_grayscale_path).unwrap();

    let inverted_image = color_inversion_filter.apply(image);
    inverted_image.save(image_inverted_path).unwrap();
}

#[no_mangle]
pub fn main_rust() {
    main();
}
