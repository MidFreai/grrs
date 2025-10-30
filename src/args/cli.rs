use clap::{Parser,Subcommand};


///grrs -- Simple program to verify patterns in files
//passando o parser pra pegar a treat parse()
#[derive(Parser,Debug)]
#[command(version,about,long_about=None)]
pub struct Cli{
    ///Pattern for search
    //#[arg(short,long)]
    pattern:Option<String>,

    ///Path of file
    //#[arg(short,long)]
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
}

#[derive(Debug,Clone,Subcommand)]
pub enum Commando {
    Test,
    Multi{
        first:String
    },
    None,
}
