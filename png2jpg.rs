use image::ImageReader;
use std::io::Write;
use std::io;

pub fn convert_png_to_jpg() -> Result<(), Box<dyn std::error::Error>> {
    print!("Chemin du fichier PNG : ");
    io::stdout().flush()?;
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path)?;
    let input_path = input_path.trim();


    print!("Chemin du fichier JPG de sortie : ");
    io::stdout().flush()?;
    let mut output_path = String::new();
    io::stdin().read_line(&mut output_path)?;
    let output_path = output_path.trim();

    let img = ImageReader::open(input_path)?.decode()?;
    let rgb_img = img.to_rgb8();
    rgb_img.save_with_format(output_path, image::ImageFormat::Jpeg)?;

    
    Ok(())
}