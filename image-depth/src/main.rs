use kornia::{
    image::{Image, ImageSize},
    imgproc,
};
use image::{ImageReader, GrayImage, Luma};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    // Load an image using the `image` crate
    let img_path = "image/c.jpg"; // Change to your image file path
    let img = ImageReader::open(img_path)?.decode()?.into_luma8(); // Convert to grayscale

    let (width, height) = img.dimensions();
    let size: ImageSize = [width as usize, height as usize].into();

    // Convert image data to Kornia-compatible format (f32 grayscale image)
    let mut img_f32: Image<f32, 1> = Image::from_size_val(size, 0f32)?;
    let mut img_f32_filtered: Image<f32, 1> = Image::from_size_val(size, 0f32)?;

    for (i, pixel) in img.pixels().enumerate() {
        img_f32.storage.as_mut_slice()[i] = pixel[0] as f32 / 255.0;
    }

    // Apply Sobel filter
    let mut img_f32_filtered_sobel = Image::from_size_val(size, 0f32)?;
    imgproc::filter::sobel(&img_f32, &mut img_f32_filtered_sobel, 3)?;

    // Normalize the Sobel output
    imgproc::normalize::normalize_min_max(
        &img_f32_filtered_sobel,
        &mut img_f32_filtered,
        0.0,
        1.0,
    )?;

    // Convert back to `GrayImage`
    let mut output_img = GrayImage::new(width, height);
    for (i, pixel) in output_img.pixels_mut().enumerate() {
        *pixel = Luma([(img_f32_filtered.storage.as_slice()[i] * 255.0) as u8]);
    }

    // Save the processed image
    let output_path = Path::new("image/output-c.jpg");
    output_img.save(output_path)?;
    println!("Filtered image saved as {:?}", output_path);

    Ok(())
}
