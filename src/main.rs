use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process::Stdio;

use mendokusai::{operate, Mendokusai, MendokusaiError, MendokusaiOp};
use serde_yaml;

#[tokio::main]
async fn main() -> Result<(), MendokusaiError> {
    let args: Vec<String> = env::args().collect();
    let path = match args.as_slice() {
        [_, path] => path,
        _ => {
            eprintln!("Required operation yaml path");
            std::process::exit(1)
        }
    };
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let op: MendokusaiOp = serde_yaml::from_reader(reader)?;

    let mut app = Mendokusai::new("http://localhost:4444").await?;
    operate(&mut app, op).await?;

    let _ = tokio::process::Command::new("pkill")
        .arg("-2")
        .arg("chromedriver")
        .stdout(Stdio::piped())
        .spawn()?
        .wait()
        .await?;
    Ok(())
}
