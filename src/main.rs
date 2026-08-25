mod app;
mod code_point;
mod components;

pub use crate::code_point::CodePoint;

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
    code_range: std::ops::RangeInclusive<CodePoint>,
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
        let start_code = CodePoint::from_str_without_u_plus(parsed[0])
            .ok_or_else(|| format!("invalid start_code: {}", parsed[0]))?;
        let parsed = parsed[1].split_terminator("; ").collect::<Vec<&str>>();
        if parsed.len() != 2 {
            return Err(format!("Invalid line format: {}", line).into());
        }
        let end_code = CodePoint::from_str_without_u_plus(parsed[0])
            .ok_or_else(|| format!("invalid end_code: {}", parsed[0]))?;
        let block_name = parsed[1];
        blocks.push(Block {
            code_range: start_code..=end_code,
            block_name: block_name.to_string(),
        });
    }
    Ok(blocks)
}

#[derive(Clone, Debug, PartialEq)]
struct CharEntry {
    aliases: Vec<String>,
    code_point: u32,
    comments: Vec<String>,
    cross_refs: Vec<(String, u32)>,
    name: String,
}

async fn fetch_names_list()
-> Result<std::collections::HashMap<u32, CharEntry>, Box<dyn std::error::Error + Send + Sync>> {
    let response =
        ::reqwest::get("https://www.unicode.org/Public/UCD/latest/ucd/NamesList.txt").await?;
    let text = response.text().await?;
    Ok(parse_names_list(&text))
}

fn parse_names_list(text: &str) -> std::collections::HashMap<u32, CharEntry> {
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

        let mut aliases = vec![];
        let mut comments = vec![];
        let mut cross_refs = vec![];
        while let Some(next_line) = iter.peek() {
            if next_line.starts_with("\t=") {
                // ALIAS_LINE:	TAB "=" SP LINE
                match next_line.strip_prefix("\t= ") {
                    None => {
                        // do nothing
                    }
                    Some(alias) => {
                        aliases.push(alias.to_string());
                    }
                }
                iter.next();
                continue;
            }
            if next_line.starts_with("\t* ") {
                match next_line.strip_prefix("\t* ") {
                    None => {
                        // do nothing
                    }
                    Some(comment) => {
                        comments.push(comment.to_string());
                    }
                }
                iter.next();
                continue;
            }
            if next_line.starts_with("\tx (") && next_line.ends_with(")") {
                match next_line
                    .strip_prefix("\tx (")
                    .and_then(|next_line| next_line.strip_suffix(")"))
                {
                    None => {
                        // do nothing
                    }
                    Some(cross_ref) => {
                        if let Some((name, code)) = cross_ref.rsplit_once(" - ") {
                            if let Ok(code_point) = u32::from_str_radix(code, 16) {
                                cross_refs.push((name.to_string(), code_point));
                            }
                        }
                    }
                }
                iter.next();
                continue;
            }
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
                aliases,
                code_point,
                comments,
                cross_refs,
                name: name.to_string(),
            },
        );
    }

    names
}

#[cfg(test)]
mod tests {
    use crate::{CodePoint, parse_names_list};

    #[::tokio::test]
    async fn test_fetch_blocks() -> ::anyhow::Result<()> {
        let blocks = super::fetch_blocks()
            .await
            .map_err(|e| ::anyhow::anyhow!(e))?;
        let block = blocks
            .iter()
            .find(|it| it.code_range.contains(&CodePoint::from_char('あ')));
        println!("{:#?}", blocks);
        println!("{:#?}", block);
        Ok(())
    }

    #[::tokio::test]
    async fn test_fetch_names_list() -> ::anyhow::Result<()> {
        let names = super::fetch_names_list()
            .await
            .map_err(|e| ::anyhow::anyhow!(e))?;
        let name = names.get(&u32::from('あ')).unwrap();
        println!("{:#?}", names);
        println!("{:#?}", name);
        let name = names.get(&u32::from('&')).unwrap();
        println!("{:#?}", name);
        Ok(())
    }

    #[test]
    fn test_parse_names_list() {
        let text = "0026\tAMPERSAND
\t= and
\t* originally derived from a ligature of 'e' and 't'
\tx (tironian sign et - 204A)
\tx (turned ampersand - 214B)
\tx (heavy ampersand ornament - 1F674)
";
        assert_eq!(
            parse_names_list(text),
            [(
                0x0026,
                super::CharEntry {
                    aliases: vec!["and".to_string(),],
                    code_point: 0x0026,
                    comments: vec!["originally derived from a ligature of 'e' and 't'".to_string()],
                    cross_refs: vec![
                        ("tironian sign et".to_string(), 0x204A),
                        ("turned ampersand".to_string(), 0x214B),
                        ("heavy ampersand ornament".to_string(), 0x1F674),
                    ],
                    name: "AMPERSAND".to_string(),
                }
            )]
            .iter()
            .cloned()
            .collect::<std::collections::HashMap<u32, super::CharEntry>>()
        );
    }
}
