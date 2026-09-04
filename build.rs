#[::tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fetch_and_save_blocks().await?;
    fetch_and_save_names_list().await?;
    Ok(())
}

async fn fetch_and_save_blocks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/ucd/Blocks.txt");
    if std::fs::exists(path)? {
        return Ok(());
    }

    let response =
        ::reqwest::get("https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt").await?;
    let text = response.text().await?;
    std::fs::write(path, text)?;
    Ok(())
}

async fn fetch_and_save_names_list() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/ucd/NamesList.txt");
    if std::fs::exists(path)? {
        return Ok(());
    }

    let response =
        ::reqwest::get("https://www.unicode.org/Public/UCD/latest/ucd/NamesList.txt").await?;
    let text = response.text().await?;
    std::fs::write(path, text)?;
    Ok(())
}
