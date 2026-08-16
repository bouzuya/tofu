mod app;

#[::tokio::main]
async fn main() {
    ::topcoat::start(app::router()).await.unwrap();
}

#[cfg(test)]
mod tests {
    #[::tokio::test]
    async fn test_fetch_blocks() -> ::anyhow::Result<()> {
        let response =
            ::reqwest::get("https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt").await?;
        let text = response.text().await?;

        #[derive(Debug)]
        struct Block {
            start_code: String,
            end_code: String,
            block_name: String,
        }
        let mut blocks = vec![];
        for line in text.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parsed = line.split_terminator("..").collect::<Vec<&str>>();
            ::anyhow::ensure!(parsed.len() == 2, "Invalid line format: {}", line);
            let start_code = parsed[0];
            let parsed = parsed[1].split_terminator("; ").collect::<Vec<&str>>();
            ::anyhow::ensure!(parsed.len() == 2, "Invalid line format: {}", line);
            let end_code = parsed[0];
            let block_name = parsed[1];
            blocks.push(Block {
                start_code: start_code.to_string(),
                end_code: end_code.to_string(),
                block_name: block_name.to_string(),
            });
        }
        println!("{:#?}", blocks);
        Ok(())
    }
}
