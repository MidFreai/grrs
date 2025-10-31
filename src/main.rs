use clap::{Parser};
use anyhow::{Result};

mod args;
use args::cli::{Cli};

fn main() -> Result<()>{
    //Cli montado pelo derive parse
    let args = Cli::parse();

    args.run()
}
