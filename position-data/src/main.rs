use kornia::{
    image::{Image, ImageSize},
    imgproc,
};
use image::{ImageReader, GrayImage, Luma};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let img_path = "image/a.jpg"; // Change to your image file path
    let img = ImageReader::open(img_path)?.decode()?.into_luma8(); // Convert to grayscale

    let (width, height) = img.dimensions();
    let size: ImageSize = [width as usize, height as usize].into();

    // Invert image (white depth instead of black)
    let mut inverted_img = img.clone();
    for pixel in inverted_img.pixels_mut() {
        pixel[0] = 255 - pixel[0]; // Invert pixel intensity
    }

    // Convert image data to Kornia-compatible format (f32 grayscale image)
    let mut img_f32: Image<f32, 1> = Image::from_size_val(size, 0f32)?;

    for (i, pixel) in inverted_img.pixels().enumerate() {
        img_f32.storage.as_mut_slice()[i] = pixel[0] as f32 / 255.0;
    }

    // Apply Sobel filter
    let mut img_f32_filtered_sobel = Image::from_size_val(size, 0f32)?;
    imgproc::filter::sobel(&img_f32, &mut img_f32_filtered_sobel, 3)?;

    // Compute row-wise intensity variance
    let mut row_variances = vec![0.0; height as usize];
    for y in 0..height as usize {
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        for x in 0..width as usize {
            let index = y * width as usize + x;
            let val = img_f32_filtered_sobel.storage.as_slice()[index];

            sum += val;
            sum_sq += val * val;
        }
        let mean = sum / width as f32;
        let variance = (sum_sq / width as f32) - (mean * mean);
        row_variances[y] = variance;
    }

    // Detect lines using a variance threshold
    let threshold = 0.005; // Adjust as needed
    let mut line_positions: Vec<usize> = Vec::new();
    for (y, &var) in row_variances.iter().enumerate() {
        if var > threshold {
            line_positions.push(y);
        }
    }

    // Sort detected lines from bottom to top
    line_positions.sort_by(|a, b| b.cmp(a));

    // Get the third white row from the bottom
    if line_positions.len() >= 3 {
        let third_line = line_positions[2]; // 0-based index

        // Define a larger slice (±10 rows around the detected line)
        let slice_height = 1000; // Total slice size (10 above + 10 below)
        let start_y = third_line.saturating_sub(slice_height / 2);
        let end_y = (third_line + slice_height / 2).min(height as usize - 1);

        println!(
            "Extracting slice from row {} to {} (third white row at {})",
            start_y, end_y, third_line
        );

        // Create a new image with the slice
        let mut slice_img = GrayImage::new(width, (end_y - start_y) as u32);
        for y in start_y..end_y {
            for x in 0..width {
                let pixel = img.get_pixel(x, y as u32);
                slice_img.put_pixel(x, (y - start_y) as u32, *pixel);
            }
        }

        // Save the extracted slice
        let slice_path = format!("image/third_white_row_slice_{}_{}.jpg", start_y, end_y);
        slice_img.save(&slice_path)?;
        println!("Extracted slice saved as {}", slice_path);
    } else {
        println!("Not enough white rows detected.");
    }

    Ok(())
}
