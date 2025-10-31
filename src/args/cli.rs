use clap::{Parser,Subcommand};
use anyhow::{Context, Result, ensure, anyhow};


///grrs -- Simple program to verify patterns in files
//passando o parser pra pegar a treat parse()
#[derive(Parser,Debug)]
#[command(version,about,long_about=None)]
pub struct Cli{
    ///Pattern for search
    pattern:String,

    ///Path of file
    file:std::path::PathBuf,

    #[command(subcommand)]
    command:Option<Commando>,
}

impl Cli{
    pub fn get_pattern(&self) -> String{
        self.pattern.clone()
    }
    pub fn get_file(&self) -> std::path::PathBuf{
        self.file.clone()
    }
    pub fn get_command(&self) -> Result<Commando>{
        match &self.command {
            Some(comand) => Ok(comand.clone()),
            None => Err(anyhow!("Nenhum comando encontrado")),
        }
        
    }

    pub fn run(&self) -> Result<(),anyhow::Error>{
        match &self.get_command() {
            Ok(Commando::Table) => {
                println!("Teste bem aqui");
            }
            Err(_) => {
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
            }
        }

        Ok(())
    }
}

#[derive(Debug,Clone,Subcommand)]
pub enum Commando {
    Table,
}
