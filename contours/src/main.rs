use opencv::{
    core::{self, Mat, Point, Scalar, Size, Vector, Rect},
    highgui, imgcodecs, imgproc, prelude::*,
    Result,
};

fn main() -> Result<()> {
    // Load the text image
    let img = imgcodecs::imread("image/a.jpg", imgcodecs::IMREAD_COLOR)?;

    // Convert to grayscale
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // Apply threshold to binarize the image
    let mut thresh = Mat::default();
    imgproc::threshold(&gray, &mut thresh, 0.0, 255.0, imgproc::THRESH_BINARY_INV | imgproc::THRESH_OTSU)?;

    // Find contours (text lines)
    let mut contours = Vector::<Vector<Point>>::new();
    imgproc::find_contours(
        &thresh,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::new(0, 0),
    )?;

    // Get bounding boxes and sort by y position
    let mut bounding_boxes: Vec<Rect> = contours
        .iter()
        .map(|contour| imgproc::bounding_rect(&contour).unwrap())
        .collect();

    bounding_boxes.sort_by_key(|r| r.y); // Sort rows by y-coordinate

    // Save each row as a separate image
    for (i, rect) in bounding_boxes.iter().enumerate() {
        let roi = Mat::roi(&img, *rect)?;
        let filename = format!("row_{}.png", i);
        imgcodecs::imwrite(&filename, &roi, &Vector::new())?;
        println!("Saved {}", filename);
    }

    Ok(())
}
