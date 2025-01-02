#[tokio::main]
async fn main() {
    if cfg!(feature = "cli") {
        cli::run().await.unwrap();
    }
}

#[cfg(feature = "cli")]
mod cli {
    use std::path::PathBuf;

    use clap::{command, Parser, Subcommand, ValueEnum};
    use promptpunch::{
        llm::LlmProvider,
        prelude::ChatGpt,
        prompt::{read_markdown_prompt_from_file, InjectableData},
        PromptBuilder,
    };

    #[derive(Parser, Debug)]
    struct Args {
        #[command(subcommand)]
        cmd: Command,
    }

    #[derive(Subcommand, Debug)]
    enum Command {
        Complete {
            #[arg(short, long)]
            prompt: PathBuf,

            #[arg(short, long, value_parser = parse_key_value)]
            argument: Vec<(String, String)>,

            #[arg(short, long)]
            output: PromptOutput,
        },
    }

    #[derive(Clone, Debug, ValueEnum)]
    enum PromptOutput {
        Last,
    }

    fn parse_key_value(input: &str) -> Result<(String, String), String> {
        let parts: Vec<&str> = input.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid key=value pair: {}", input));
        }
        Ok((parts[0].to_string(), parts[1].to_string()))
    }

    pub async fn run() -> anyhow::Result<()> {
        let args = Args::parse();
        let llm = ChatGpt::from_env();
        match args.cmd {
            Command::Complete {
                prompt,
                argument,
                output,
            } => {
                let data = argument
                    .into_iter()
                    .map(|(placeholder, value)| InjectableData::new(placeholder, value))
                    .collect::<Vec<_>>();
                let requests = read_markdown_prompt_from_file(prompt, data.as_slice())?;
                let prompt = PromptBuilder::default()
                    .messages(requests)
                    .temperature(0.5)
                    .build()?;
                let completion = llm.complete_chat(prompt).await?;

                match output {
                    PromptOutput::Last => {
                        println!("{}", completion.last_assistant_response()?);
                    }
                }
            }
        }
        Ok(())
    }
}
