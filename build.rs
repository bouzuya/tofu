#[::tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fetch_and_save(
        "Blocks.txt",
        "https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt",
    )
    .await?;
    fetch_and_save(
        "NamesList.txt",
        "https://www.unicode.org/Public/UCD/latest/ucd/NamesList.txt",
    )
    .await?;
    fetch_and_save(
        "Unihan.zip",
        "https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip",
    )
    .await?;
    unzip_unihan()?;
    Ok(())
}

async fn fetch_and_save(
    file: &str,
    url: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("ucd")
        .join(file);
    if path.exists() {
        return Ok(());
    }

    let response = ::reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    std::fs::write(path, bytes)?;
    Ok(())
}

fn unzip_unihan() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let zip_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("ucd")
        .join("Unihan.zip");
    if !zip_path.exists() {
        return Ok(());
    }

    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("ucd")
        .join("Unihan_Readings.txt");
    if path.exists() {
        return Ok(());
    }

    let bytes = std::fs::read(zip_path)?;
    let mut archive = ::zip::ZipArchive::new(::std::io::Cursor::new(bytes))?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.name() == "Unihan_Readings.txt" {
            let mut contents = String::new();
            use ::std::io::Read;
            file.read_to_string(&mut contents)?;
            std::fs::write(&path, contents)?;
        }
    }
    Ok(())
}
