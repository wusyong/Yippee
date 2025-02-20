use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

const VERSO_VERSION: &str = "0.0.1";

fn main() {
    if env::var_os("PRE_BUILT_VERSOVIEW").is_some() {
        download_and_extract_verso().unwrap();
    }
}

fn decompress_archive(archive: &Path) -> Result<(), std::io::Error> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // Not ideal, but there's no good way of getting the target directory
    let target_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    if Command::new("tar")
        .current_dir(target_dir)
        .arg("-xf")
        .arg(archive)
        .status()?
        .success()
    {
        Ok(())
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }
}

fn download_archive(base_url: &str) -> Result<PathBuf, std::io::Error> {
    let target = env::var("TARGET").unwrap();
    let archive_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("verso.tar.gz");
    if !archive_path.exists() {
        if !Command::new("curl")
            .arg("-L")
            .arg("-f")
            .arg("-s")
            .arg("-o")
            .arg(&archive_path)
            .arg(format!(
                "{base_url}/download/versoview-v{VERSO_VERSION}/verso-{target}.tar.gz"
            ))
            .status()?
            .success()
        {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
        }
    }

    Ok(archive_path)
}

fn download_and_extract_verso() -> Result<(), std::io::Error> {
    if let Ok(archive) = env::var("VERSO_ARCHIVE") {
        // If the archive variable is present, assume it's a URL base to download from.
        let archive = download_archive(&archive).unwrap_or(PathBuf::from(archive));
        // Panic directly since the archive is specified manually.
        decompress_archive(&archive).unwrap();
    } else {
        let archive = download_archive("https://github.com/Legend-Master/verso/releases")?;
        decompress_archive(&archive)?;
    };

    Ok(())
}
