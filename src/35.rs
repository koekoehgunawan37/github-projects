use std::fs::{create_dir_all, File};
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let root_path = "A collection of projects related to GitHub".to_string();
    let output_file = format!("{}.txt", &root_path);

    create_dir_all(&root_path)?;

    if !create_dir_all(format!("{}/{}", &root_path, "data"))? {
        eprintln!("Failed to create data directory.");
        return Err(io::Error::new(io::ErrorKind::Other, "failed to create data directory"));
    }

    let file = File::create(output_file)?;
    let mut writer = BufReader::new(file);

    for line in io::stdin().lock()? {
        writer.write_line(&line)?;
    }
    writeln!(writer)?;

    Ok(())
}
