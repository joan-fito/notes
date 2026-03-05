use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    // The word we're looking for
    #[arg(short = 'w', long)]
    word: String,

    // The path to the file to read
    #[arg(short = 'p', long)]
    path: std::path::PathBuf
}
fn main() -> Result<()> {
    env_logger::init();
    info!("starting");
    let args = Cli::parse();

    println!("Word: {:?}\nPath: {:?}", args.word, args.path);
    warn!("warnign");
    // Read the file
    let text = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read the file `{}`", &args.path.display()))?;

    // Show its content
    for line in text.lines() {
        if line.contains(&args.word) {
            println!("{}", line);
        }
    }
    Ok(())
}
