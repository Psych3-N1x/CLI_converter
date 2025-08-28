use std::fs::File;
use ico::{IconDir, IconImage, IconDirEntry, ResourceType};
use image::ImageReader;
use image::GenericImageView;
use std::io::{self, Write};

pub fn convert_jpg_to_ico() -> Result<(), Box<dyn std::error::Error>> {
    // Demande le chemin du fichier source
    print!("Chemin du fichier JPG : ");
    io::stdout().flush()?;
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path)?;
    let input_path = input_path.trim();

    // Demande le chemin de sortie
    print!("Chemin du fichier ICO de sortie : ");
    io::stdout().flush()?;
    let mut output_path = String::new();
    io::stdin().read_line(&mut output_path)?;
    let output_path = output_path.trim();

    let img = ImageReader::open(input_path)?.decode()?;
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();

    let icon_image = IconImage::from_rgba_data(width, height, rgba.into_raw());

    let mut icon_dir = IconDir::new(ResourceType::Icon);
    icon_dir.add_entry(IconDirEntry::encode(&icon_image)?);IconDirEntry::encode(&icon_image)?;

    let file = File::create(output_path)?;
    icon_dir.write(file)?;

    println!("Conversion terminée !");
    Ok(())
}