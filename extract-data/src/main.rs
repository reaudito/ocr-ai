use opencv::{
    core,
    imgcodecs,
    imgproc,
    core::{Point, Scalar, Size, Vector, Rect},
    prelude::*,
};

fn main() -> opencv::Result<()> {
    // Load the image
    let mut image = imgcodecs::imread("image/polling-data/line_2.png", imgcodecs::IMREAD_COLOR)?;

    // Convert to grayscale
    let mut gray = core::Mat::default();
    imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // Apply binary thresholding
    let mut thresh = core::Mat::default();
    imgproc::threshold(&gray, &mut thresh, 128.0, 255.0, imgproc::THRESH_BINARY_INV)?;

    // Find contours
    let mut contours = Vector::<Vector<Point>>::new();
    imgproc::find_contours(
        &thresh,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::new(0, 0),
    )?;

    // Iterate through contours and filter based on area or aspect ratio
    for contour in contours {
        let rect = imgproc::bounding_rect(&contour)?;
        let area = rect.width * rect.height;

        // Filter contours based on area (adjust thresholds as needed)
        if area > 100 && area < 10000 {
            // Draw the contour on the original image
            imgproc::rectangle(
                &mut image,
                rect,
                core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                8,
                0,
            )?;
        }
    }

    // Save or display the result
    imgcodecs::imwrite("output_image.jpg", &image, &core::Vector::new())?;

    Ok(())
}