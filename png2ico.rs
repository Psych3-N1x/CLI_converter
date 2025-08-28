use std::fs::File;
use std::io;
use std::io::Write;
use ico::{IconDir, IconImage, IconDirEntry, ResourceType};

pub fn convert_png_to_ico() -> Result<(), Box<dyn std::error::Error>> {
    print!("Chemin du fichier PNG : ");
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

    let input_file = File::open(input_path)?;
    let img = IconImage::read_png(input_file)?;
    
    let mut icon_dir = IconDir::new(ResourceType::Icon);
    icon_dir.add_entry(IconDirEntry::encode(&img)?);

    let output_ico = File::create(output_path)?;
    icon_dir.write(output_ico)?;

    println!("path {:?}", input_path);
    println!("ouput file {:?}", output_path);
    println!("conversion réalisée de  {:?} vers {:?}", input_path, output_path);

    Ok(())

}