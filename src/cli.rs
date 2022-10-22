use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct DebugArgs {
    #[clap(subcommand)]
    /// First Arg
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    //    Mode(UserMode),
    /// Queue Node
    QueueJob(JobCommand),
}
/*
#[derive(Debug, Subcommand)]
pub enum UserMode {
    Run(),
    Teaching(),
    Stop(),
}
*/

#[derive(Debug, Args)]
pub struct JobCommand {
    /// Id of Job
    pub id: i32,
    /// Alternative: Name of Job
    pub alias: String,
}
