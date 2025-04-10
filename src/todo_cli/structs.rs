use clap::{Subcommand,Args};

#[derive(Subcommand)]

pub enum Commands {
    
    Add(AddArgs) ,
    Remove(RemoveArgs),
    Display,
    Update(UpdateArgs)

}

#[derive(Debug,Args)]
pub struct RemoveArgs {
    title : String
}


#[derive(Debug,Args)]
pub struct UpdateArgs{
    #[arg(short,long,help="provide the title of the todo to update")]
    existed_title : String,
    #[arg(short,long,help="provide the updated title of the todo ")]
    pub title : Option<String>,
    #[arg(short,long,help="provide the updated description of the todo ")]
    pub description : Option<String>,
    #[arg(short,long,help="provide the updated status of the todo ")]
    pub status : Option<Status>
}

#[derive(Debug,Args)]
pub struct AddArgs {
    #[arg(short,long,help="title of the todo")]
    title : String,
    #[arg(short,long,help="Detailed description of the todo")]
    description : String,
    #[arg(short,long,help="Status of the todo")]
    status : Status
}


#[derive(Debug,Clone,clap::ValueEnum)]
pub enum Status {
    COMPLETED,
    PENDING,
    PROGRESS,
    NOTSTARTED
}