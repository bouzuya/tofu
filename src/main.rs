mod app;

#[::tokio::main]
async fn main() {
    let blocks = fetch_blocks().await.unwrap();
    let names_list = fetch_names_list().await.unwrap();
    ::topcoat::start(self::app::router(blocks, names_list))
        .await
        .unwrap();
}

#[derive(Debug)]
struct Block {
    code_range: std::ops::RangeInclusive<u32>,
    block_name: String,
}

async fn fetch_blocks() -> Result<Vec<Block>, Box<dyn std::error::Error + Send + Sync>> {
    let response =
        ::reqwest::get("https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt").await?;
    let text = response.text().await?;

    let mut blocks = vec![];
    for line in text.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parsed = line.split_terminator("..").collect::<Vec<&str>>();
        if parsed.len() != 2 {
            return Err(format!("Invalid line format: {}", line).into());
        }
        let start_code = u32::from_str_radix(parsed[0], 16)
            .map_err(|_| format!("invalid start_code: {}", parsed[0]))?;
        let parsed = parsed[1].split_terminator("; ").collect::<Vec<&str>>();
        if parsed.len() != 2 {
            return Err(format!("Invalid line format: {}", line).into());
        }
        let end_code = u32::from_str_radix(parsed[0], 16)
            .map_err(|_| format!("invalid end_code: {}", parsed[0]))?;
        let block_name = parsed[1];
        blocks.push(Block {
            code_range: start_code..=end_code,
            block_name: block_name.to_string(),
        });
    }
    Ok(blocks)
}

#[derive(Debug)]
struct CharEntry {
    code_point: u32,
    name: String,
}

async fn fetch_names_list()
-> Result<std::collections::HashMap<u32, CharEntry>, Box<dyn std::error::Error + Send + Sync>> {
    let response =
        ::reqwest::get("https://www.unicode.org/Public/UCD/latest/ucd/NamesList.txt").await?;
    let text = response.text().await?;

    let mut names = std::collections::HashMap::<u32, CharEntry>::new();
    let mut iter = text.lines().peekable();
    while let Some(line) = iter.next() {
        if line.starts_with(';') {
            continue;
        }
        if line.starts_with('@') {
            while let Some(next_line) = iter.peek() {
                if next_line.starts_with('\t') {
                    iter.next();
                    continue;
                }
                break;
            }
            continue;
        }

        let parts = line.split('\t').collect::<Vec<&str>>();
        let char = parts[0];
        let name = parts[1];
        if name.starts_with('<') {
            while let Some(next_line) = iter.peek() {
                if next_line.starts_with('\t') {
                    iter.next();
                    continue;
                }
                break;
            }
            continue;
        }

        while let Some(next_line) = iter.peek() {
            if next_line.starts_with('\t') {
                iter.next();
                continue;
            }
            break;
        }

        let code_point = u32::from_str_radix(char, 16).unwrap();
        names.insert(
            code_point,
            CharEntry {
                code_point,
                name: name.to_string(),
            },
        );
    }

    Ok(names)
}

#[cfg(test)]
mod tests {
    #[::tokio::test]
    async fn test_fetch_blocks() -> ::anyhow::Result<()> {
        let blocks = super::fetch_blocks()
            .await
            .map_err(|e| ::anyhow::anyhow!(e))?;
        let block = blocks
            .iter()
            .find(|it| it.code_range.contains(&u32::from('あ')));
        println!("{:#?}", blocks);
        println!("{:#?}", block);
        Ok(())
    }

    #[::tokio::test]
    async fn test_parse_names_list() -> ::anyhow::Result<()> {
        let names = super::fetch_names_list()
            .await
            .map_err(|e| ::anyhow::anyhow!(e))?;
        let name = names.get(&u32::from('あ')).unwrap();
        println!("{:#?}", names);
        println!("{:#?}", name);
        Ok(())
    }
}
