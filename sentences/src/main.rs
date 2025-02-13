use opencv::{
    core::{self, Mat, Point, Rect, Scalar, Size, Vector},
    highgui, imgcodecs, imgproc, prelude::*,
    Result,
};

fn main() -> Result<()> {

    // Load the text image
    let img = imgcodecs::imread("image/c.jpg", imgcodecs::IMREAD_COLOR)?;

    // Convert to grayscale
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // Apply threshold to binarize the image
    let mut thresh = Mat::default();
    imgproc::threshold(
        &gray,
        &mut thresh,
        0.0,
        255.0,
        imgproc::THRESH_BINARY_INV | imgproc::THRESH_OTSU,
    )?;

    // Find contours (text elements)
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

    if bounding_boxes.is_empty() {
        return Ok(());
    }

    bounding_boxes.sort_by_key(|r| r.y); // Sort by vertical position

    // Merge boxes into lines based on vertical proximity
    let avg_height = bounding_boxes.iter().map(|r| r.height).sum::<i32>() as f32 / bounding_boxes.len() as f32;
    let threshold = (avg_height * 0.3) as i32; // Adjust this threshold as needed

    let mut lines = Vec::new();
    let mut current_line = bounding_boxes[0];

    for rect in bounding_boxes.iter().skip(1) {
        let current_line_bottom = current_line.y + current_line.height;
        let rect_bottom = rect.y + rect.height;
        let y_distance = rect.y - current_line_bottom;

        if y_distance <= threshold {
            // Merge with current line
            let new_x = current_line.x.min(rect.x);
            let new_y = current_line.y.min(rect.y);
            let new_right = (current_line.x + current_line.width).max(rect.x + rect.width);
            let new_bottom = current_line_bottom.max(rect_bottom);
            current_line = Rect::new(
                new_x,
                new_y,
                new_right - new_x,
                new_bottom - new_y,
            );
        } else {
            lines.push(current_line);
            current_line = *rect;
        }
    }
    lines.push(current_line);

    // Save each merged line as an image
    for (i, line_rect) in lines.iter().enumerate() {
        let roi = Mat::roi(&img, *line_rect)?;
        let filename = format!("lines/lines_c/line_{}.png", i);
        imgcodecs::imwrite(&filename, &roi, &Vector::new())?;
        println!("Saved {}", filename);
    }

    Ok(())
}