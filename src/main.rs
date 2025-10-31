use clap::{Parser};
use anyhow::{Result};

mod args;
use args::cli::{Cli};

fn main() -> Result<()>{

    let args = Cli::parse();

    args.run()
}
