use std::io::Write;
use image::ImageReader;

pub fn convert_jpg_to_png() -> Result<(), Box<dyn std::error::Error >> {
    print!("chemin du fichier jpg : ");
    std::io::stdout().flush()?;
    let mut input_path = String::new();
    std::io::stdin().read_line(&mut input_path)?;
    let input_path = input_path.trim();

    print!("Chemin du fichier PNG de sortie : ");
    std::io::stdout().flush()?;
    let mut output_path = String::new();
    std::io::stdin().read_line(&mut output_path)?;
    let output_path = output_path.trim();

    let img = ImageReader::open(input_path)?.decode()?;
    img.save(output_path)?;

    println!("Conversion terminée !");
    Ok(())
}