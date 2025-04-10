use clap::Parser;
pub mod todo_cli;

use todo_cli::structs::Commands;

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command : Commands
}


fn main()  {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(todo) => {
             println!("Add a Todo {:?}",todo);
        }
        Commands::Remove(title) => {
             println!("Remove a Todo {:?}",title);
        }
        Commands::Display => {
             println!("Display all the Todo");
        }
        Commands::Update(todo) => {
            if let Some(title) = &todo.title {
                println!("We have a updated title : {:?}",title)
            }
            if let Some(description) = &todo.description {
                println!("We have a updated description : {:?}",description)
            }
            if let Some(status) = &todo.status {
                println!("We have a updated status : {:?}",status)
            }
        }
    }
}

