use clap::{Parser};
use anyhow::{Context, Result, ensure};

mod args;
use args::cli::{Cli,Commando};

fn main() -> Result<()>{
    //Cli montado pelo derive parse
    let args = Cli::parse();

    args.run_command();

    //anyhow devolvendo com muito contexto // nem presisa
    let content = std::fs::read_to_string(&args.get_file())
        .with_context(|| format!("could not read file `{}`", args.get_file().display()))?;

    let mut ifprint = false;
    for line in content.lines(){
        if line.contains(&args.get_pattern()){
            println!("  {}",line);
            ifprint = true;
        }
    }

    ensure!(ifprint, "No lines find");

    Ok(())
}
