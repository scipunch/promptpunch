use promptpunch::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let prompt = PromptBuilder::default()
        .messages(vec![
            message::system!("Hello there"),
            message::user!("How r you?")
        ])
        .build();

    Ok(())
}
