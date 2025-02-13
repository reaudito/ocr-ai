use std::path::PathBuf;
use tesseract_rs::TesseractAPI;
use std::sync::Arc;
use std::thread;
use std::error::Error;

/// Determines the default tessdata directory based on the OS
fn get_default_tessdata_dir() -> PathBuf {
    if cfg!(target_os = "macos") {
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home_dir)
            .join("Library/Application Support/tesseract-rs/tessdata")
    } else if cfg!(target_os = "linux") {
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home_dir).join(".tesseract-rs/tessdata")
    } else if cfg!(target_os = "windows") {
        PathBuf::from(std::env::var("APPDATA").expect("APPDATA environment variable not set"))
            .join("tesseract-rs/tessdata")
    } else {
        panic!("Unsupported operating system");
    }
}

/// Gets the tessdata directory, either from TESSDATA_PREFIX or default
fn get_tessdata_dir() -> PathBuf {
    match std::env::var("TESSDATA_PREFIX") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => get_default_tessdata_dir(),
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let tessdata_dir = get_tessdata_dir();
    let api = TesseractAPI::new();

    // Initialize the main API
    api.init(tessdata_dir.to_str().unwrap(), "eng")?;
    api.set_variable("tessedit_pageseg_mode", "1")?;

    // Load and prepare image data
    let (image_data, width, height) = load_test_image("image/polling-data/line_19.png")?;

    // Share image data across threads
    let image_data = Arc::new(image_data);
    let mut handles = vec![];

    // Spawn multiple threads for parallel OCR processing
    for _ in 0..3 {
        let api_clone = api.clone(); // Clones the API with all configurations
        let image_data = Arc::clone(&image_data);

        let handle = thread::spawn(move || {
            // Set image in each thread
            let res = api_clone.set_image(
                &image_data,
                width as i32,
                height as i32,
                3,
                3 * width as i32,
            );
            assert!(res.is_ok());

            // Perform OCR in parallel
            let text = api_clone.get_utf8_text()
                .expect("Failed to get text");
            println!("Thread result: {}", text);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}


// Helper function to load test image
fn load_test_image(filename: &str) -> Result<(Vec<u8>, u32, u32), Box<dyn Error>> {
    let img = image::open(filename)?
        .to_rgb8();
    let (width, height) = img.dimensions();
    Ok((img.into_raw(), width, height))
}