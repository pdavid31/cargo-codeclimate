mod lib;

use anyhow::{Context, Result};
use cargo_metadata::Message;
use serde_json;

use lib::CodeClimate;

// TODO: add CLI / clap
fn main() -> Result<()> {
    let output = std::process::Command::new("cargo")
        .arg("check")
        .arg("--message-format")
        .arg("json")
        .output()
        .context("running cargo check failed")?;

    // create the BufReader on stdout
    let reader = std::io::BufReader::new(&output.stdout[..]);
    // iterate over all messages in the reader
    let hints: Vec<CodeClimate> = Message::parse_stream(reader)
        // filter out all Errors and messages that are not of type CompilerMessage
        .filter_map(|message_res| {
            if let Ok(Message::CompilerMessage(msg)) = message_res {
                Some(msg)
            } else {
                None
            }
        })
        // map all Messages to CodeClimate hints
        .map(|msg| {
            let message = &msg.message;
            message
                .spans
                .iter()
                .map(|span| {
                    CodeClimate::new(
                        message.code.clone(),
                        &message.message,
                        &span.file_name,
                        span.line_start,
                        message.level,
                    )
                })
                .collect::<Vec<CodeClimate>>()
        })
        .flatten()
        .collect();

    // create a Write on stdout
    let writer = std::io::stdout();
    // serialize the CodeClimate hints to the writer
    serde_json::to_writer(writer, &hints).context("writing to stdout failed")?;

    Ok(())
}
