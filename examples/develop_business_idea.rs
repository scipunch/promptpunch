use promptpunch::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let llm = ChatGpt::from_env();
    let prompt = PromptBuilder::default()
        .messages(vec![
            message::system!("Hello there"),
            message::user!("How r you?"),
            message::complete!(),
            message::user!("Repeate ur words"),
        ])
        .build()?;

    let completion = llm.complete_chat(prompt).await?;

    println!("{:#?}", completion);
    Ok(())
}
