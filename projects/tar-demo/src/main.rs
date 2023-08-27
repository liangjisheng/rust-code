use flate2::write::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;

fn archive() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("bundle/logs", ".")?;
    Ok(())
}

fn un_archive() -> Result<(), std::io::Error> {
    let path = "archive.tar.gz";
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

fn main() {
    // _ = archive();
    // _ = un_archive();
}
