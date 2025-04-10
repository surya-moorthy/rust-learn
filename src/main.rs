use clap::{Parser,Subcommand,Args};


#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command : Commands
}

#[derive(Subcommand)]

enum Commands {
    
    Add(AddArgs) ,
    Remove,
    Display,
    Update

}



#[derive(Debug,Args)]
struct AddArgs {
    name : String,
    description : String,
    status : Status
}


#[derive(Debug,Clone,clap::ValueEnum)]
enum Status {
    COMPLETED,
    PENDING,
    PROGRESS,
    NOTSTARTED
}




fn main()  {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add(todo) => {
             println!("Add a Todo {:?}",todo);
        }
        Commands::Remove => {
             println!("Remove a Todo");
        }
        Commands::Display => {
             println!("Display all the Todo");
        }
        Commands::Update => {
             println!("Update a Todo");
        }
    }
}

