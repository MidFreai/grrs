use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli{
    pattern:String,
    path:std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();

    //anyhow devolvendo com muito contexto // nem presisa
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }

    Ok(())
}
