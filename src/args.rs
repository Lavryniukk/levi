use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct LeviArgs {
    #[clap(subcommand)]
    pub operation_type: OperationType,
}


#[derive(Subcommand, Debug)]
pub enum OperationType {
    Init,
}
