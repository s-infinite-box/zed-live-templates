use anyhow::{Result, ensure};

#[derive(Debug)]
pub enum Node {
    Text(String),
    Variable(String),
    Cursor,
}

pub fn parse(body: &str) -> Result<Vec<Node>> {
    let mut nodes = Vec::new();
    let mut text = String::new();
    let mut offset = 0;
    let mut has_cursor = false;
    while offset < body.len() {
        let rest = &body[offset..];
        if rest.starts_with("\\$") {
            text.push('$');
            offset += 2;
            continue;
        }
        if let Some(after_dollar) = rest.strip_prefix('$')
            && let Some(end) = after_dollar.find('$')
        {
            let name = &after_dollar[..end];
            if is_name(name) {
                if !text.is_empty() {
                    nodes.push(Node::Text(std::mem::take(&mut text)));
                }
                if name == "END" {
                    ensure!(!has_cursor, "a template can contain only one $END$");
                    has_cursor = true;
                    nodes.push(Node::Cursor);
                } else {
                    nodes.push(Node::Variable(name.into()));
                }
                offset += end + 2;
                continue;
            }
        }
        let character = rest.chars().next().unwrap();
        text.push(character);
        offset += character.len_utf8();
    }
    if !text.is_empty() {
        nodes.push(Node::Text(text));
    }
    if !has_cursor {
        nodes.push(Node::Cursor);
    }
    Ok(nodes)
}

fn is_name(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && characters.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
