use clap::{Parser,Subcommand};
use anyhow::{Context, Result, ensure, anyhow};
use tabled::{Table, Tabled};


///grrs -- Simple program to verify patterns in files
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
        let content = std::fs::read_to_string(&self.get_file())
            .with_context(|| format!("could not read file `{}`", self.get_file().display()))?;
        
        let pattern = &self.get_pattern();
        
        let mut ifprint = false;
        
        match &self.get_command() {
            Ok(Commando::Table) => {
                let mut file_lines:Vec<String> = vec![];
                for line in content.lines(){
                    if line.contains(pattern){
                        file_lines.push(line.to_string());
                        ifprint = true;
                    }
                }
                let table = Table::new(file_lines);
                println!("{}",table);
            }

            Ok(Commando::Reverse) => {
                for line in content.lines(){
                    if !line.contains(pattern){
                        println!("{}",line);
                        ifprint = true;
                    }
                }
            }

            Err(_) => {
                for line in content.lines(){
                    if line.contains(pattern){
                        println!("{}",line);
                        ifprint = true;
                    }
                }
            }
        }
        
        ensure!(ifprint, "No lines find");
        Ok(())
    }
}

#[derive(Debug,Clone,Subcommand)]
pub enum Commando {
    #[command(visible_alias = "t", alias="tab")]
    ///Print in table format
    Table,
    
    #[command(visible_alias = "r", alias="re")]
    ///Print all lines that do not match the pattern
    Reverse,
}

#[derive(Debug,Tabled)]
struct Line{
    line:String,
}