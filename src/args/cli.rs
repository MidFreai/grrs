use clap::{Parser,Subcommand};
use anyhow::{Context, Result, ensure};


///grrs -- Simple program to verify patterns in files
//passando o parser pra pegar a treat parse()
#[derive(Parser,Debug)]
#[command(version,about,long_about=None)]
pub struct Cli{
    ///Pattern for search
    pattern:Option<String>,

    ///Path of file
    file:Option<std::path::PathBuf>,

    #[command(subcommand)]
    command:Option<Commando>,
}

impl Cli{
    pub fn get_pattern(&self) -> String{
        self.pattern
        .clone()
        .unwrap_or("".to_string())
    }
    pub fn get_file(&self) -> std::path::PathBuf{
        match self.file {
            Some(_) => {
                self.file
                    .clone()
                    .unwrap()
            }
            None => {
                std::path::PathBuf::new()
            }
        }
    }
    pub fn get_command(&self) -> Commando{
        match self.command {
            Some(_) => self.command
                        .clone()
                        .unwrap(),
            None => Commando::None,
        }
        
    }

    pub fn run_command(&self){
        match self.get_command() {
            Commando::Test => {
                println!("Teste bem aqui");
            }
            Commando::Multi{ref first} => {println!("{first}");}
            Commando::None => {}
        }
    }

    pub fn run(&self) -> Result<(),anyhow::Error>{
        //anyhow devolvendo com muito contexto // nem presisa
        let content = std::fs::read_to_string(&self.get_file())
            .with_context(|| format!("could not read file `{}`", self.get_file().display()))?;

        let mut ifprint = false;
        for line in content.lines(){
            if line.contains(&self.get_pattern()){
                println!("  {}",line);
                ifprint = true;
            }
        }

        ensure!(ifprint, "No lines find");
        Ok(())
    }
}

#[derive(Debug,Clone,Subcommand)]
pub enum Commando {
    Test,
    Multi{
        first:String
    },
    None,
}
