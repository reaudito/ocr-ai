use rusty_tesseract::Image;
use rusty_tesseract::Args;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = Image::from_path("image/polling-data/line_6.png")?;

    let  my_args = Args {
        lang: "eng".into(),
        config_variables: HashMap::from([(
                "tessedit_char_whitelist".into(),
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".into(),
            )]),
        dpi: Some(150),
        psm: Some(6),
        oem: Some(3)
    };
    
    // string output
    let output = rusty_tesseract::image_to_string(&img, &my_args).unwrap();
        println!("The String output is: {:?}", output);
    

    Ok(())
}
