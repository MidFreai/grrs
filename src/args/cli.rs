use clap::{Parser,Subcommand};
use anyhow::{Context, Result, ensure, anyhow};
use tabled::{Tabled, 
    settings::{Color, object::{Columns, Rows}, Modify}
};

use super::functions;

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
                let mut file_lines:Vec<Line> = vec![];

                for line in content.lines(){
                    if line.contains(pattern){
                        file_lines.push(Line { lines: (line.to_string()), color: (false) });
                        ifprint = true;
                    }
                }

                let mut table = functions::new_table(&file_lines);
                
                table.modify(Columns::first(), Color::FG_GREEN);

                println!("{}",table);
            }

            Ok(Commando::TableAll) => {
                let mut file_lines:Vec<Line> = vec![];

                for line in content.lines(){
                    if line.contains(pattern){
                        file_lines.push(Line { lines: (line.to_string()), color: (true) });
                        ifprint = true;
                    }
                    else {
                        file_lines.push(Line { lines: (line.to_string()), color: (false) });
                    }
                }

                let mut table = functions::new_table(&file_lines);

                for i in 0..file_lines.len() {
                    if file_lines[i].color == true{
                        table.with(Modify::new(Rows::one(i)).with(Color::FG_GREEN));
                    }
                }


                println!("{}",table);
                ensure!(ifprint, "No lines find with the pattern");
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

    #[command(visible_alias = "ta", alias="taball", alias="all")]
    ///Print all lines in table format
    TableAll,
    
    #[command(visible_alias = "r", alias="re")]
    ///Print all lines that do not match the pattern
    Reverse,
}

#[derive(Debug,Tabled)]
pub struct Line{
    lines:String,
    color:bool,
}