use std::io::BufRead;
use std::{fs::File, path::PathBuf};

use crate::{PromptMessage, PromptMessageRequest, Role};

pub fn read_markdown_prompt(path: PathBuf) -> anyhow::Result<Vec<PromptMessageRequest>> {
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);
    _read_markdown_prompt(reader.lines().map_while(Result::ok))
}

fn _read_markdown_prompt(
    lines: impl IntoIterator<Item = impl AsRef<str>>,
) -> anyhow::Result<Vec<PromptMessageRequest>> {
    let mut messages = vec![];
    let mut role = None;
    let mut content = String::new();

    for (idx, line) in lines.into_iter().enumerate() {
        let line = line.as_ref();
        if line.starts_with("#") {
            let maybe_role = match line.replace("#", "").trim().to_lowercase().as_str() {
                "system" => Some(Role::System),
                "user" => Some(Role::User),
                "assistant" => Some(Role::Assistant),
                _ => None,
            };
            match role {
                Some(inner_role) => {
                    messages.push(match inner_role {
                        Role::Assistant => PromptMessageRequest::WaitCompletion,
                        any_other => PromptMessageRequest::Message {
                            body: PromptMessage {
                                role: any_other,
                                content: content.clone(),
                            },
                        },
                    });
                    content = String::new();
                }
                None => {
                    if maybe_role.is_none() {
                        panic!("Falied to parse role from header on line {}", idx);
                    }
                }
            }
            role = maybe_role;
        } else {
            content += line;
        }
    }

    Ok(messages)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::_read_markdown_prompt;

    #[test]
    fn parses_markdown() {
        let markdown = r#"
# System
System prompt

# User
Some user prompt

# Assistant
# User
Another user prompt
"#;
        let got = _read_markdown_prompt(markdown.lines()).unwrap();
        let expected = vec![
            message::system!("System prompt"),
            message::user!("Some user prompt"),
            message::complete!(),
            message::user!("Another user prompt"),
        ];
        for (left, right) in got.into_iter().zip(expected) {
            assert_eq!(left, right);
        }
    }
}
