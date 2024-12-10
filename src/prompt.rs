use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

use crate::{PromptMessage, PromptMessageRequest, Role};

pub struct InjectableData {
    placeholder: String,
    content: String,
}

impl InjectableData {
    pub fn new(placeholder: impl Display, content: impl Display) -> Self {
        Self {
            placeholder: placeholder.to_string(),
            content: content.to_string(),
        }
    }
}

pub fn read_markdown_prompt_from_file(
    path: impl AsRef<Path>,
    injectable_data: &[InjectableData],
) -> anyhow::Result<Vec<PromptMessageRequest>> {
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);
    read_markdown_prompt(reader.lines().map_while(Result::ok), injectable_data)
}

pub fn read_markdown_prompt(
    lines: impl IntoIterator<Item = impl AsRef<str>>,
    injectable_data: &[InjectableData],
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
            let mut line = line.to_string();
            for data in injectable_data {
                line = line.replace(&data.placeholder, &data.content);
            }
            content += &line;
        }
    }

    Ok(messages)
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::{read_markdown_prompt, InjectableData};

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
        let got = read_markdown_prompt(markdown.lines(), &[]).unwrap();
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
