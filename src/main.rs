use clap::{Parser};
use anyhow::{Context, Result, ensure};


///grrs -- Simple program to verify patterns in files
//passando o parser pra pegar a treat parse()
#[derive(Parser,Debug)]
#[command(version,about,long_about=None)]
struct Cli{
    ///Pattern for search
    //#[arg(short,long)]
    pattern:String,

    ///Path of file
    //#[arg(short,long)]
    file:std::path::PathBuf,
}

fn main() -> Result<()>{
    //Cli montado pelo derive parse
    let args = Cli::parse();

    //anyhow devolvendo com muito contexto // nem presisa
    let content = std::fs::read_to_string(&args.file)
        .with_context(|| format!("could not read file `{}`", args.file.display()))?;

    let mut ifprint = false;
    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line);
            ifprint = true;
        }
    }

    ensure!(ifprint, "No lines find");

    Ok(())
}
